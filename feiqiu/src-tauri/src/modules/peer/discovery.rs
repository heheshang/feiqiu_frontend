// Peer discovery module - broadcast and LAN peer detection
//
// This module handles peer discovery through UDP broadcasts:
// - Announcing online status to LAN
// - Listening for peer announcements
// - Processing incoming discovery messages

use crate::network::msg_type;
use crate::network::serialize_message;
use crate::network::ProtocolMessage;
use crate::network::UdpTransport;
use crate::Result;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Default receive buffer size for UDP
const RECV_BUFFER_SIZE: usize = 65535;

/// Time window for rate limiting
const RATE_LIMIT_WINDOW: Duration = Duration::from_secs(1);

/// Rate limiter for UDP message reception
///
/// Tracks the last seen time for each IP address to prevent DoS attacks.
#[derive(Clone)]
struct RateLimiter {
    /// Maps IP address to last message timestamp
    last_seen: Arc<Mutex<HashMap<std::net::IpAddr, Instant>>>,
}

impl RateLimiter {
    /// Create a new rate limiter
    fn new() -> Self {
        Self {
            last_seen: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Check if a message from the given IP should be allowed
    ///
    /// Returns `true` if the message is allowed, `false` if rate limited.
    fn check_rate(&self, ip: std::net::IpAddr) -> bool {
        let mut map = self.last_seen.lock().unwrap();
        let now = Instant::now();

        // Check if this IP has sent a message recently
        if let Some(last_time) = map.get(&ip) {
            if now.duration_since(*last_time) < RATE_LIMIT_WINDOW {
                // Rate limited - already received message within window
                return false;
            }
        }

        map.insert(ip, now);
        true
    }
}

/// Peer discovery service
///
/// Handles UDP broadcast-based peer discovery for LAN communication.
#[derive(Clone)]
pub struct PeerDiscovery {
    /// UDP transport for sending/receiving
    udp: Arc<UdpTransport>,

    /// Local user_id (user unique identifier)
    user_id: String,

    /// Local username (sent in announcements)
    username: String,

    /// Local hostname (sent in announcements)
    hostname: String,

    /// Packet ID counter (for unique message IDs)
    packet_id: Arc<AtomicU64>,

    /// Rate limiter for incoming messages
    rate_limiter: RateLimiter,
}

impl PeerDiscovery {
    /// Create a new peer discovery service
    ///
    /// # Arguments
    /// * `udp` - UDP transport instance
    /// * `user_id` - Local user unique identifier
    /// * `username` - Local username (displayed to other peers)
    /// * `hostname` - Local hostname
    ///
    /// # Returns
    /// * `PeerDiscovery` - New discovery service instance
    pub fn new(udp: UdpTransport, user_id: String, username: String, hostname: String) -> Self {
        tracing::info!("Creating PeerDiscovery: {}@{}", username, hostname);

        Self {
            udp: Arc::new(udp),
            user_id,
            username,
            hostname,
            packet_id: Arc::new(AtomicU64::new(1)),
            rate_limiter: RateLimiter::new(),
        }
    }

    /// Create from UdpTransport with default system username/hostname
    ///
    /// # Arguments
    /// * `udp` - UDP transport instance
    /// * `user_id` - User unique identifier (will be generated if not provided)
    ///
    /// # Returns
    /// * `PeerDiscovery` - New discovery service instance with system identity
    pub fn with_defaults(udp: UdpTransport, user_id: String) -> Self {
        let username = whoami::username();
        let hostname = whoami::fallible::hostname().unwrap_or_else(|_| "localhost".to_string());

        Self::new(udp, user_id, username, hostname)
    }

    /// Announce online status to LAN
    ///
    /// Sends a STATUS_ONLINE broadcast message to all peers on the LAN.
    /// Other peers running NeoLan will receive this and add this peer to their list.
    ///
    /// # Returns
    /// * `Ok(())` - Announcement sent successfully (or gracefully skipped on macOS)
    /// * `Err(NeoLanError)` - Send failed
    pub fn announce_online(&self) -> Result<()> {
        tracing::info!("Announcing online status to LAN (FeiQ compatible)");

        // Enable broadcast if not already enabled
        if let Err(e) = self.udp.set_broadcast_enabled(true) {
            tracing::warn!("Failed to enable broadcast: {:?}, continuing anyway", e);
            // On macOS, this can fail due to interface issues - continue anyway
        }

        // Get MAC address for FeiQ format
        let mac_address =
            crate::network::get_local_mac_address().unwrap_or_else(|_| "000000000000".to_string());

        // Create BR_ENTRY message (broadcast online)
        let msg = ProtocolMessage {
            version: 1,
            packet_id: self.next_packet_id(),
            user_id: self.user_id.clone(),
            sender_name: self.username.clone(),
            sender_host: self.hostname.clone(),
            msg_type: msg_type::IPMSG_BR_ENTRY | msg_type::IPMSG_UTF8OPT,
            content: self.username.clone(), // FeiQ puts username in content for BR_ENTRY
        };

        // Try FeiQ format first, fall back to standard format
        let feiq_bytes =
            crate::network::serialize_message_for_feiq(&msg, &mac_address, self.udp.port())?;

        match self.udp.broadcast(&feiq_bytes) {
            Ok(()) => {
                tracing::debug!(
                    "FeiQ online announcement sent: {}@{}",
                    self.username,
                    self.hostname
                );
            }
            Err(e) => {
                tracing::warn!("FeiQ broadcast failed: {:?}, trying standard format", e);
                // Fallback to standard IPMsg format
                let bytes = serialize_message(&msg)?;
                if let Err(e2) = self.udp.broadcast(&bytes) {
                    tracing::warn!("Standard broadcast failed: {:?} (normal on macOS)", e2);
                }
            }
        }

        Ok(())
    }

    /// Listen for incoming peer messages (blocking)
    ///
    /// This method blocks and continuously listens for incoming UDP messages.
    /// For each received message, the provided callback is invoked with the
    /// parsed protocol message and sender address.
    ///
    /// # Arguments
    /// * `callback` - Function to call for each received message
    ///
    /// # Callback Signature
    /// `Fn(ProtocolMessage, SocketAddr)`
    /// - First argument: Parsed protocol message
    /// - Second argument: Sender's socket address
    pub fn listen_incoming<F>(&self, mut callback: F) -> Result<()>
    where
        F: FnMut(ProtocolMessage, SocketAddr),
    {
        tracing::info!("Starting peer discovery listener");

        let mut buffer = [0u8; RECV_BUFFER_SIZE];

        loop {
            // Receive UDP packet
            let (len, sender) = match self.udp.recv_from(&mut buffer) {
                Ok(result) => result,
                Err(e) => {
                    tracing::warn!("Receive error: {:?}", e);
                    // Brief sleep before retry
                    std::thread::sleep(std::time::Duration::from_millis(100));
                    continue;
                }
            };

            let data = &buffer[..len];

            // Rate limiting check - drop if this IP is sending too fast
            if !self.rate_limiter.check_rate(sender.ip()) {
                tracing::trace!("Rate limited message from {}", sender.ip());
                continue;
            }

            // Parse protocol message
            match crate::network::parse_message(data) {
                Ok(msg) => {
                    tracing::trace!(
                        "Received {} from {}: {}",
                        crate::network::get_message_type_name(msg.msg_type),
                        sender,
                        msg.sender_name
                    );

                    // Invoke callback
                    callback(msg, sender);
                }
                Err(e) => {
                    tracing::warn!("Failed to parse message from {}: {:?}", sender, e);
                }
            }
        }
    }

    /// Send a specific protocol message to a target address
    ///
    /// # Arguments
    /// * `msg` - Protocol message to send
    /// * `addr` - Target socket address
    ///
    /// # Returns
    /// * `Ok(())` - Message sent successfully
    /// * `Err(NeoLanError)` - Send failed
    pub fn send_message(&self, msg: &ProtocolMessage, addr: SocketAddr) -> Result<()> {
        let bytes = serialize_message(msg)?;
        self.udp.send_to(&bytes, addr)?;
        Ok(())
    }

    /// Get the UDP transport port
    ///
    /// # Returns
    /// * `u16` - The port this discovery service is bound to
    pub fn port(&self) -> u16 {
        self.udp.port()
    }

    /// Get local socket address
    ///
    /// # Returns
    /// * `Result<SocketAddr>` - Local socket address
    pub fn local_addr(&self) -> Result<SocketAddr> {
        self.udp.local_addr()
    }

    /// Get local user_id
    ///
    /// # Returns
    /// * `&str` - Local user_id
    pub fn user_id(&self) -> &str {
        &self.user_id
    }

    /// Get local username
    ///
    /// # Returns
    /// * `&str` - Local username
    pub fn username(&self) -> &str {
        &self.username
    }

    /// Get local hostname
    ///
    /// # Returns
    /// * `&str` - Local hostname
    pub fn hostname(&self) -> &str {
        &self.hostname
    }

    /// Generate next packet ID
    ///
    /// # Returns
    /// * `u64` - Next unique packet ID
    pub fn next_packet_id(&self) -> u64 {
        self.packet_id.fetch_add(1, Ordering::SeqCst)
    }

    /// Create a protocol message with local identity
    ///
    /// # Arguments
    /// * `msg_type` - Message type constant
    /// * `content` - Message content
    ///
    /// # Returns
    /// * `ProtocolMessage` - Message with local sender info
    pub fn create_message(&self, msg_type: u32, content: String) -> ProtocolMessage {
        ProtocolMessage {
            version: 1,
            packet_id: self.next_packet_id(),
            user_id: self.user_id.clone(),
            sender_name: self.username.clone(),
            sender_host: self.hostname.clone(),
            msg_type,
            content,
        }
    }

    /// Clone the UDP transport (for sharing with other components)
    ///
    /// # Returns
    /// * `Arc<UdpTransport>` - Cloned transport reference
    pub fn clone_transport(&self) -> Arc<UdpTransport> {
        Arc::clone(&self.udp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_peer_discovery_creation() {
        let udp = UdpTransport::bind(0).unwrap();
        let discovery = PeerDiscovery::new(
            udp,
            "T0170006".to_string(),
            "TestUser".to_string(),
            "test-host".to_string(),
        );

        assert_eq!(discovery.username(), "TestUser");
        assert_eq!(discovery.hostname(), "test-host");
    }

    #[test]
    fn test_peer_discovery_with_defaults() {
        let udp = UdpTransport::bind(0).unwrap();
        let discovery = PeerDiscovery::with_defaults(udp, "T0170006".to_string());

        // Username should be non-empty
        assert!(!discovery.username().is_empty());
        // Hostname should be non-empty
        assert!(!discovery.hostname().is_empty());
    }

    #[test]
    fn test_announce_online() {
        let udp = UdpTransport::bind(0).unwrap();
        let discovery = PeerDiscovery::new(
            udp,
            "T0170006".to_string(),
            "TestUser".to_string(),
            "test-host".to_string(),
        );

        // Should not fail
        let result = discovery.announce_online();
        assert!(result.is_ok());
    }

    #[test]
    fn test_create_message() {
        let udp = UdpTransport::bind(0).unwrap();
        let discovery = PeerDiscovery::new(
            udp,
            "T0170006".to_string(),
            "Alice".to_string(),
            "alice-pc".to_string(),
        );

        let msg = discovery.create_message(msg_type::IPMSG_SENDMSG, "Hello".to_string());

        assert_eq!(msg.version, 1);
        assert_eq!(msg.sender_name, "Alice");
        assert_eq!(msg.sender_host, "alice-pc");
        assert_eq!(msg.msg_type, msg_type::IPMSG_SENDMSG);
        assert_eq!(msg.content, "Hello");
    }

    #[test]
    fn test_packet_id_increment() {
        let udp = UdpTransport::bind(0).unwrap();
        let discovery = PeerDiscovery::new(
            udp,
            "T0170006".to_string(),
            "Test".to_string(),
            "test".to_string(),
        );

        let id1 = discovery.next_packet_id();
        let id2 = discovery.next_packet_id();
        let id3 = discovery.next_packet_id();

        assert!(id2 > id1);
        assert!(id3 > id2);
    }

    #[test]
    fn test_message_types() {
        // Verify IPMsg message type constants are accessible
        assert_eq!(msg_type::IPMSG_BR_ENTRY, 0x00000001);
        assert_eq!(msg_type::IPMSG_BR_EXIT, 0x00000002);
        assert_eq!(msg_type::IPMSG_SENDMSG, 0x00000020);
    }
}

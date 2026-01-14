// Peer discovery module - broadcast and LAN peer detection
//
// This module handles peer discovery through UDP broadcasts:
// - Announcing online status to LAN
// - Listening for peer announcements
// - Processing incoming discovery messages

use crate::network::{msg_type, serialize_message, ProtocolMessage, UdpTransport};
use crate::Result;
use std::net::SocketAddr;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

/// Default receive buffer size for UDP
const RECV_BUFFER_SIZE: usize = 65535;

/// Peer discovery service
///
/// Handles UDP broadcast-based peer discovery for LAN communication.
#[derive(Clone)]
pub struct PeerDiscovery {
    /// UDP transport for sending/receiving
    udp: Arc<UdpTransport>,

    /// Local username (sent in announcements)
    username: String,

    /// Local hostname (sent in announcements)
    hostname: String,

    /// Packet ID counter (for unique message IDs)
    packet_id: Arc<AtomicU64>,
}

impl PeerDiscovery {
    /// Create a new peer discovery service
    ///
    /// # Arguments
    /// * `udp` - UDP transport instance
    /// * `username` - Local username (displayed to other peers)
    /// * `hostname` - Local hostname
    ///
    /// # Returns
    /// * `PeerDiscovery` - New discovery service instance
    pub fn new(udp: UdpTransport, username: String, hostname: String) -> Self {
        tracing::info!("Creating PeerDiscovery: {}@{}", username, hostname);

        Self {
            udp: Arc::new(udp),
            username,
            hostname,
            packet_id: Arc::new(AtomicU64::new(1)),
        }
    }

    /// Create from UdpTransport with default system username/hostname
    ///
    /// # Arguments
    /// * `udp` - UDP transport instance
    ///
    /// # Returns
    /// * `PeerDiscovery` - New discovery service instance with system identity
    pub fn with_defaults(udp: UdpTransport) -> Self {
        let username = whoami::username();
        let hostname = whoami::fallible::hostname().unwrap_or_else(|_| "localhost".to_string());

        Self::new(udp, username, hostname)
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
        tracing::info!("Announcing online status to LAN");

        // Enable broadcast if not already enabled
        if let Err(e) = self.udp.set_broadcast_enabled(true) {
            tracing::warn!("Failed to enable broadcast: {:?}, continuing anyway", e);
            // On macOS, this can fail due to interface issues - continue anyway
        }

        // Create BR_ENTRY message (broadcast online)
        let msg = ProtocolMessage {
            version: 1,
            packet_id: self.next_packet_id(),
            sender_name: self.username.clone(),
            sender_host: self.hostname.clone(),
            msg_type: msg_type::IPMSG_BR_ENTRY,
            content: String::new(),
        };

        // Serialize and send
        let bytes = serialize_message(&msg)?;

        // Try to broadcast, but handle macOS broadcast issues gracefully
        match self.udp.broadcast(&bytes) {
            Ok(()) => {
                tracing::debug!(
                    "Online announcement sent: {}@{}",
                    self.username,
                    self.hostname
                );
            }
            Err(e) => {
                // On macOS, broadcast can fail with EADDRNOTAVAIL (error 49) due to
                // virtual interfaces (VPN, Docker, etc.). We continue anyway since:
                // 1. We can still receive peer announcements
                // 2. Other peers will discover us when they broadcast
                tracing::warn!(
                    "Failed to send broadcast announcement (this is normal on macOS with VPNs/Docker): {:?}. \
                     Continuing - peer discovery will work when other peers announce.",
                    e
                );
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
    fn next_packet_id(&self) -> u64 {
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
        let discovery = PeerDiscovery::new(udp, "TestUser".to_string(), "test-host".to_string());

        assert_eq!(discovery.username(), "TestUser");
        assert_eq!(discovery.hostname(), "test-host");
    }

    #[test]
    fn test_peer_discovery_with_defaults() {
        let udp = UdpTransport::bind(0).unwrap();
        let discovery = PeerDiscovery::with_defaults(udp);

        // Username should be non-empty
        assert!(!discovery.username().is_empty());
        // Hostname should be non-empty
        assert!(!discovery.hostname().is_empty());
    }

    #[test]
    fn test_announce_online() {
        let udp = UdpTransport::bind(0).unwrap();
        let discovery = PeerDiscovery::new(udp, "TestUser".to_string(), "test-host".to_string());

        // Should not fail
        let result = discovery.announce_online();
        assert!(result.is_ok());
    }

    #[test]
    fn test_create_message() {
        let udp = UdpTransport::bind(0).unwrap();
        let discovery = PeerDiscovery::new(udp, "Alice".to_string(), "alice-pc".to_string());

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
        let discovery = PeerDiscovery::new(udp, "Test".to_string(), "test".to_string());

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

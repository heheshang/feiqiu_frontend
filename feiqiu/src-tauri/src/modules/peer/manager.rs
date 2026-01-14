// Peer manager - core peer management logic
//
// This module handles:
// - Maintaining the peer list (in-memory HashMap)
// - Processing discovery messages
// - Managing peer state transitions
// - Routing text messages to MessageHandler

use crate::modules::peer::{discovery::PeerDiscovery, types::*};
use crate::{network::ProtocolMessage, Result};
use std::collections::HashMap;
use std::io::{self, Error as IoError};
use std::net::{IpAddr, SocketAddr};
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex, PoisonError};
use tracing::{debug, error, info, warn};

/// Safe mutex lock helper - prevents panics on mutex poisoning
///
/// When a mutex is poisoned (previous holder panicked), we still allow access
/// to the data. This is safe because the poison flag is just a warning - the
/// mutex guard still provides exclusive access to the data.
macro_rules! safe_lock {
    ($mutex:expr) => {
        match $mutex.lock() {
            Ok(guard) => guard,
            Err(e) => {
                warn!("Mutex poisoned, recovering: {}", e);
                // PoisonError contains the guard, we can still use it
                e.into_inner()
            }
        }
    };
}

/// Convert lock poison error to io error (kept for compatibility)
fn lock_error<T>(_: PoisonError<T>) -> io::Error {
    IoError::other("Mutex lock poisoned")
}

/// Message routing request
#[derive(Clone, Debug)]
pub struct MessageRouteRequest {
    /// Protocol message
    pub message: ProtocolMessage,
    /// Sender address
    pub sender: SocketAddr,
}

/// Peer manager
///
/// Manages the in-memory peer list and handles peer discovery events.
#[derive(Clone)]
pub struct PeerManager {
    /// Peer discovery service
    discovery: PeerDiscovery,

    /// In-memory peer map (IP -> PeerNode)
    peers: Arc<Mutex<HashMap<IpAddr, PeerNode>>>,

    /// Whether the manager is running
    running: Arc<Mutex<bool>>,

    /// Channel sender for routing text messages to MessageHandler
    message_tx: Arc<Mutex<Option<Sender<MessageRouteRequest>>>>,
}

impl PeerManager {
    /// Create a new peer manager
    ///
    /// # Arguments
    /// * `discovery` - Peer discovery service
    ///
    /// # Returns
    /// * `PeerManager` - New peer manager instance
    pub fn new(discovery: PeerDiscovery) -> Self {
        info!("Creating PeerManager");

        Self {
            discovery,
            peers: Arc::new(Mutex::new(HashMap::new())),
            running: Arc::new(Mutex::new(false)),
            message_tx: Arc::new(Mutex::new(None)),
        }
    }

    /// Set the message handler channel
    ///
    /// This allows the PeerManager to route text messages to the MessageHandler.
    ///
    /// # Arguments
    /// * `tx` - Channel sender for message routing
    pub fn set_message_handler_channel(&self, tx: Sender<MessageRouteRequest>) {
        *safe_lock!(self.message_tx) = Some(tx);
        info!("Message handler channel set in PeerManager");
    }

    /// Start the peer manager
    ///
    /// This announces online presence and starts listening for peer messages.
    /// Note: This is a blocking call that runs the message listener loop.
    /// In production, run this in a separate thread.
    ///
    /// # Returns
    /// * `Ok(())` - Manager started successfully
    /// * `Err(NeoLanError)` - Start failed
    pub fn start(&self) -> Result<()> {
        // Check if already running
        {
            let running = self.running.lock().map_err(lock_error)?;
            if *running {
                warn!("PeerManager already running");
                return Ok(());
            }
            drop(running);
            let mut running = self.running.lock().map_err(lock_error)?;
            *running = true;
        }

        info!("Starting PeerManager");

        // Announce online presence
        self.discovery.announce_online()?;

        // Start listening for incoming messages (blocking)
        let peers = Arc::clone(&self.peers);
        let running = Arc::clone(&self.running);
        let message_tx = Arc::clone(&self.message_tx);

        self.discovery.listen_incoming(move |msg, sender| {
            // Check if still running
            {
                let is_running = running.lock().map(|r| *r).unwrap_or(false);
                if !is_running {
                    return;
                }
            }

            // Handle the message
            if let Err(e) = Self::handle_message(&peers, msg, sender, &message_tx) {
                warn!("Failed to handle message: {:?}", e);
            }
        })?;

        Ok(())
    }

    /// Stop the peer manager
    pub fn stop(&self) {
        if let Ok(mut running) = self.running.lock() {
            *running = false;
            info!("Stopping PeerManager");
        }
    }

    /// Handle a protocol message
    fn handle_message(
        peers: &Arc<Mutex<HashMap<IpAddr, PeerNode>>>,
        msg: ProtocolMessage,
        sender: SocketAddr,
        message_tx: &Arc<Mutex<Option<Sender<MessageRouteRequest>>>>,
    ) -> Result<()> {
        let ip = sender.ip();

        info!(
            "📨 [UDP RECEIVE] Received message from {}: type={}, sender={}, content={}",
            ip,
            msg.msg_type,
            msg.sender_name,
            msg.content.chars().take(50).collect::<String>()
        );

        // Extract base mode (low 8 bits) to handle messages with options
        let mode = crate::network::msg_type::get_mode(msg.msg_type);

        match mode as u32 {
            // IPMSG_BR_ENTRY: Peer is online / broadcasting presence
            crate::network::msg_type::IPMSG_BR_ENTRY => {
                debug!("📢 Handling BR_ENTRY (peer online)");
                Self::handle_online_msg(peers, msg, sender)?;
            }
            // IPMSG_BR_EXIT: Peer is going offline
            crate::network::msg_type::IPMSG_BR_EXIT => {
                debug!("📴 Handling BR_EXIT (peer offline)");
                Self::handle_offline_msg(peers, ip)?;
            }
            // IPMSG_ANSENTRY: Response to BR_ENTRY (also indicates online presence)
            crate::network::msg_type::IPMSG_ANSENTRY => {
                debug!("📢 Handling ANSENTRY (peer online response)");
                Self::handle_online_msg(peers, msg, sender)?;
            }
            // IPMSG_SENDMSG: Text message - route to MessageHandler
            crate::network::msg_type::IPMSG_SENDMSG => {
                info!(
                    "💌 [TEXT MESSAGE] Routing text message to MessageHandler: from={}, content={}",
                    msg.sender_name,
                    msg.content.chars().take(100).collect::<String>()
                );
                if let Some(ref tx) = *safe_lock!(message_tx) {
                    let route_req = MessageRouteRequest {
                        message: msg,
                        sender,
                    };
                    if let Err(e) = tx.send(route_req) {
                        error!("❌ Failed to send message to MessageHandler: {}", e);
                    } else {
                        debug!("✅ Message routed to MessageHandler successfully");
                    }
                } else {
                    warn!("⚠️ MessageHandler channel not set - text message not routed");
                }
            }
            // IPMSG_RECVMSG: Message acknowledgment - route to MessageHandler
            crate::network::msg_type::IPMSG_RECVMSG => {
                info!("✅ [RECEIPT ACK] Routing message acknowledgment to MessageHandler: from={}, packet_id={}",
                    msg.sender_name, msg.packet_id);
                if let Some(ref tx) = *safe_lock!(message_tx) {
                    let route_req = MessageRouteRequest {
                        message: msg,
                        sender,
                    };
                    if let Err(e) = tx.send(route_req) {
                        error!("❌ Failed to send acknowledgment to MessageHandler: {}", e);
                    } else {
                        debug!("✅ Acknowledgment routed to MessageHandler successfully");
                    }
                } else {
                    warn!("⚠️ MessageHandler channel not set - acknowledgment not routed");
                }
            }
            _ => {
                // Other message types (FILE_SEND_REQ, etc.)
                debug!(
                    "ℹ️ Ignoring message type: {} (mode: {}, options: 0x{:06x})",
                    msg.msg_type,
                    mode,
                    crate::network::msg_type::get_opt(msg.msg_type)
                );
            }
        }

        Ok(())
    }

    /// Handle online message
    fn handle_online_msg(
        peers: &Arc<Mutex<HashMap<IpAddr, PeerNode>>>,
        msg: ProtocolMessage,
        sender: SocketAddr,
    ) -> Result<()> {
        let ip = sender.ip();

        info!(
            "Peer online: {} ({}@{})",
            ip, msg.sender_name, msg.sender_host
        );

        let mut peers = peers.lock().map_err(lock_error)?;

        // Create or update peer
        let peer = peers
            .entry(ip)
            .or_insert_with(|| PeerNode::new(ip, sender.port()));

        // Update peer information
        peer.port = sender.port();
        peer.username = Some(msg.sender_name.clone());
        peer.hostname = Some(msg.sender_host.clone());
        peer.status = PeerStatus::Online;
        peer.last_seen = std::time::SystemTime::now();

        debug!("Peer added/updated: {}", ip);
        Ok(())
    }

    /// Handle offline message
    fn handle_offline_msg(peers: &Arc<Mutex<HashMap<IpAddr, PeerNode>>>, ip: IpAddr) -> Result<()> {
        info!("Peer offline: {}", ip);

        let mut peers = peers.lock().map_err(lock_error)?;

        if let Some(peer) = peers.get_mut(&ip) {
            peer.mark_offline();
            debug!("Peer marked offline: {}", ip);
        } else {
            debug!("Peer not found: {}", ip);
        }

        Ok(())
    }

    /// Handle heartbeat message
    fn handle_heartbeat_msg(
        peers: &Arc<Mutex<HashMap<IpAddr, PeerNode>>>,
        ip: IpAddr,
    ) -> Result<()> {
        debug!("Heartbeat from: {}", ip);

        let mut peers = peers.lock().map_err(lock_error)?;

        if let Some(peer) = peers.get_mut(&ip) {
            peer.update_last_seen();
            // Ensure status is online (in case it was marked offline)
            if peer.status != PeerStatus::Online {
                peer.mark_online();
            }
        } else {
            debug!("Heartbeat from unknown peer: {}", ip);
        }

        Ok(())
    }

    /// Add a peer to the list
    ///
    /// # Arguments
    /// * `peer` - Peer node to add
    ///
    /// # Returns
    /// * `Ok(())` - Peer added successfully
    /// * `Err(NeoLanError)` - Add failed
    pub fn add_peer(&self, peer: PeerNode) -> Result<()> {
        let ip = peer.ip;

        info!("Adding peer: {} ({})", ip, peer.display_name());

        let mut peers = self.peers.lock().map_err(lock_error)?;

        peers.insert(ip, peer);

        Ok(())
    }

    /// Update peer status
    ///
    /// # Arguments
    /// * `ip` - Peer IP address
    /// * `status` - New status
    ///
    /// # Returns
    /// * `Ok(())` - Status updated successfully
    /// * `Err(NeoLanError)` - Update failed
    pub fn update_peer_status(&self, ip: IpAddr, status: PeerStatus) -> Result<()> {
        debug!("Updating peer status: {} -> {:?}", ip, status);

        let mut peers = self.peers.lock().map_err(lock_error)?;

        if let Some(peer) = peers.get_mut(&ip) {
            peer.status = status.clone();
            if status == PeerStatus::Online {
                peer.last_seen = std::time::SystemTime::now();
            }
            Ok(())
        } else {
            warn!("Peer not found: {}", ip);
            Err(crate::NeoLanError::PeerNotFound(ip.to_string()))
        }
    }

    /// Get all peers
    ///
    /// # Returns
    /// * `Vec<PeerNode>` - List of all peers
    pub fn get_all_peers(&self) -> Vec<PeerNode> {
        self.peers
            .lock()
            .map(|peers| peers.values().cloned().collect())
            .unwrap_or_default()
    }

    /// Get online peers only
    ///
    /// # Returns
    /// * `Vec<PeerNode>` - List of online peers
    pub fn get_online_peers(&self) -> Vec<PeerNode> {
        self.peers
            .lock()
            .map(|peers| peers.values().filter(|p| p.is_online()).cloned().collect())
            .unwrap_or_default()
    }

    /// Get a specific peer by IP
    ///
    /// # Arguments
    /// * `ip` - Peer IP address
    ///
    /// # Returns
    /// * `Option<PeerNode>` - Peer if found
    pub fn get_peer(&self, ip: IpAddr) -> Option<PeerNode> {
        self.peers.lock().ok()?.get(&ip).cloned()
    }

    /// Get peer count
    ///
    /// # Returns
    /// * `usize` - Number of peers
    pub fn peer_count(&self) -> usize {
        self.peers.lock().map(|peers| peers.len()).unwrap_or(0)
    }

    /// Get online peer count
    ///
    /// # Returns
    /// * `usize` - Number of online peers
    pub fn online_peer_count(&self) -> usize {
        self.peers
            .lock()
            .map(|peers| peers.values().filter(|p| p.is_online()).count())
            .unwrap_or(0)
    }

    /// Get the discovery service
    pub fn discovery(&self) -> &PeerDiscovery {
        &self.discovery
    }

    /// Remove a peer by IP
    ///
    /// # Arguments
    /// * `ip` - Peer IP address
    ///
    /// # Returns
    /// * `bool` - true if peer was removed, false if not found
    pub fn remove_peer(&self, ip: IpAddr) -> bool {
        self.peers
            .lock()
            .ok()
            .and_then(|mut peers| peers.remove(&ip))
            .is_some()
    }

    /// Check if a peer exists
    ///
    /// # Arguments
    /// * `ip` - Peer IP address
    ///
    /// # Returns
    /// * `bool` - true if peer exists
    pub fn has_peer(&self, ip: IpAddr) -> bool {
        self.peers
            .lock()
            .map(|peers| peers.contains_key(&ip))
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::network::UdpTransport;

    #[test]
    fn test_peer_manager_creation() {
        let udp = UdpTransport::bind(0).unwrap();
        let discovery = PeerDiscovery::new(udp, "TestUser".to_string(), "test-host".to_string());
        let manager = PeerManager::new(discovery);

        assert_eq!(manager.peer_count(), 0);
        assert_eq!(manager.online_peer_count(), 0);
    }

    #[test]
    fn test_add_peer() {
        let udp = UdpTransport::bind(0).unwrap();
        let discovery = PeerDiscovery::new(udp, "TestUser".to_string(), "test-host".to_string());
        let manager = PeerManager::new(discovery);

        let ip = "192.168.1.100".parse().unwrap();
        let peer = PeerNode::new(ip, 2425);

        manager.add_peer(peer).unwrap();

        assert_eq!(manager.peer_count(), 1);
        assert!(manager.has_peer(ip));
    }

    #[test]
    fn test_get_peer() {
        let udp = UdpTransport::bind(0).unwrap();
        let discovery = PeerDiscovery::new(udp, "TestUser".to_string(), "test-host".to_string());
        let manager = PeerManager::new(discovery);

        let ip = "192.168.1.100".parse().unwrap();
        let peer = PeerNode::new(ip, 2425);

        manager.add_peer(peer).unwrap();

        let retrieved = manager.get_peer(ip);
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().ip, ip);
    }

    #[test]
    fn test_update_peer_status() {
        let udp = UdpTransport::bind(0).unwrap();
        let discovery = PeerDiscovery::new(udp, "TestUser".to_string(), "test-host".to_string());
        let manager = PeerManager::new(discovery);

        let ip = "192.168.1.100".parse().unwrap();
        let peer = PeerNode::new(ip, 2425);

        manager.add_peer(peer).unwrap();

        // Update to offline
        manager.update_peer_status(ip, PeerStatus::Offline).unwrap();

        let retrieved = manager.get_peer(ip).unwrap();
        assert_eq!(retrieved.status, PeerStatus::Offline);

        // Online count should be 0
        assert_eq!(manager.online_peer_count(), 0);

        // Update back to online
        manager.update_peer_status(ip, PeerStatus::Online).unwrap();

        let retrieved = manager.get_peer(ip).unwrap();
        assert_eq!(retrieved.status, PeerStatus::Online);
        assert_eq!(manager.online_peer_count(), 1);
    }

    #[test]
    fn test_remove_peer() {
        let udp = UdpTransport::bind(0).unwrap();
        let discovery = PeerDiscovery::new(udp, "TestUser".to_string(), "test-host".to_string());
        let manager = PeerManager::new(discovery);

        let ip = "192.168.1.100".parse().unwrap();
        let peer = PeerNode::new(ip, 2425);

        manager.add_peer(peer).unwrap();
        assert_eq!(manager.peer_count(), 1);

        let removed = manager.remove_peer(ip);
        assert!(removed);
        assert_eq!(manager.peer_count(), 0);

        // Remove again should return false
        let removed_again = manager.remove_peer(ip);
        assert!(!removed_again);
    }

    #[test]
    fn test_get_all_peers() {
        let udp = UdpTransport::bind(0).unwrap();
        let discovery = PeerDiscovery::new(udp, "TestUser".to_string(), "test-host".to_string());
        let manager = PeerManager::new(discovery);

        let ip1 = "192.168.1.100".parse().unwrap();
        let ip2 = "192.168.1.101".parse().unwrap();

        manager.add_peer(PeerNode::new(ip1, 2425)).unwrap();
        manager.add_peer(PeerNode::new(ip2, 2425)).unwrap();

        let all_peers = manager.get_all_peers();
        assert_eq!(all_peers.len(), 2);
    }

    #[test]
    fn test_get_online_peers() {
        let udp = UdpTransport::bind(0).unwrap();
        let discovery = PeerDiscovery::new(udp, "TestUser".to_string(), "test-host".to_string());
        let manager = PeerManager::new(discovery);

        let ip1 = "192.168.1.100".parse().unwrap();
        let ip2 = "192.168.1.101".parse().unwrap();

        let mut peer1 = PeerNode::new(ip1, 2425);
        peer1.mark_offline();

        manager.add_peer(peer1).unwrap();
        manager.add_peer(PeerNode::new(ip2, 2425)).unwrap();

        let online_peers = manager.get_online_peers();
        assert_eq!(online_peers.len(), 1);
        assert_eq!(online_peers[0].ip, ip2);
    }
}

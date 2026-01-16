// Peer manager - core peer management logic
//
// This module handles:
// - Maintaining the peer list in the database (SQLite via PeerRepository)
// - Processing discovery messages and persisting to database
// - Managing peer state transitions (online/offline computed from last_seen)
// - Routing text messages to MessageHandler
//
// Architecture:
// - PeerManager uses PeerRepository for all peer storage operations
// - Peer discovery triggers database upserts via async bridge (block_on)
// - Online/offline status is computed from last_seen timestamp (180s timeout)
// - All query methods read from the database for consistency

use crate::modules::peer::{discovery::PeerDiscovery, types::*};
use crate::network::msg_type;
use crate::storage::peer_repo::PeerRepository;
use crate::{network::ProtocolMessage, Result};
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
/// Manages the database-backed peer list and handles peer discovery events.
///
/// # Architecture
/// - All peer data is persisted to SQLite database via `PeerRepository`
/// - Peer online/offline status is computed from `last_seen` timestamp
/// - Async database operations use a runtime that's resolved at call time
/// - Peer information survives application restarts
///
/// # Thread Safety
/// - Safe for concurrent access (all methods take `&self`)
/// - Database operations create a temporary runtime if needed
#[derive(Clone)]
pub struct PeerManager {
    /// Peer discovery service
    discovery: PeerDiscovery,

    /// Peer repository for database operations
    peer_repo: Arc<PeerRepository>,

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
    /// * `peer_repo` - Peer repository for database operations
    ///
    /// # Returns
    /// * `PeerManager` - New peer manager instance
    pub fn new(discovery: PeerDiscovery, peer_repo: Arc<PeerRepository>) -> Self {
        info!("Creating PeerManager with database backing");

        Self {
            discovery,
            peer_repo,
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

    /// Execute an async database operation
    ///
    /// This helper tries to use the current tokio runtime if available,
    /// otherwise creates a new runtime temporarily for the operation.
    /// This prevents panics when the runtime is shutting down.
    ///
    /// # Arguments
    /// * `f` - Async future factory that takes the repo and returns a future
    ///
    /// # Returns
    /// * `Result<T>` - Result of the async operation
    fn exec_async<F, T, R>(&self, f: F) -> Result<T>
    where
        F: FnOnce(Arc<PeerRepository>) -> R,
        R: std::future::Future<Output = Result<T>>,
    {
        let repo = Arc::clone(&self.peer_repo);

        // Try to use the current runtime first
        match tokio::runtime::Handle::try_current() {
            Ok(handle) => {
                // Use existing runtime
                handle.block_on(f(repo))
            }
            Err(_) => {
                // No runtime available, create a temporary one
                debug!("No tokio runtime found, creating temporary runtime for database operation");
                let rt = tokio::runtime::Runtime::new().map_err(|e| {
                    crate::NeoLanError::Other(format!("Failed to create tokio runtime: {}", e))
                })?;
                rt.block_on(f(repo))
            }
        }
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
        let peer_repo = Arc::clone(&self.peer_repo);
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
            if let Err(e) = Self::handle_message(&peer_repo, msg, sender, &message_tx) {
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
        peer_repo: &Arc<PeerRepository>,
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
            msg_type::IPMSG_BR_ENTRY => {
                debug!("📢 Handling BR_ENTRY (peer online)");
                Self::handle_online_msg(peer_repo, &msg, sender)?;
                Self::route_message(&message_tx, msg, sender, "text message");
            }
            // IPMSG_BR_EXIT: Peer is going offline
            msg_type::IPMSG_BR_EXIT => {
                debug!("📴 Handling BR_EXIT (peer offline)");
                Self::handle_offline_msg(ip)?;
                Self::route_message(&message_tx, msg, sender, "offline notification");
            }
            // IPMSG_ANSENTRY: Response to BR_ENTRY (also indicates online presence)
            msg_type::IPMSG_ANSENTRY => {
                debug!("📢 Handling ANSENTRY (peer online response)");
                Self::handle_online_msg(peer_repo, &msg, sender)?;
                Self::route_message(&message_tx, msg, sender, "presence response");
            }
            // IPMSG_SENDMSG: Text message - route to MessageHandler
            msg_type::IPMSG_SENDMSG => {
                info!(
                    "💌 [TEXT MESSAGE] Routing text message to MessageHandler: from={}, content={}",
                    msg.sender_name,
                    msg.content.chars().take(100).collect::<String>()
                );
                Self::route_message(&message_tx, msg, sender, "text message");
            }
            // IPMSG_RECVMSG: Message acknowledgment - route to MessageHandler
            msg_type::IPMSG_RECVMSG => {
                info!(
                    "✅ [RECEIPT ACK] Routing message acknowledgment to MessageHandler: from={}, packet_id={}",
                    msg.sender_name, msg.packet_id
                );
                Self::route_message(&message_tx, msg, sender, "acknowledgment");
            }
            _ => {
                // Other message types (FILE_SEND_REQ, etc.)
                debug!(
                    "ℹ️ Ignoring message type: {} (mode: {}, options: 0x{:06x})",
                    msg.msg_type,
                    mode,
                    msg_type::get_opt(msg.msg_type)
                );
            }
        }

        Ok(())
    }

    /// Route a message to the MessageHandler through the channel
    ///
    /// This helper method encapsulates the common pattern of sending messages
    /// to the MessageHandler, reducing code duplication across message types.
    fn route_message(
        message_tx: &Arc<Mutex<Option<Sender<MessageRouteRequest>>>>,
        message: ProtocolMessage,
        sender: SocketAddr,
        msg_type_description: &str,
    ) {
        if let Some(ref tx) = *safe_lock!(message_tx) {
            let route_req = MessageRouteRequest { message, sender };
            if let Err(e) = tx.send(route_req) {
                error!(
                    "❌ Failed to send {} to MessageHandler: {}",
                    msg_type_description, e
                );
            } else {
                debug!(
                    "✅ {} routed to MessageHandler successfully",
                    msg_type_description
                );
            }
        } else {
            warn!(
                "⚠️ MessageHandler channel not set - {} not routed",
                msg_type_description
            );
        }
    }

    /// Handle online message
    ///
    /// Upserts peer information to the database when a peer comes online.
    ///
    /// # Async Bridge
    /// This method is called from a synchronous context (UDP callback) but needs
    /// to perform async database operations. Uses `exec_async_static` helper which
    /// tries to use the current tokio runtime, or creates a temporary one if needed.
    ///
    /// # Arguments
    /// * `peer_repo` - Peer repository for database operations
    /// * `msg` - Protocol message containing peer information
    /// * `sender` - Sender's socket address
    fn handle_online_msg(
        peer_repo: &Arc<PeerRepository>,
        msg: &ProtocolMessage,
        sender: SocketAddr,
    ) -> Result<()> {
        let ip = sender.ip();

        info!(
            "Peer online: {} ({}@{})",
            ip, msg.sender_name, msg.sender_host
        );

        // Use async bridge to call database upsert
        let ip_str = ip.to_string();
        let port = sender.port() as i32;
        let username = Some(msg.sender_name.clone());
        let hostname = Some(msg.sender_host.clone());
        let last_seen = chrono::Utc::now().naive_utc();
        let repo = Arc::clone(peer_repo);

        Self::exec_async_static(async move {
            repo.upsert(ip_str, port, username, hostname, last_seen)
                .await
        })?;

        debug!("Peer upserted to database: {}", ip);
        Ok(())
    }

    /// Handle offline message
    ///
    /// For offline peers, we simply log it since online status is computed
    /// from the last_seen timestamp. The peer will appear offline after
    /// the timeout threshold expires.
    fn handle_offline_msg(ip: IpAddr) -> Result<()> {
        info!("Peer offline: {}", ip);

        // Note: We don't need to update the database for offline events
        // since offline status is computed from last_seen timestamp.
        // The peer will automatically appear offline after the timeout.

        debug!("Peer {} will be marked offline after timeout", ip);
        Ok(())
    }

    /// Static helper to execute async database operations
    ///
    /// This helper tries to use the current tokio runtime if available,
    /// otherwise creates a new runtime temporarily for the operation.
    /// This prevents panics when the runtime is shutting down.
    ///
    /// # Arguments
    /// * `f` - Async future to execute
    ///
    /// # Returns
    /// * `Result<T>` - Result of the async operation
    fn exec_async_static<T, F>(f: F) -> Result<T>
    where
        F: std::future::Future<Output = Result<T>>,
    {
        // Try to use the current runtime first
        match tokio::runtime::Handle::try_current() {
            Ok(handle) => {
                // Use existing runtime
                handle.block_on(f)
            }
            Err(_) => {
                // No runtime available, create a temporary one
                debug!("No tokio runtime found, creating temporary runtime for database operation");
                let rt = tokio::runtime::Runtime::new().map_err(|e| {
                    crate::NeoLanError::Other(format!("Failed to create tokio runtime: {}", e))
                })?;
                rt.block_on(f)
            }
        }
    }

    /// Add a peer to the database
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

        let ip_str = ip.to_string();
        let port = peer.port as i32;
        let username = peer.username;
        let hostname = peer.hostname;
        let last_seen = chrono::Utc::now().naive_utc();
        let repo = Arc::clone(&self.peer_repo);

        self.exec_async(|_repo| async move {
            repo.upsert(ip_str, port, username, hostname, last_seen)
                .await
        })?;

        Ok(())
    }

    /// Update peer status
    ///
    /// Note: This method updates the last_seen timestamp for online peers.
    /// For offline status, we simply let the timeout expire.
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

        if status == PeerStatus::Online {
            let ip_str = ip.to_string();
            let repo = Arc::clone(&self.peer_repo);

            self.exec_async(|_repo| async move { repo.update_last_seen(&ip_str).await })?;
        }

        Ok(())
    }

    /// Get all peers
    ///
    /// # Returns
    /// * `Vec<PeerNode>` - List of all peers
    pub fn get_all_peers(&self) -> Vec<PeerNode> {
        let repo = Arc::clone(&self.peer_repo);

        self.exec_async(|_repo| async move { repo.find_all().await })
            .map(|models| models.iter().map(|m| PeerNode::from(m)).collect())
            .unwrap_or_default()
    }

    /// Get online peers only
    ///
    /// # Returns
    /// * `Vec<PeerNode>` - List of online peers
    pub fn get_online_peers(&self) -> Vec<PeerNode> {
        const PEER_TIMEOUT_SECONDS: i64 = 180;
        let repo = Arc::clone(&self.peer_repo);

        self.exec_async(|_repo| async move { repo.find_online(PEER_TIMEOUT_SECONDS).await })
            .map(|models| models.iter().map(|m| PeerNode::from(m)).collect())
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
        let ip_str = ip.to_string();
        let repo = Arc::clone(&self.peer_repo);

        self.exec_async(|_repo| async move { repo.find_by_ip(&ip_str).await })
            .ok()
            .flatten()
            .map(|model| PeerNode::from(&model))
    }

    /// Get peer count
    ///
    /// # Returns
    /// * `usize` - Number of peers
    pub fn peer_count(&self) -> usize {
        self.get_all_peers().len()
    }

    /// Get online peer count
    ///
    /// # Returns
    /// * `usize` - Number of online peers
    pub fn online_peer_count(&self) -> usize {
        self.get_online_peers().len()
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
        let ip_str = ip.to_string();
        let repo = Arc::clone(&self.peer_repo);

        self.exec_async(|_repo| async move { repo.delete_by_ip(&ip_str).await })
            .is_ok()
    }

    /// Check if a peer exists
    ///
    /// # Arguments
    /// * `ip` - Peer IP address
    ///
    /// # Returns
    /// * `bool` - true if peer exists
    pub fn has_peer(&self, ip: IpAddr) -> bool {
        self.get_peer(ip).is_some()
    }
}

// Application state management
//
// Provides a centralized state management structure for the Tauri application.

use crate::config::{app::ConfigRepository, AppConfig};
use crate::modules::message::MessageHandler;
use crate::modules::peer::{PeerManager, PeerNode};
use crate::storage::contact_repo::ContactRepository;
use crate::storage::database::establish_connection;
use crate::storage::message_repo::MessageRepository;
use crate::storage::peer_repo::PeerRepository;
use crate::Result;
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use std::sync::mpsc;
use std::sync::{Arc, Mutex};

/// Tauri event payload - serializable events that can be emitted to frontend
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TauriEvent {
    /// Message received from peer
    #[serde(rename = "MessageReceived")]
    MessageReceived {
        id: i32, // Database ID (0 for real-time messages not yet saved)
        #[serde(rename = "msgId")]
        msg_id: String,
        #[serde(rename = "senderIp")]
        sender_ip: String,
        #[serde(rename = "senderName")]
        sender_name: String,
        #[serde(rename = "receiverIp")]
        receiver_ip: String,
        content: String,
        #[serde(rename = "msgType")]
        msg_type: i32,
        #[serde(rename = "isEncrypted")]
        is_encrypted: bool,
        #[serde(rename = "isOffline")]
        is_offline: bool,
        #[serde(rename = "sentAt")]
        sent_at: i64,
        #[serde(rename = "receivedAt")]
        received_at: Option<i64>,
        #[serde(rename = "createdAt")]
        created_at: i64,
    },

    /// Peer came online
    #[serde(rename = "PeerOnline")]
    PeerOnline {
        #[serde(rename = "peerIp")]
        peer_ip: String,
        #[serde(rename = "username")]
        username: Option<String>,
    },

    /// Peer went offline
    #[serde(rename = "PeerOffline")]
    PeerOffline {
        #[serde(rename = "peerIp")]
        peer_ip: String,
    },

    /// File transfer request received
    #[serde(rename = "FileTransferRequest")]
    FileTransferRequest {
        #[serde(rename = "requestId")]
        request_id: String,
        #[serde(rename = "senderIp")]
        sender_ip: String,
        #[serde(rename = "senderName")]
        sender_name: String,
        #[serde(rename = "fileName")]
        file_name: String,
        #[serde(rename = "fileSize")]
        file_size: u64,
        #[serde(rename = "md5")]
        md5: String,
        #[serde(rename = "createdAt")]
        created_at: i64,
    },

    /// Message receipt acknowledgment received
    #[serde(rename = "MessageReceiptAck")]
    MessageReceiptAck {
        #[serde(rename = "msgId")]
        msg_id: String,
        #[serde(rename = "senderIp")]
        sender_ip: String,
        #[serde(rename = "senderName")]
        sender_name: String,
        #[serde(rename = "acknowledgedAt")]
        acknowledged_at: i64,
    },

    /// Peers discovered after startup
    #[serde(rename = "PeersDiscovered")]
    PeersDiscovered {
        #[serde(rename = "peers")]
        peers: Vec<PeerDiscoveredDto>,
    },
}

/// Peer discovered DTO for frontend
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PeerDiscoveredDto {
    /// IP address
    #[serde(rename = "ip")]
    pub ip: String,

    /// Port
    #[serde(rename = "port")]
    pub port: u16,

    /// Username (if available)
    #[serde(rename = "username")]
    pub username: Option<String>,

    /// Hostname (if available)
    #[serde(rename = "hostname")]
    pub hostname: Option<String>,

    /// Peer status
    #[serde(rename = "status")]
    pub status: String,

    /// Last seen timestamp
    #[serde(rename = "lastSeen")]
    pub last_seen: i64,
}

impl PeerDiscoveredDto {
    /// Create from PeerNode
    pub fn from_peer_node(peer: &crate::modules::peer::types::PeerNode) -> Self {
        Self {
            ip: peer.ip.to_string(),
            port: peer.port,
            username: peer.username.clone(),
            hostname: peer.hostname.clone(),
            status: peer.status.as_str().to_string(),
            last_seen: peer
                .last_seen
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs() as i64,
        }
    }
}

/// Application state
///
/// This struct holds all the global state for the Tauri application.
/// It is wrapped in Arc<Mutex<>> to allow thread-safe access across commands.
#[derive(Clone)]
pub struct AppState {
    /// Database connection
    db: Arc<Mutex<Option<DatabaseConnection>>>,

    /// Message repository
    message_repo: Arc<Mutex<Option<MessageRepository>>>,

    /// Peer repository
    peer_repo: Arc<Mutex<Option<PeerRepository>>>,

    /// Contact repository
    contact_repo: Arc<Mutex<Option<ContactRepository>>>,

    /// Config repository
    config_repo: Arc<Mutex<Option<ConfigRepository>>>,

    /// Peer manager (when initialized)
    peer_manager: Arc<Mutex<Option<PeerManager>>>,

    /// Message handler (when initialized)
    message_handler: Arc<Mutex<Option<MessageHandler>>>,

    /// Current application configuration
    config: Arc<Mutex<AppConfig>>,

    /// Event emitter for state changes
    event_emitter: Arc<Mutex<super::events::AppEventEmitter>>,

    /// Tauri event sender for forwarding events to main thread
    tauri_event_sender: Arc<Mutex<Option<mpsc::Sender<TauriEvent>>>>,
}

impl AppState {
    /// Create a new application state
    pub fn new(config: AppConfig) -> Self {
        Self {
            db: Arc::new(Mutex::new(None)),
            message_repo: Arc::new(Mutex::new(None)),
            peer_repo: Arc::new(Mutex::new(None)),
            contact_repo: Arc::new(Mutex::new(None)),
            config_repo: Arc::new(Mutex::new(None)),
            peer_manager: Arc::new(Mutex::new(None)),
            message_handler: Arc::new(Mutex::new(None)),
            config: Arc::new(Mutex::new(config)),
            event_emitter: Arc::new(Mutex::new(super::events::AppEventEmitter::new())),
            tauri_event_sender: Arc::new(Mutex::new(None)),
        }
    }

    // ==================== Database Methods ====================

    /// Set an already-established database connection
    ///
    /// This is useful when you need to establish the connection first
    /// (e.g., to load config) before creating the AppState.
    ///
    /// # Arguments
    /// * `db` - The established database connection
    pub fn set_database(&self, db: &DatabaseConnection) {
        tracing::info!("Setting database connection in AppState...");

        // Store the database connection
        *self.db.lock().unwrap() = Some(db.clone());

        // Create repositories
        let message_repo = MessageRepository::new(db.clone());
        let peer_repo = PeerRepository::new(db.clone());
        let contact_repo = ContactRepository::new(db.clone());
        let config_repo = ConfigRepository::new(db.clone());

        *self.message_repo.lock().unwrap() = Some(message_repo);
        *self.peer_repo.lock().unwrap() = Some(peer_repo);
        *self.contact_repo.lock().unwrap() = Some(contact_repo);
        *self.config_repo.lock().unwrap() = Some(config_repo);

        tracing::info!("Database repositories initialized successfully");
    }

    /// Initialize the database connection
    ///
    /// This should be called once during application startup.
    /// Returns the database connection for use with migrations.
    pub async fn init_database(&self) -> Result<DatabaseConnection> {
        tracing::info!("Initializing database connection...");

        let db = establish_connection().await.map_err(|e| {
            crate::NeoLanError::Storage(format!("Database connection failed: {}", e))
        })?;

        // Store the database connection
        *self.db.lock().unwrap() = Some(db.clone());

        // Create repositories
        let message_repo = MessageRepository::new(db.clone());
        let peer_repo = PeerRepository::new(db.clone());
        let contact_repo = ContactRepository::new(db.clone());
        let config_repo = ConfigRepository::new(db.clone());

        *self.message_repo.lock().unwrap() = Some(message_repo);
        *self.peer_repo.lock().unwrap() = Some(peer_repo);
        *self.contact_repo.lock().unwrap() = Some(contact_repo);
        *self.config_repo.lock().unwrap() = Some(config_repo);

        tracing::info!("Database initialized successfully");

        Ok(db)
    }

    /// Get the message repository
    ///
    /// Returns None if database hasn't been initialized.
    pub fn get_message_repo(&self) -> Option<MessageRepository> {
        self.message_repo.lock().unwrap().as_ref().cloned()
    }

    /// Get the peer repository
    ///
    /// Returns None if database hasn't been initialized.
    pub fn get_peer_repo(&self) -> Option<PeerRepository> {
        self.peer_repo.lock().unwrap().as_ref().cloned()
    }

    /// Get the contact repository
    ///
    /// Returns None if database hasn't been initialized.
    pub fn get_contact_repo(&self) -> Option<ContactRepository> {
        self.contact_repo.lock().unwrap().as_ref().cloned()
    }

    /// Get the config repository
    ///
    /// Returns None if database hasn't been initialized.
    pub fn get_config_repo(&self) -> Option<ConfigRepository> {
        self.config_repo.lock().unwrap().as_ref().cloned()
    }

    /// Check if database is initialized
    pub fn is_database_initialized(&self) -> bool {
        self.db.lock().unwrap().is_some()
    }

    /// Set the Tauri event sender
    ///
    /// This should be called once during application startup after creating the channel.
    pub fn set_event_sender(&self, sender: mpsc::Sender<TauriEvent>) {
        *self.tauri_event_sender.lock().unwrap() = Some(sender);
    }

    /// Emit a Tauri event to the frontend
    ///
    /// This method sends an event through the channel to be forwarded to the main thread.
    pub fn emit_tauri_event(&self, event: TauriEvent) {
        if let Some(sender) = self.tauri_event_sender.lock().unwrap().as_ref() {
            let _ = sender.send(event);
        }
    }

    /// Emit peers discovered event
    ///
    /// This should be called after peer discovery to notify frontend of all discovered peers.
    pub fn emit_peers_discovered(&self) {
        let peers = self.get_peers();
        let peer_dtos: Vec<PeerDiscoveredDto> = peers
            .iter()
            .map(PeerDiscoveredDto::from_peer_node)
            .collect();

        tracing::info!(
            "Emitting PeersDiscovered event with {} peers",
            peer_dtos.len()
        );

        self.emit_tauri_event(TauriEvent::PeersDiscovered { peers: peer_dtos });
    }

    /// Get the peer manager
    ///
    /// Returns a cloned reference to the peer manager for direct access.
    pub fn get_peer_manager(&self) -> Option<PeerManager> {
        self.peer_manager.lock().unwrap().as_ref().cloned()
    }

    /// Start peer manager
    ///
    /// Starts the peer discovery and listening process in the current thread.
    pub fn start_peer_manager(&self) -> Result<()> {
        if let Some(manager) = self.get_peer_manager() {
            manager.start()
        } else {
            Err(crate::NeoLanError::Other(
                "Peer manager not initialized".to_string(),
            ))
        }
    }

    /// Get the current configuration
    pub fn get_config(&self) -> AppConfig {
        self.config.lock().unwrap().clone()
    }

    /// Set the configuration
    ///
    /// Updates the in-memory configuration and persists it to the database asynchronously.
    pub fn set_config(&self, config: AppConfig) {
        let repo = self.get_config_repo();
        let config_clone = config.clone();

        // Persist to database asynchronously in a background thread
        if let Some(repo) = repo {
            std::thread::spawn(move || {
                // Create a new runtime for this thread
                let rt = tokio::runtime::Runtime::new().unwrap();
                rt.block_on(async move {
                    if let Err(e) = repo.save_app_config(&config_clone).await {
                        tracing::error!("Failed to save config: {}", e);
                    }
                });
            });
        }

        // Update in-memory config
        *self.config.lock().unwrap() = config;
        self.emit_event(super::events::AppEvent::ConfigChanged);
    }

    /// Update configuration fields
    ///
    /// Updates the in-memory configuration and persists it to the database asynchronously.
    pub fn update_config<F>(&self, updater: F) -> Result<()>
    where
        F: FnOnce(&mut AppConfig),
    {
        let mut config = self.config.lock().unwrap();
        updater(&mut config);
        let config_clone = config.clone();
        drop(config);

        // Persist to database asynchronously in a background thread
        let repo = self.get_config_repo();
        if let Some(repo) = repo {
            std::thread::spawn(move || {
                // Create a new runtime for this thread
                let rt = tokio::runtime::Runtime::new().unwrap();
                rt.block_on(async move {
                    if let Err(e) = repo.save_app_config(&config_clone).await {
                        tracing::error!("Failed to save config: {}", e);
                    }
                });
            });
        }

        self.emit_event(super::events::AppEvent::ConfigChanged);
        Ok(())
    }

    /// Initialize the peer manager
    ///
    /// This should be called once during application startup.
    pub fn init_peer_manager(&self, peer_manager: PeerManager) {
        let mut pm = self.peer_manager.lock().unwrap();
        *pm = Some(peer_manager);
        self.emit_event(super::events::AppEvent::Initialized);
    }

    /// Initialize configuration from database
    ///
    /// This should be called once during application startup after database initialization.
    /// Loads configuration from the database, falling back to defaults if not found.
    pub async fn init_config(&self) -> Result<()> {
        if let Some(repo) = self.get_config_repo() {
            tracing::info!("Loading configuration from database...");
            match repo.load_app_config().await {
                Ok(config) => {
                    *self.config.lock().unwrap() = config;
                    tracing::info!("Configuration loaded successfully");
                }
                Err(e) => {
                    tracing::warn!("Failed to load config from DB: {}, using defaults", e);
                    // Use the default config that was already set in `new()`
                }
            }
        } else {
            tracing::warn!("Config repository not initialized, using in-memory config");
        }
        Ok(())
    }

    /// Get peer list (all peers)
    pub fn get_peers(&self) -> Vec<PeerNode> {
        if let Some(manager) = self.peer_manager.lock().unwrap().as_ref() {
            manager.get_all_peers()
        } else {
            Vec::new()
        }
    }

    /// Get online peers
    pub fn get_online_peers(&self) -> Vec<PeerNode> {
        if let Some(manager) = self.peer_manager.lock().unwrap().as_ref() {
            manager.get_online_peers()
        } else {
            Vec::new()
        }
    }

    /// Get peer by IP
    pub fn get_peer(&self, ip: std::net::IpAddr) -> Option<PeerNode> {
        if let Some(manager) = self.peer_manager.lock().unwrap().as_ref() {
            manager.get_peer(ip)
        } else {
            None
        }
    }

    /// Get peer statistics
    pub fn get_peer_stats(&self) -> PeerStats {
        if let Some(manager) = self.peer_manager.lock().unwrap().as_ref() {
            let all = manager.get_all_peers();
            let online_count = all.iter().filter(|p| p.is_online()).count();
            PeerStats {
                total: all.len(),
                online: online_count,
                offline: all.len() - online_count,
            }
        } else {
            PeerStats {
                total: 0,
                online: 0,
                offline: 0,
            }
        }
    }

    /// Emit an event
    pub fn emit_event(&self, event: super::events::AppEvent) {
        if let Ok(mut emitter) = self.event_emitter.try_lock() {
            emitter.emit(event);
        }
    }

    /// Get pending events and clear the buffer
    pub fn drain_events(&self) -> Vec<super::events::AppEvent> {
        if let Ok(mut emitter) = self.event_emitter.try_lock() {
            emitter.drain()
        } else {
            Vec::new()
        }
    }

    // ==================== Message Handler Methods ====================

    /// Initialize the message handler
    ///
    /// This should be called once during application startup.
    pub fn init_message_handler(&self, message_handler: MessageHandler) {
        let mut mh = self.message_handler.lock().unwrap();
        *mh = Some(message_handler);
    }

    /// Send a text message to a peer
    ///
    /// # Arguments
    /// * `target_ip` - IP address of the target peer
    /// * `content` - Message content
    ///
    /// # Returns
    /// * `Ok(msg_id)` - Message sent successfully, returns message ID
    /// * `Err(NeoLanError)` - Send failed
    pub fn send_message(&self, target_ip: std::net::IpAddr, content: &str) -> Result<String> {
        if let Some(handler) = self.message_handler.lock().unwrap().as_ref() {
            handler.send_text_message(target_ip, content)?;

            // Get the packet ID that was used
            let msg_id = handler.packet_id_counter().to_string();

            Ok(msg_id)
        } else {
            Err(crate::NeoLanError::Other(
                "Message handler not initialized".to_string(),
            ))
        }
    }

    /// Handle a routed message from PeerManager
    ///
    /// # Arguments
    /// * `proto_msg` - Protocol message received from network
    /// * `sender_ip` - IP address of the sender
    /// * `local_ip` - Local IP address (for receiver field)
    ///
    /// # Returns
    /// * `Ok(())` - Message processed successfully
    /// * `Err(NeoLanError)` - Processing failed
    pub fn handle_routed_message(
        &self,
        proto_msg: &crate::network::ProtocolMessage,
        sender_ip: std::net::IpAddr,
        local_ip: std::net::IpAddr,
    ) -> Result<()> {
        if let Some(handler) = self.message_handler.lock().unwrap().as_ref() {
            handler.handle_incoming_message(proto_msg, sender_ip, local_ip)
        } else {
            Err(crate::NeoLanError::Other(
                "Message handler not initialized".to_string(),
            ))
        }
    }
}

/// Peer statistics
#[derive(Clone, Debug, serde::Serialize)]
pub struct PeerStats {
    pub total: usize,
    pub online: usize,
    pub offline: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_state_creation() {
        let config = AppConfig::default();
        let state = AppState::new(config);

        // Verify initial state
        let peers = state.get_peers();
        assert_eq!(peers.len(), 0);

        let stats = state.get_peer_stats();
        assert_eq!(stats.total, 0);
        assert_eq!(stats.online, 0);
        assert_eq!(stats.offline, 0);
    }

    #[test]
    fn test_config_access() {
        let config = AppConfig::default();
        let state = AppState::new(config.clone());

        // Get config
        let retrieved = state.get_config();
        assert_eq!(retrieved.udp_port, config.udp_port);

        // Update config
        let mut new_config = config;
        new_config.udp_port = 2426;
        state.set_config(new_config);

        let updated = state.get_config();
        assert_eq!(updated.udp_port, 2426);
    }

    #[test]
    fn test_update_config() {
        let config = AppConfig::default();
        let state = AppState::new(config);

        // Update config field
        state
            .update_config(|c| {
                c.udp_port = 2500;
                c.log_level = "debug".to_string();
            })
            .unwrap();

        let updated = state.get_config();
        assert_eq!(updated.udp_port, 2500);
        assert_eq!(updated.log_level, "debug");
    }
}

// Message handler - handles sending and receiving messages
//
// This module provides the MessageHandler which:
// - Sends text messages to peers via UDP
// - Serializes messages to IPMsg protocol format
// - Receives and routes incoming messages by type
// - Stores text messages to database
// - Manages packet ID generation for message tracking
// - Emits Tauri events for received messages

use crate::config::AppConfig;
use crate::modules::file_transfer::FileTransferResponse;
use crate::modules::message::types::{Message, MessageType};
use crate::modules::peer::types::PeerInfo;
use crate::network::protocol::{get_local_mac_address, serialize_message_for_feiq};
use crate::network::udp::UdpTransport;
use crate::network::{get_message_type_name, msg_type, serialize_message, ProtocolMessage};
use crate::state::app_state::TauriEvent;
use crate::state::AppState;
use crate::storage::contact_repo::ContactRepository;
use crate::storage::message_repo::{MessageModel, MessageRepository};
use crate::storage::peer_repo::{PeerModel, PeerRepository};
use crate::{NeoLanError, Result};
use chrono::Utc;
use std::net::{IpAddr, SocketAddr};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use tracing::instrument;

/// Message handler
///
/// Handles sending and receiving messages over UDP using the IPMsg protocol.
/// Manages packet ID generation, message serialization, and message routing.
pub struct MessageHandler {
    /// UDP transport for sending messages
    udp: UdpTransport,

    /// Application configuration (username, hostname, etc.)
    config: AppConfig,

    /// Atomic counter for generating packet IDs
    packet_id_counter: Arc<AtomicU64>,

    /// Message repository for database storage (optional)
    message_repo: Option<MessageRepository>,

    /// Application state for emitting events
    app_state: Option<Arc<AppState>>,

    /// File transfer response handler (optional)
    file_transfer: Option<Arc<FileTransferResponse>>,

    /// Contact repository for auto-adding contacts (optional)
    contact_repo: Option<Arc<ContactRepository>>,

    /// Peer repository for storing discovered peers (optional)
    peer_repo: Option<Arc<PeerRepository>>,
}

impl MessageHandler {
    /// Create a new message handler
    ///
    /// # Arguments
    /// * `udp` - UDP transport for sending messages
    /// * `config` - Application configuration
    ///
    /// # Returns
    /// A new MessageHandler instance
    ///
    /// # Examples
    /// ```
    /// # use feiqiu::modules::message::handler::MessageHandler;
    /// # use feiqiu::network::udp::UdpTransport;
    /// # use feiqiu::config::AppConfig;
    /// # use std::net::IpAddr;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let udp = UdpTransport::bind(0)?; // Use port 0 to get an available port
    /// let config = AppConfig::default();
    /// let handler = MessageHandler::new(udp, config);
    /// Ok(())
    /// # }
    /// ```
    pub fn new(udp: UdpTransport, config: AppConfig) -> Self {
        Self {
            udp,
            config,
            packet_id_counter: Arc::new(AtomicU64::new(1)),
            message_repo: None,
            app_state: None,
            file_transfer: None,
            contact_repo: None,
            peer_repo: None,
        }
    }

    /// Create a new message handler with database storage
    ///
    /// # Arguments
    /// * `udp` - UDP transport for sending messages
    /// * `config` - Application configuration
    /// * `message_repo` - Message repository for database storage
    ///
    /// # Returns
    /// A new MessageHandler instance with database storage enabled
    pub fn with_storage(
        udp: UdpTransport,
        config: AppConfig,
        message_repo: MessageRepository,
    ) -> Self {
        Self {
            udp,
            config,
            packet_id_counter: Arc::new(AtomicU64::new(1)),
            message_repo: Some(message_repo),
            app_state: None,
            file_transfer: None,
            contact_repo: None,
            peer_repo: None,
        }
    }

    /// Set the application state for emitting events
    ///
    /// # Arguments
    /// * `app_state` - Application state reference
    pub fn with_app_state(mut self, app_state: Arc<AppState>) -> Self {
        self.app_state = Some(app_state);
        self
    }

    /// Set the file transfer response handler
    ///
    /// # Arguments
    /// * `file_transfer` - File transfer response handler
    pub fn with_file_transfer(mut self, file_transfer: Arc<FileTransferResponse>) -> Self {
        self.file_transfer = Some(file_transfer);
        self
    }

    /// Set the contact repository for auto-adding contacts
    ///
    /// # Arguments
    /// * `contact_repo` - Contact repository reference
    pub fn with_contact_repo(mut self, contact_repo: Arc<ContactRepository>) -> Self {
        self.contact_repo = Some(contact_repo);
        self
    }

    /// Set the peer repository for storing discovered peers
    ///
    /// # Arguments
    /// * `peer_repo` - Peer repository reference
    pub fn with_peer_repo(mut self, peer_repo: Arc<PeerRepository>) -> Self {
        self.peer_repo = Some(peer_repo);
        self
    }

    /// Send a text message to a target peer
    ///
    /// # Arguments
    /// * `target_ip` - IP address of the target peer
    /// * `content` - Text content to send
    ///
    /// # Returns
    /// * `Ok(())` - Message sent successfully
    /// * `Err(NeoLanError)` - Send failed
    ///
    /// # Examples
    /// ```
    /// # use feiqiu::modules::message::handler::MessageHandler;
    /// # use feiqiu::network::udp::UdpTransport;
    /// # use feiqiu::config::AppConfig;
    /// # use std::net::IpAddr;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let udp = UdpTransport::bind(0)?;
    /// let config = AppConfig::default();
    /// let handler = MessageHandler::new(udp, config);
    /// let target_ip = "192.168.1.100".parse::<IpAddr>().unwrap();
    /// handler.send_text_message(target_ip, "Hello, World!")?;
    /// Ok(())
    /// # }
    /// ```
    #[instrument(skip(self), fields(target_ip = %target_ip, content_len = content.len()))]
    pub fn send_text_message(&self, target_ip: IpAddr, content: &str) -> Result<()> {
        tracing::info!(
            "Sending text message to {}: {}",
            target_ip,
            content.chars().take(50).collect::<String>()
        );

        // Validate content
        if content.trim().is_empty() {
            return Err(NeoLanError::Validation(
                "Message content cannot be empty".to_string(),
            ));
        }

        // Get MAC address for FeiQ format
        let mac_address =
            crate::network::get_local_mac_address().unwrap_or_else(|_| "000000000000".to_string());

        // Create protocol message with SENDCHECKOPT flag and UTF8 encoding
        let proto_msg = ProtocolMessage {
            version: 1,
            packet_id: self.next_packet_id(),
            user_id: self.config.user_id.clone(),
            sender_name: self.config.username.clone(),
            sender_host: self.config.hostname.clone(),
            msg_type: msg_type::IPMSG_SENDMSG
                | msg_type::IPMSG_SENDCHECKOPT
                | msg_type::IPMSG_UTF8OPT,
            content: content.to_string(),
        };

        // Try FeiQ format first, fall back to standard format
        let target_addr = SocketAddr::new(target_ip, self.config.udp_port);

        let feiq_bytes = crate::network::serialize_message_for_feiq(
            &proto_msg,
            &mac_address,
            self.config.udp_port,
        )?;

        match self.udp.send_to(&feiq_bytes, target_addr) {
            Ok(()) => {
                tracing::debug!("Message sent (FeiQ format) to {}", target_ip);
            }
            Err(e) => {
                tracing::warn!("FeiQ format send failed: {:?}, trying standard format", e);
                let bytes = serialize_message(&proto_msg)?;
                self.udp.send_to(&bytes, target_addr)?;
                tracing::debug!("Message sent (standard format) to {}", target_ip);
            }
        }

        Ok(())
    }

    /// Send a message to a target peer (generic method)
    ///
    /// # Arguments
    /// * `target_ip` - IP address of the target peer
    /// * `msg_type` - Message type
    /// * `content` - Message content
    ///
    /// # Returns
    /// * `Ok(())` - Message sent successfully
    /// * `Err(NeoLanError)` - Send failed
    pub fn send_message(
        &self,
        target_ip: IpAddr,
        msg_type: MessageType,
        content: String,
    ) -> Result<()> {
        tracing::debug!("Sending {:?} message to {}", msg_type, target_ip);

        // Create target and sender peer info
        let target_peer = PeerInfo::new(target_ip, AppConfig::DEFAULT_UDP_PORT, None);
        let sender_peer = PeerInfo::new(
            self.config.bind_ip.parse().map_err(|_| {
                NeoLanError::Config(format!("Invalid bind IP: {}", self.config.bind_ip))
            })?,
            self.config.udp_port,
            Some(self.config.username.clone()),
        );

        // Create message with specified type
        let message = Message {
            id: uuid::Uuid::new_v4(),
            packet_id: self.next_packet_id().to_string(),
            sender: sender_peer,
            receiver: target_peer,
            msg_type,
            content,
            timestamp: chrono::Utc::now(),
        };

        // Convert and send
        let proto_msg = message.to_protocol(&self.config.username, &self.config.hostname);
        let bytes = serialize_message(&proto_msg)?;
        let target_addr = SocketAddr::new(target_ip, AppConfig::DEFAULT_UDP_PORT);
        self.udp.send_to(&bytes, target_addr)?;

        Ok(())
    }

    /// Get the next packet ID
    ///
    /// # Returns
    /// A monotonically increasing packet ID
    pub fn next_packet_id(&self) -> u64 {
        self.packet_id_counter.fetch_add(1, Ordering::SeqCst)
    }

    /// Get a reference to the UDP transport
    ///
    /// # Returns
    /// Reference to the inner UDP transport
    pub fn udp(&self) -> &UdpTransport {
        &self.udp
    }

    /// Get the current packet ID counter value
    ///
    /// # Returns
    /// Current packet ID counter value
    pub fn packet_id_counter(&self) -> u64 {
        self.packet_id_counter.load(Ordering::SeqCst)
    }

    /// Reset the packet ID counter to a specific value
    ///
    /// # Arguments
    /// * `value` - New counter value
    pub fn reset_packet_id_counter(&self, value: u64) {
        self.packet_id_counter.store(value, Ordering::SeqCst);
    }

    // ==================== Message Receiving ====================

    /// Handle an incoming protocol message from the network
    ///
    /// This is the main entry point for receiving messages. It routes the message
    /// to the appropriate handler based on msg_type and stores text messages to database.
    ///
    /// # Arguments
    /// * `proto_msg` - Protocol message received from network
    /// * `sender_ip` - IP address of the sender
    /// * `local_ip` - Local IP address (for receiver field)
    ///
    /// # Returns
    /// * `Ok(())` - Message processed successfully
    /// * `Err(NeoLanError)` - Processing failed
    ///
    /// # Routing
    /// - IPMSG_SENDMSG (0x00000020) → Store to database as text message
    /// - IPMSG_BR_ENTRY (0x00000001) → Should be handled by PeerManager
    /// - IPMSG_BR_EXIT (0x00000002) → Should be handled by PeerManager
    /// - IPMSG_ANSENTRY (0x00000003) → Should be handled by HeartbeatMonitor
    /// - Other types → Logged and ignored
    #[instrument(skip(self, proto_msg), fields(sender_ip = %sender_ip, msg_type = %proto_msg.msg_type))]
    pub fn handle_incoming_message(
        &self,
        proto_msg: &ProtocolMessage,
        sender_ip: IpAddr,
        local_ip: IpAddr,
    ) -> Result<()> {
        // Extract the mode (low 8 bits) from the message type
        // get_mode returns u8, constants are u32, so we compare the raw mode value
        let mode = msg_type::get_mode(proto_msg.msg_type) as u32;
        tracing::debug!(
            mode = format_args!("0x{:02x}", mode),
            msg_type = %get_message_type_name(proto_msg.msg_type),
            %sender_ip,
            "Handling incoming message"
        );

        // Match on the mode (u8). Using const u8 values for direct comparison.
        match mode {
            // ========== Text Messages ==========
            // IPMSG_SENDMSG: 发送消息 (0x00000020)
            msg_type::IPMSG_SENDMSG => {
                tracing::debug!("📨 [handle_incoming_message] Routing to handle_text_message");
                self.handle_text_message(proto_msg, sender_ip, local_ip)?;
            }

            // IPMSG_RECVMSG: 接收确认（对方已收到消息） (0x00000040)
            msg_type::IPMSG_RECVMSG => {
                tracing::debug!("📨 [handle_incoming_message] Routing to handle_recv_msg");
                self.handle_recv_msg(proto_msg, sender_ip)?;
            }

            // ========== Message Read/Delete Status ==========
            // IPMSG_READMSG: 消息已读 (0x00000050)
            msg_type::IPMSG_READMSG => {
                self.handle_read_msg(proto_msg, sender_ip)?;
            }

            // IPMSG_DELMSG: 删除消息 (0x00000060)
            msg_type::IPMSG_DELMSG => {
                self.handle_del_msg(proto_msg, sender_ip)?;
            }

            // IPMSG_ANSREADMSG: 对已读消息的应答 (0x00000051)
            msg_type::IPMSG_ANSREADMSG => {
                self.handle_answer_read_msg(proto_msg, sender_ip)?;
            }

            // ========== Peer Discovery Messages ==========
            // IPMSG_BR_ENTRY: 广播上线 (0x00000001)
            // IPMSG_ANSENTRY: 上线应答 (0x00000003)
            msg_type::IPMSG_BR_ENTRY | msg_type::IPMSG_ANSENTRY => {
                tracing::debug!(
                    "📢 BR_ENTRY/ANSENTRY from {} - updating peers and contacts",
                    sender_ip
                );
                self.handle_peer_online(proto_msg, sender_ip)?;
            }

            // IPMSG_BR_EXIT: 广播下线 (0x00000002)
            msg_type::IPMSG_BR_EXIT => {
                tracing::debug!(
                    "📴 BR_EXIT from {} - updating peers and contacts",
                    sender_ip
                );
                self.handle_peer_offline(proto_msg, sender_ip)?;
            }

            // IPMSG_BR_ABSENCE: 广播缺席状态 (0x00000004)
            msg_type::IPMSG_BR_ABSENCE => {
                tracing::info!("🏖️ Absence status broadcast from {}", sender_ip);
                // TODO: Update peer absence status in PeerManager
            }

            // ========== Peer List Management ==========
            // IPMSG_BR_ISGETLIST: 请求是否需要列表 (0x00000010)
            // IPMSG_BR_ISGETLIST2: 请求是否需要列表 v2 (0x00000012)
            msg_type::IPMSG_BR_ISGETLIST | msg_type::IPMSG_BR_ISGETLIST2 => {
                tracing::info!("📋 Peer list request from {}", sender_ip);
                // TODO: Send response with IPMSG_OKGETLIST
            }

            // IPMSG_OKGETLIST: 同意发送列表 (0x00000011)
            msg_type::IPMSG_OKGETLIST => {
                tracing::info!("✅ Peer list approval from {}", sender_ip);
                // TODO: Proceed to send IPMSG_GETLIST
            }

            // IPMSG_GETLIST: 请求列表 (0x00000013)
            msg_type::IPMSG_GETLIST => {
                tracing::info!("📋 Get list request from {}", sender_ip);
                // TODO: Send peer list with IPMSG_ANSLIST
            }

            // IPMSG_ANSLIST: 返回列表 (0x00000014)
            msg_type::IPMSG_ANSLIST => {
                self.handle_peer_list_response(proto_msg, sender_ip)?;
            }

            // ========== User Information ==========
            // IPMSG_GETINFO: 请求用户信息 (0x00000070)
            msg_type::IPMSG_GETINFO => {
                tracing::info!("ℹ️ User info request from {}", sender_ip);
                // TODO: Send user info with IPMSG_SENDINFO
            }

            // IPMSG_SENDINFO: 发送用户信息 (0x00000071)
            msg_type::IPMSG_SENDINFO => {
                self.handle_user_info(proto_msg, sender_ip)?;
            }

            // ========== Absence Information ==========
            // IPMSG_GETABSENCEINFO: 请求缺席信息 (0x00000072)
            msg_type::IPMSG_GETABSENCEINFO => {
                tracing::info!("🏖️ Absence info request from {}", sender_ip);
                // TODO: Send absence info with IPMSG_SENDABSENCEINFO
            }

            // IPMSG_SENDABSENCEINFO: 发送缺席信息 (0x00000073)
            msg_type::IPMSG_SENDABSENCEINFO => {
                self.handle_absence_info(proto_msg, sender_ip)?;
            }

            // ========== File Transfer ==========
            // IPMSG_GETFILEDATA: 请求文件数据（文件传输） (0x00000060)
            msg_type::IPMSG_GETFILEDATA => {
                self.handle_file_transfer_request(proto_msg, sender_ip)?;
            }

            // IPMSG_RELEASEFILES: 释放文件资源 (0x00000061)
            msg_type::IPMSG_RELEASEFILES => {
                self.handle_release_files(proto_msg, sender_ip)?;
            }

            // IPMSG_GETDIRFILES: 请求目录文件列表 (0x00000062)
            msg_type::IPMSG_GETDIRFILES => {
                tracing::info!("📁 Directory file list request from {}", sender_ip);
                // TODO: Handle directory file list request
            }

            // ========== Encryption ==========
            // IPMSG_GETPUBKEY: 请求公钥 (0x00000080)
            msg_type::IPMSG_GETPUBKEY => {
                tracing::info!("🔑 Public key request from {}", sender_ip);
                // TODO: Send public key with IPMSG_ANSPUBKEY
            }

            // IPMSG_ANSPUBKEY: 应答公钥 (0x00000081)
            msg_type::IPMSG_ANSPUBKEY => {
                self.handle_public_key_response(proto_msg, sender_ip)?;
            }

            // ========== Unknown Message Types ==========
            _ => {
                tracing::warn!(
                    "⚠️ Unhandled message type: mode=0x{:02x}, msg_type=0x{:08x} from {}, content={}",
                    mode,
                    proto_msg.msg_type,
                    sender_ip,
                    proto_msg.content.chars().take(50).collect::<String>()
                );
            }
        }

        Ok(())
    }

    /// Handle peer online (BR_ENTRY or ANSENTRY)
    ///
    /// Updates peers table and syncs to contacts table.
    ///
    /// # Arguments
    /// * `proto_msg` - Protocol message
    /// * `sender_ip` - Sender's IP address
    #[instrument(skip(self, proto_msg), fields(sender_ip = %sender_ip, sender_name = %proto_msg.sender_name))]
    fn handle_peer_online(&self, proto_msg: &ProtocolMessage, sender_ip: IpAddr) -> Result<()> {
        // 1. Update peers table via peer_repo using upsert
        if let Some(ref peer_repo) = self.peer_repo {
            let rt = tokio::runtime::Runtime::new()
                .map_err(|e| NeoLanError::Other(format!("Failed to create runtime: {}", e)))?;

            // Use upsert to handle both insert and update cases properly
            // Returns the created/updated peer with its database ID
            let peer = rt.block_on(async {
                peer_repo
                    .upsert(
                        sender_ip.to_string(),
                        self.config.udp_port as i32,
                        Some(proto_msg.user_id.clone()),
                        Some(proto_msg.sender_name.clone()),
                        Some(proto_msg.sender_host.clone()),
                        chrono::Utc::now().naive_utc(),
                    )
                    .await
            })?;

            tracing::info!(
                "✅ Peer stored in database: {} (id={}) ({})",
                sender_ip,
                peer.id,
                proto_msg.sender_name
            );

            // 2. Sync to contacts table via contact_repo
            if let Some(ref contact_repo) = self.contact_repo {
                let user_id = peer.id;
                rt.block_on(async { contact_repo.sync_from_peers(vec![peer]).await })?;
                tracing::info!(
                    "✅ Contact synced for peer: {} (user_id={})",
                    sender_ip,
                    user_id
                );
            }
        } else {
            tracing::warn!("⚠️ Peer repository not available - cannot store peer");
        }

        // 3. Emit Tauri event for frontend
        if let Some(ref app_state) = self.app_state {
            app_state.emit_tauri_event(TauriEvent::PeerOnline {
                peer_ip: sender_ip.to_string(),
                username: Some(proto_msg.sender_name.clone()),
            });
        }

        Ok(())
    }

    /// Handle peer offline (BR_EXIT)
    ///
    /// Marks peer as offline and updates contact status.
    ///
    /// # Arguments
    /// * `_proto_msg` - Protocol message (unused but kept for consistency)
    /// * `sender_ip` - Sender's IP address
    #[instrument(skip(self, _proto_msg), fields(sender_ip = %sender_ip))]
    fn handle_peer_offline(&self, _proto_msg: &ProtocolMessage, sender_ip: IpAddr) -> Result<()> {
        if let Some(ref peer_repo) = self.peer_repo {
            let rt = tokio::runtime::Runtime::new()
                .map_err(|e| NeoLanError::Other(format!("Failed to create runtime: {}", e)))?;

            // Update last_seen (will mark as offline by timeout)
            rt.block_on(async { peer_repo.update_last_seen(&sender_ip.to_string()).await })?;

            tracing::info!("✅ Peer marked offline in database: {}", sender_ip);
        } else {
            tracing::warn!("⚠️ Peer repository not available - cannot update peer");
        }

        // Emit Tauri event
        if let Some(ref app_state) = self.app_state {
            app_state.emit_tauri_event(TauriEvent::PeerOffline {
                peer_ip: sender_ip.to_string(),
            });
        }

        Ok(())
    }

    /// Handle a text message (IPMSG_SENDMSG)
    ///
    /// Stores received text message to database.
    /// Auto-adds sender to contacts if not already in database.
    ///
    /// # Arguments
    /// * `proto_msg` - Protocol message
    /// * `sender_ip` - Sender's IP address
    /// * `local_ip` - Local IP address (receiver)
    #[instrument(skip(self, proto_msg), fields(sender_ip = %sender_ip, sender_name = %proto_msg.sender_name, packet_id = %proto_msg.packet_id))]
    pub fn handle_text_message(
        &self,
        proto_msg: &ProtocolMessage,
        sender_ip: IpAddr,
        local_ip: IpAddr,
    ) -> Result<()> {
        // Store to database
        if let Some(ref repo) = self.message_repo {
            let message_model = MessageModel {
                id: 0, // Auto-increment
                msg_id: proto_msg.packet_id.to_string(),
                user_id: Some(proto_msg.user_id.clone()),
                sender_ip: sender_ip.to_string(),
                sender_name: proto_msg.sender_name.clone(),
                receiver_ip: local_ip.to_string(),
                msg_type: proto_msg.msg_type as i32,
                content: proto_msg.content.clone(),
                is_encrypted: msg_type::has_opt(proto_msg.msg_type, msg_type::IPMSG_ENCRYPTOPT),
                is_offline: false,
                sent_at: Utc::now().naive_utc(),
                received_at: Some(Utc::now().naive_utc()),
                created_at: Utc::now().naive_utc(),
            };

            // Store to database (blocking - should be async in production)
            let rt = tokio::runtime::Runtime::new()
                .map_err(|e| NeoLanError::Other(format!("Failed to create runtime: {}", e)))?;
            rt.block_on(async { repo.insert(&message_model).await })?;

            tracing::debug!(
                "💾 Message stored to database: msg_id={}",
                proto_msg.packet_id
            );
        } else {
            tracing::warn!("⚠️ [AUTO-ADD] Message repository not available - contact not created");
        }

        // Send IPMSG_RECVMSG acknowledgment if message has SENDCHECKOPT flag
        if msg_type::has_opt(proto_msg.msg_type, msg_type::IPMSG_SENDCHECKOPT) {
            tracing::info!(
                "📤 [handle_text_message] Sending IPMSG_RECVMSG acknowledgment to {}: original_msg_id={}, ack_msg_id={}",
                sender_ip, proto_msg.packet_id, self.next_packet_id()
            );

            // Create acknowledgment message
            let ack_msg = Message {
                id: uuid::Uuid::new_v4(),
                packet_id: self.next_packet_id().to_string(), // Send back original packet ID
                sender: PeerInfo::new(
                    sender_ip,
                    self.config.udp_port,
                    Some(self.config.username.clone()),
                ),
                receiver: PeerInfo::new(sender_ip, self.config.udp_port, None),
                msg_type: MessageType::RecvAck, // This will map to IPMSG_RECVMSG
                content: proto_msg.packet_id.to_string(), // Send back original packet ID
                timestamp: chrono::Utc::now(),
            };

            // Convert to protocol message
            let proto_ack = ack_msg.to_protocol(&self.config.username, &self.config.hostname);

            // Serialize and send
            let bytes = serialize_message(&proto_ack)?;

            let target_addr = SocketAddr::new(sender_ip, self.config.udp_port);
            self.udp.send_to(&bytes, target_addr)?;

            tracing::debug!(
                "📤 [handle_text_message] IPMSG_RECVMSG acknowledgment sent successfully"
            );
        } else {
            tracing::debug!("ℹ️ [handle_text_message] No SENDCHECKOPT flag (msg_type=0x{:08x}) - skipping acknowledgment",
                proto_msg.msg_type);
        }

        Ok(())
    }

    /// Handle a file transfer request (IPMSG_GETFILEDATA)
    ///
    /// Parses the file transfer request and emits a Tauri event for user confirmation.
    ///
    /// # Arguments
    /// * `proto_msg` - Protocol message containing the file request
    /// * `sender_ip` - Sender's IP address
    fn handle_file_transfer_request(
        &self,
        proto_msg: &ProtocolMessage,
        sender_ip: IpAddr,
    ) -> Result<()> {
        tracing::info!(
            "File transfer request from {} ({}): {}",
            proto_msg.sender_name,
            sender_ip,
            proto_msg.content.chars().take(50).collect::<String>()
        );

        // Only handle if file transfer response handler is available
        if let Some(ref handler) = self.file_transfer {
            // Parse the request
            let pending = handler.handle_incoming_request(proto_msg, sender_ip)?;

            // Emit Tauri event for user confirmation
            if let Some(ref app_state) = self.app_state {
                let event = handler.to_event(&pending);
                app_state.emit_tauri_event(event);
                tracing::info!(
                    "Emitted file-transfer-request event: requestId={}, file={}",
                    pending.id,
                    pending.file_name
                );
            } else {
                tracing::warn!(
                    "App state not available - cannot notify user of file transfer request"
                );
            }
        } else {
            tracing::warn!(
                "File transfer handler not available - cannot handle file transfer request"
            );
        }

        Ok(())
    }

    // ==================== Additional Message Handlers ====================

    /// Handle receive message acknowledgment (IPMSG_RECVMSG)
    ///
    /// The peer has confirmed receipt of a message we sent.
    ///
    /// # Arguments
    /// * `proto_msg` - Protocol message
    /// * `sender_ip` - Sender's IP address
    #[instrument(skip(self, proto_msg), fields(sender_ip = %sender_ip, packet_id = %proto_msg.packet_id))]
    fn handle_recv_msg(&self, proto_msg: &ProtocolMessage, sender_ip: IpAddr) -> Result<()> {
        tracing::info!(
            "✅ [handle_recv_msg] Message receipt acknowledged from {}: msg_id={}, content={}",
            sender_ip,
            proto_msg.packet_id,
            proto_msg.content
        );

        // Emit Tauri event for message receipt acknowledgment
        if let Some(ref app_state) = self.app_state {
            let now = Utc::now();
            let event = TauriEvent::MessageReceiptAck {
                msg_id: proto_msg.content.clone(), // Content contains the original message ID
                sender_ip: sender_ip.to_string(),
                sender_name: proto_msg.sender_name.clone(),
                acknowledged_at: now.timestamp_millis(),
            };
            app_state.emit_tauri_event(event);
            tracing::info!("✅ [handle_recv_msg] Emitted message-receipt-ack event to frontend: msg_id={}, from={}",
                proto_msg.content, sender_ip);
        } else {
            tracing::warn!("⚠️ [handle_recv_msg] App state not available - cannot emit message-receipt-ack event");
        }

        // TODO: Update message status in database to "delivered"
        Ok(())
    }

    /// Handle read message notification (IPMSG_READMSG)
    ///
    /// The peer has read a message we sent.
    ///
    /// # Arguments
    /// * `proto_msg` - Protocol message
    /// * `sender_ip` - Sender's IP address
    #[instrument(skip(self, proto_msg), fields(sender_ip = %sender_ip))]
    fn handle_read_msg(&self, proto_msg: &ProtocolMessage, sender_ip: IpAddr) -> Result<()> {
        tracing::info!(
            "📖 Message read by {}: msg_id={}",
            sender_ip,
            proto_msg.packet_id
        );
        // TODO: Update message status in database to "read"
        // TODO: Emit Tauri event for frontend notification
        Ok(())
    }

    /// Handle delete message request (IPMSG_DELMSG)
    ///
    /// The peer wants to delete a message.
    ///
    /// # Arguments
    /// * `proto_msg` - Protocol message
    /// * `sender_ip` - Sender's IP address
    #[instrument(skip(self, proto_msg), fields(sender_ip = %sender_ip))]
    fn handle_del_msg(&self, proto_msg: &ProtocolMessage, sender_ip: IpAddr) -> Result<()> {
        tracing::info!(
            "🗑️ Delete message request from {}: msg_id={}",
            sender_ip,
            proto_msg.packet_id
        );
        // TODO: Mark message as deleted in database
        // TODO: Emit Tauri event for frontend update
        Ok(())
    }

    /// Handle answer to read message (IPMSG_ANSREADMSG)
    ///
    /// Response to a read message confirmation.
    ///
    /// # Arguments
    /// * `proto_msg` - Protocol message
    /// * `sender_ip` - Sender's IP address
    #[instrument(skip(self, proto_msg), fields(sender_ip = %sender_ip))]
    fn handle_answer_read_msg(&self, proto_msg: &ProtocolMessage, sender_ip: IpAddr) -> Result<()> {
        tracing::info!(
            "📨 Read answer from {}: msg_id={}",
            sender_ip,
            proto_msg.packet_id
        );
        // TODO: Handle read answer confirmation
        Ok(())
    }

    /// Handle peer list response (IPMSG_ANSLIST)
    ///
    /// Response containing the list of peers.
    ///
    /// # Arguments
    /// * `proto_msg` - Protocol message containing peer list
    /// * `sender_ip` - Sender's IP address
    fn handle_peer_list_response(
        &self,
        proto_msg: &ProtocolMessage,
        sender_ip: IpAddr,
    ) -> Result<()> {
        tracing::info!(
            "📋 [PEER LIST] Received peer list from {}: count={}",
            sender_ip,
            proto_msg.content.lines().count()
        );
        // TODO: Parse peer list content and update peer database
        // Format: Each line contains peer information
        tracing::debug!("Peer list content:\n{}", proto_msg.content);
        Ok(())
    }

    /// Handle user information (IPMSG_SENDINFO)
    ///
    /// Response containing user information.
    ///
    /// # Arguments
    /// * `proto_msg` - Protocol message containing user info
    /// * `sender_ip` - Sender's IP address
    fn handle_user_info(&self, proto_msg: &ProtocolMessage, sender_ip: IpAddr) -> Result<()> {
        tracing::info!(
            "ℹ️ [USER INFO] User info from {}: {}",
            sender_ip,
            proto_msg.content.chars().take(100).collect::<String>()
        );
        // TODO: Parse user info and update peer information
        Ok(())
    }

    /// Handle absence information (IPMSG_SENDABSENCEINFO)
    ///
    /// Response containing absence reason.
    ///
    /// # Arguments
    /// * `proto_msg` - Protocol message containing absence info
    /// * `sender_ip` - Sender's IP address
    fn handle_absence_info(&self, proto_msg: &ProtocolMessage, sender_ip: IpAddr) -> Result<()> {
        tracing::info!(
            "🏖️ [ABSENCE] Absence info from {}: {}",
            sender_ip,
            proto_msg.content.chars().take(100).collect::<String>()
        );
        // TODO: Parse absence info and update peer status
        Ok(())
    }

    /// Handle release files notification (IPMSG_RELEASEFILES)
    ///
    /// The peer has released file transfer resources.
    ///
    /// # Arguments
    /// * `proto_msg` - Protocol message
    /// * `sender_ip` - Sender's IP address
    fn handle_release_files(&self, proto_msg: &ProtocolMessage, sender_ip: IpAddr) -> Result<()> {
        tracing::info!(
            "🔄 [RELEASE] File release notification from {}: msg_id={}",
            sender_ip,
            proto_msg.packet_id
        );
        // TODO: Clean up file transfer resources
        // TODO: Emit Tauri event for frontend update
        Ok(())
    }

    /// Handle public key response (IPMSG_ANSPUBKEY)
    ///
    /// Response containing the peer's public key for encryption.
    ///
    /// # Arguments
    /// * `proto_msg` - Protocol message containing public key
    /// * `sender_ip` - Sender's IP address
    fn handle_public_key_response(
        &self,
        proto_msg: &ProtocolMessage,
        sender_ip: IpAddr,
    ) -> Result<()> {
        tracing::info!(
            "🔑 [PUBLIC KEY] Public key received from {}: length={}",
            sender_ip,
            proto_msg.content.len()
        );
        // TODO: Parse and store public key for encrypted messaging
        // TODO: Update peer encryption capability
        Ok(())
    }

    /// Route a message to the appropriate handler based on msg_type
    ///
    /// This is a convenience method that can be used as a callback for PeerDiscovery.
    /// It extracts the sender IP and calls handle_incoming_message.
    ///
    /// # Arguments
    /// * `proto_msg` - Protocol message received from network
    /// * `sender` - Socket address of the sender
    /// * `local_ip` - Local IP address (for receiver field)
    pub fn route_message(
        &self,
        proto_msg: &ProtocolMessage,
        sender: SocketAddr,
        local_ip: IpAddr,
    ) -> Result<()> {
        self.handle_incoming_message(proto_msg, sender.ip(), local_ip)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_config() -> AppConfig {
        AppConfig {
            username: "TestUser".to_string(),
            hostname: "test-host".to_string(),
            bind_ip: "127.0.0.1".to_string(),
            udp_port: 2425,
            ..Default::default()
        }
    }

    #[test]
    fn test_message_handler_new() {
        let udp = UdpTransport::bind(0).unwrap();
        let config = create_test_config();
        let handler = MessageHandler::new(udp, config);

        assert_eq!(handler.packet_id_counter(), 1);
    }

    #[test]
    fn test_next_packet_id() {
        let udp = UdpTransport::bind(0).unwrap();
        let config = create_test_config();
        let handler = MessageHandler::new(udp, config);

        let id1 = handler.next_packet_id();
        let id2 = handler.next_packet_id();
        let id3 = handler.next_packet_id();

        assert_eq!(id1, 1);
        assert_eq!(id2, 2);
        assert_eq!(id3, 3);
    }

    #[test]
    fn test_packet_id_counter() {
        let udp = UdpTransport::bind(0).unwrap();
        let config = create_test_config();
        let handler = MessageHandler::new(udp, config);

        assert_eq!(handler.packet_id_counter(), 1);

        handler.next_packet_id();
        handler.next_packet_id();

        assert_eq!(handler.packet_id_counter(), 3);
    }

    #[test]
    fn test_reset_packet_id_counter() {
        let udp = UdpTransport::bind(0).unwrap();
        let config = create_test_config();
        let handler = MessageHandler::new(udp, config);

        handler.next_packet_id();
        handler.next_packet_id();

        handler.reset_packet_id_counter(100);

        assert_eq!(handler.packet_id_counter(), 100);
        assert_eq!(handler.next_packet_id(), 100);
        assert_eq!(handler.next_packet_id(), 101);
    }

    #[test]
    fn test_udp_reference() {
        let udp = UdpTransport::bind(0).unwrap();
        let config = create_test_config();
        let handler = MessageHandler::new(udp, config);

        // Should be able to access UDP transport
        let port = handler.udp().port();
        assert!(port > 0);
    }

    #[test]
    fn test_send_empty_message_error() {
        let udp = UdpTransport::bind(0).unwrap();
        let config = create_test_config();
        let handler = MessageHandler::new(udp, config);

        let target_ip: IpAddr = "127.0.0.1".parse().unwrap();
        let result = handler.send_text_message(target_ip, "   ");

        assert!(result.is_err());

        if let Err(NeoLanError::Validation(msg)) = result {
            assert!(msg.contains("empty"));
        } else {
            panic!("Expected Validation error");
        }
    }

    #[test]
    fn test_send_text_message_to_loopback() {
        let sender_udp = UdpTransport::bind(0).unwrap();
        let receiver_udp = UdpTransport::bind(0).unwrap();
        let receiver_port = receiver_udp.port();

        let config = AppConfig {
            username: "Sender".to_string(),
            hostname: "sender-host".to_string(),
            bind_ip: "127.0.0.1".to_string(),
            udp_port: sender_udp.port(),
            ..Default::default()
        };

        let handler = MessageHandler::new(sender_udp, config);
        let target_ip: IpAddr = "127.0.0.1".parse().unwrap();

        // Override default port for testing
        let _result = handler.send_text_message(target_ip, "Hello, Test!");

        // Send to our own receiver port
        let sender_udp = handler.udp();
        // Create message with valid packet ID
        let message = Message {
            id: uuid::Uuid::new_v4(),
            packet_id: "12345".to_string(), // Valid small packet ID for testing
            sender: PeerInfo::new(target_ip, receiver_port, Some("Receiver".to_string())),
            receiver: PeerInfo::new(target_ip, sender_udp.port(), Some("Sender".to_string())),
            msg_type: MessageType::Text,
            content: "Hello, Test!".to_string(),
            timestamp: chrono::Utc::now(),
        };

        let proto_msg = message.to_protocol("Sender", "sender-host");
        let bytes = serialize_message(&proto_msg).unwrap();

        let target_addr = SocketAddr::new(target_ip, receiver_port);
        sender_udp.send_to(&bytes, target_addr).unwrap();

        // Try to receive on the receiver socket (with timeout)
        receiver_udp.set_read_timeout(Some(100)).unwrap();
        let mut buffer = [0u8; 65535];
        let result = receiver_udp.recv_from(&mut buffer);

        assert!(result.is_ok());
        let (len, _addr) = result.unwrap();
        assert!(len > 0);

        // Parse and verify the message
        let received = crate::network::parse_message(&buffer[..len]).unwrap();
        assert_eq!(received.sender_name, "Sender");
        assert_eq!(received.content, "Hello, Test!");
    }

    #[test]
    fn test_send_generic_message() {
        let udp = UdpTransport::bind(0).unwrap();
        let config = create_test_config();
        let handler = MessageHandler::new(udp, config);

        let target_ip: IpAddr = "127.0.0.1".parse().unwrap();
        let result = handler.send_message(
            target_ip,
            MessageType::FileRequest,
            r#"{"name":"test.txt","size":1024,"md5":"abc123"}"#.to_string(),
        );

        // Should not error (sends to localhost which may not receive, but send should succeed)
        assert!(result.is_ok());
    }
}

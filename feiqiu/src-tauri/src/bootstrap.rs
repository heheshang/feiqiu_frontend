//! Application bootstrap and initialization module
//!
//! This module handles the initialization sequence for the NeoLan application:
//! 1. Database connection and migrations
//! 2. Configuration loading
//! 3. Network transport setup
//! 4. Peer discovery and message handling

use crate::config::app::{AppConfig, ConfigRepository};
use crate::migration::{Migrator, MigratorTrait};
use crate::modules::message::handler::MessageHandler;
use crate::modules::peer::manager::MessageRouteRequest;
use crate::modules::peer::{discovery::PeerDiscovery, PeerManager};
use crate::network::UdpTransport;
use crate::state::app_state::TauriEvent;
use crate::storage::database::establish_connection;
use crate::AppState;
use sea_orm::DatabaseConnection;
use std::sync::mpsc;
use std::sync::Arc;
use std::thread;
use tauri::{App, Emitter};

/// Bootstrap result containing initialized AppState and event receiver
pub struct BootstrapResult {
    pub app_state: AppState,
    pub event_rx: mpsc::Receiver<TauriEvent>,
}

/// Initialize database connection and run migrations
pub fn init_database() -> DatabaseConnection {
    tracing::info!("Establishing database connection...");
    let db = match tauri::async_runtime::block_on(establish_connection()) {
        Ok(db) => {
            tracing::info!("Database connection established");
            db
        }
        Err(e) => {
            tracing::error!("Failed to establish database connection: {}", e);
            panic!("Database connection failed: {}", e);
        }
    };

    tracing::info!("Running database migrations...");
    if let Err(e) = tauri::async_runtime::block_on(async { Migrator::up(&db, None).await }) {
        tracing::error!("Database migration failed: {:?}", e);
        panic!("Database migration failed: {:?}", e);
    }
    tracing::info!("Database migrations completed");

    db
}

/// Load application configuration from database
pub fn load_config(db: &DatabaseConnection) -> AppConfig {
    tracing::info!("Loading configuration from database...");
    let config_repo = ConfigRepository::new(db.clone());
    match tauri::async_runtime::block_on(config_repo.load_app_config()) {
        Ok(cfg) => {
            tracing::info!("Configuration loaded successfully from database");
            cfg
        }
        Err(e) => {
            tracing::warn!("Failed to load config from database: {}, using defaults", e);
            AppConfig::default()
        }
    }
}

/// Bootstrap the application - initialize all core components
pub fn bootstrap() -> BootstrapResult {
    // Step 1: Database
    let db = init_database();

    // Step 2: Configuration
    let config = load_config(&db);

    // Step 3: Create AppState
    let app_state = AppState::new(config);
    app_state.set_database(&db);

    // Step 4: Event channel
    let (event_tx, event_rx) = mpsc::channel::<TauriEvent>();
    app_state.set_event_sender(event_tx);

    BootstrapResult {
        app_state,
        event_rx,
    }
}

/// Spawn the event listener task that forwards events to the frontend
pub fn spawn_event_listener(app_handle: tauri::AppHandle, event_rx: mpsc::Receiver<TauriEvent>) {
    tauri::async_runtime::spawn(async move {
        tracing::info!("🎧 [EVENT TASK] Event listener task started");
        for event in event_rx {
            emit_event(&app_handle, &event);
        }
        tracing::info!("Event listener task ended");
    });
}

/// Emit a single event to the frontend
fn emit_event(app_handle: &tauri::AppHandle, event: &TauriEvent) {
    let (event_name, result) = match event {
        TauriEvent::MessageReceived {
            msg_id,
            sender_name,
            sender_ip,
            content,
            ..
        } => {
            tracing::info!(
                "📤 [TAURI EMIT] message-received: msg_id={}, from={}, ip={}, content={}",
                msg_id,
                sender_name,
                sender_ip,
                content.chars().take(50).collect::<String>()
            );
            (
                "message-received",
                app_handle.emit("message-received", event),
            )
        }
        TauriEvent::PeerOnline { .. } => {
            tracing::info!("👤 [TAURI EMIT] peer-online: {:?}", event);
            ("peer-online", app_handle.emit("peer-online", event))
        }
        TauriEvent::PeerOffline { .. } => ("peer-offline", app_handle.emit("peer-offline", event)),
        TauriEvent::FileTransferRequest { .. } => (
            "file-transfer-request",
            app_handle.emit("file-transfer-request", event),
        ),
        TauriEvent::PeersDiscovered { .. } => (
            "peers-discovered",
            app_handle.emit("peers-discovered", event),
        ),
        TauriEvent::MessageReceiptAck {
            msg_id,
            sender_ip,
            sender_name,
            ..
        } => {
            tracing::info!(
                "📤 [TAURI EMIT] message-receipt-ack: msg_id={}, from={}, ip={}",
                msg_id,
                sender_name,
                sender_ip
            );
            (
                "message-receipt-ack",
                app_handle.emit("message-receipt-ack", event),
            )
        }
        TauriEvent::MessageDelivered { msg_id, .. } => {
            tracing::info!("📤 [TAURI EMIT] message-delivered: msg_id={}", msg_id);
            (
                "message-delivered",
                app_handle.emit("message-delivered", event),
            )
        }
        TauriEvent::MessageRead { msg_id, .. } => {
            tracing::info!("📤 [TAURI EMIT] message-read: msg_id={}", msg_id);
            ("message-read", app_handle.emit("message-read", event))
        }
        TauriEvent::MessageDeleted { msg_id, .. } => {
            tracing::info!("📤 [TAURI EMIT] message-deleted: msg_id={}", msg_id);
            ("message-deleted", app_handle.emit("message-deleted", event))
        }
        TauriEvent::FileTransferRejected { request_id, .. } => {
            tracing::info!(
                "📤 [TAURI EMIT] file-transfer-rejected: request_id={}",
                request_id
            );
            (
                "file-transfer-rejected",
                app_handle.emit("file-transfer-rejected", event),
            )
        }
        TauriEvent::ConversationCreated { conversation_id, .. } => {
            tracing::info!("📤 [TAURI EMIT] conversation-created: conversation_id={}", conversation_id);
            (
                "conversation-created",
                app_handle.emit("conversation-created", event),
            )
        }
        TauriEvent::ConversationUpdated { conversation_id, .. } => {
            tracing::info!("📤 [TAURI EMIT] conversation-updated: conversation_id={}", conversation_id);
            (
                "conversation-updated",
                app_handle.emit("conversation-updated", event),
            )
        }
    };

    if let Err(e) = result {
        tracing::error!("❌ Failed to emit {} event: {}", event_name, e);
    }
}

/// Initialize network components (UDP transports, PeerManager, MessageHandler)
pub fn init_network(
    _app: &App,
    app_state: &AppState,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    tracing::info!("Initializing network components...");

    let config = app_state.get_config();
    let udp_port = config.udp_port;

    // Log UDP port source
    use crate::config::AppConfig;
    if udp_port == AppConfig::DEFAULT_UDP_PORT {
        tracing::info!("Using default UDP port: {}", udp_port);
    } else {
        tracing::info!("Using configured UDP port from database: {}", udp_port);
    }

    // Bind UDP transports
    let udp_recv = bind_udp_receive(udp_port)?;
    let udp_send = bind_udp_send()?;

    // Log TCP port range
    let tcp_port_count = config.tcp_port_end - config.tcp_port_start + 1;
    tracing::info!(
        "TCP port range: {}-{} ({} ports)",
        config.tcp_port_start,
        config.tcp_port_end,
        tcp_port_count
    );

    // Create message routing channel
    let (message_route_tx, message_route_rx) = mpsc::channel::<MessageRouteRequest>();

    // Initialize MessageHandler
    init_message_handler(app_state, udp_send, &config);

    // Spawn message routing task
    start_message_router(app_state.clone(), message_route_rx, &config);

    // Initialize PeerManager
    init_peer_manager(app_state, udp_recv, message_route_tx);

    // Start peer discovery
    start_peer_manager(app_state.clone());

    Ok(())
}

fn bind_udp_receive(port: u16) -> Result<UdpTransport, Box<dyn std::error::Error + Send + Sync>> {
    match UdpTransport::bind_with_retry(port, 10) {
        Ok(u) => {
            let actual_port = u.port();
            if actual_port != port {
                tracing::warn!(
                    "Requested UDP port {} unavailable, using port {}",
                    port,
                    actual_port
                );
            }
            tracing::info!("UDP receive transport bound to port {}", actual_port);
            Ok(u)
        }
        Err(e) => {
            tracing::error!("Failed to bind UDP transport after retries: {}", e);
            Err(Box::new(e))
        }
    }
}

fn bind_udp_send() -> Result<UdpTransport, Box<dyn std::error::Error + Send + Sync>> {
    match UdpTransport::bind(0) {
        Ok(u) => {
            tracing::info!("UDP send transport bound to port {}", u.port());
            Ok(u)
        }
        Err(e) => {
            tracing::error!("Failed to bind UDP send transport: {}", e);
            Err(Box::new(e))
        }
    }
}

fn init_message_handler(app_state: &AppState, udp_send: UdpTransport, config: &AppConfig) {
    tracing::info!("Initializing MessageHandler...");
    let app_state_arc = Arc::new(app_state.clone());

    // Get repositories from AppState
    let peer_repo = app_state.get_peer_repo();
    let contact_repo = app_state.get_contact_repo();
    let conversation_repo = app_state.get_conversation_repo();

    let mut handler = MessageHandler::new(udp_send, config.clone()).with_app_state(app_state_arc);

    // Inject peer repository (already wrapped in Arc)
    if let Some(repo) = peer_repo {
        handler = handler.with_peer_repo(repo);
        tracing::info!("Peer repository injected into MessageHandler");
    } else {
        tracing::warn!(
            "⚠️ Peer repository not available - peer discovery will not persist to database"
        );
    }

    // Inject contact repository
    if let Some(repo) = contact_repo {
        handler = handler.with_contact_repo(Arc::new(repo));
        tracing::info!("Contact repository injected into MessageHandler");
    } else {
        tracing::warn!("⚠️ Contact repository not available - contacts will not be auto-created");
    }

    // Inject conversation repository
    if let Some(repo) = conversation_repo {
        handler = handler.with_conversation_repo(Arc::new(repo));
        tracing::info!("Conversation repository injected into MessageHandler");
    } else {
        tracing::warn!("⚠️ Conversation repository not available - conversations will not be auto-created");
    }

    app_state.init_message_handler(handler);
    tracing::info!("MessageHandler initialized");
}

fn start_message_router(
    app_state: AppState,
    message_route_rx: mpsc::Receiver<MessageRouteRequest>,
    config: &AppConfig,
) {
    let local_ip = config
        .bind_ip
        .parse()
        .unwrap_or_else(|_| std::net::IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1)));

    thread::spawn(move || {
        tracing::info!("Message router task started");
        let rt = tokio::runtime::Runtime::new().expect("Failed to create tokio runtime");
        for route_request in message_route_rx {
            let sender_ip = route_request.sender.ip();
            if let Err(e) = rt.block_on(async {
                app_state
                    .handle_routed_message(&route_request.message, sender_ip, local_ip)
                    .await
            }) {
                tracing::error!("Failed to handle incoming message: {:?}", e);
            }
        }
        tracing::info!("Message router task ended");
    });
}

fn init_peer_manager(
    app_state: &AppState,
    udp_recv: UdpTransport,
    message_route_tx: mpsc::Sender<MessageRouteRequest>,
) {
    tracing::info!("Initializing PeerManager...");

    // Get config to access user_id
    let config = app_state.get_config();

    let discovery = PeerDiscovery::with_defaults(udp_recv, config.user_id.clone());
    tracing::info!(
        "PeerDiscovery created: {}@{} (user_id: {})",
        discovery.username(),
        discovery.hostname(),
        discovery.user_id()
    );

    let peer_repo = match app_state.get_peer_repo() {
        Some(repo) => repo,
        None => {
            tracing::error!(
                "Peer repository not initialized. Ensure init_database() is called before init_peer_manager."
            );
            return;
        }
    };

    let peer_manager = PeerManager::new(discovery, peer_repo);
    peer_manager.set_message_handler_channel(message_route_tx);
    app_state.init_peer_manager(peer_manager);
    tracing::info!("PeerManager initialized");
}

fn start_peer_manager(app_state: AppState) {
    thread::spawn(move || {
        tracing::info!("PeerManager thread started");
        if let Err(e) = app_state.start_peer_manager() {
            tracing::error!("PeerManager failed: {}", e);
        }
        tracing::info!("PeerManager thread ended");
    });
}

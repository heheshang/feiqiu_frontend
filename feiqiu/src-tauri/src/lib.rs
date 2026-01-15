//! NeoLan - Local Area Network Communication Application
//!
//! This is the main library entry point for the Tauri backend.

// Module declarations
mod bootstrap;
mod commands;
mod config;
mod error;
mod migration;
mod modules;
pub mod network;
pub mod state;
mod storage;
pub mod utils;

// Re-export commonly used types
pub use error::{NeoLanError, Result};
pub use state::app_state::{PeerDiscoveredDto, TauriEvent};
pub use state::AppState;

// Import Tauri commands
use commands::config::{get_config, get_config_value, reset_config, set_config, set_config_value};
use commands::contacts::{
    add_contacts_to_group, create_contact, create_contact_group, delete_contact,
    delete_contact_group, get_contact, get_contact_groups, get_contact_stats, get_contacts,
    remove_contacts_from_group, search_contacts, update_contact, update_contact_group,
};
use commands::events::poll_events;
use commands::file_transfer::{
    accept_file_transfer, cancel_file_transfer, get_file_transfers, reject_file_transfer,
};
use commands::message::{get_messages, send_message, send_text_message};
use commands::peer::{
    get_network_status, get_online_peers, get_peer_by_ip, get_peer_stats, get_peers,
};
use commands::system::get_system_info;

/// Application entry point
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Initialize logging
    utils::logger::init_logger();
    tracing::info!("NeoLan starting...");

    // Bootstrap application (database, config, app state)
    let bootstrap::BootstrapResult {
        app_state,
        event_rx,
    } = bootstrap::bootstrap();
    let app_state_for_setup = app_state.clone();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .setup(move |app| {
            // Start event listener
            bootstrap::spawn_event_listener(app.handle().clone(), event_rx);

            // Initialize network components
            bootstrap::init_network(app, &app_state_for_setup)
                .map_err(|e| e as Box<dyn std::error::Error>)?;

            Ok(())
        })
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            // System
            get_system_info,
            // Peers
            get_peers,
            get_online_peers,
            get_peer_by_ip,
            get_peer_stats,
            get_network_status,
            // Config
            get_config,
            set_config,
            reset_config,
            get_config_value,
            set_config_value,
            // Events
            poll_events,
            // Messages
            send_message,
            send_text_message,
            get_messages,
            // File transfers
            accept_file_transfer,
            reject_file_transfer,
            get_file_transfers,
            cancel_file_transfer,
            // Contacts
            get_contacts,
            get_contact,
            create_contact,
            update_contact,
            delete_contact,
            get_contact_groups,
            create_contact_group,
            update_contact_group,
            delete_contact_group,
            add_contacts_to_group,
            remove_contacts_from_group,
            search_contacts,
            get_contact_stats,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

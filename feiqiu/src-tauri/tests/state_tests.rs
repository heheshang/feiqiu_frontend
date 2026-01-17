//! State tests

use feiqiu::config::AppConfig;
use feiqiu::state::app_state::AppState;
use feiqiu::state::events::{AppEvent, AppEventEmitter};
use std::net::IpAddr;

// ============== AppState tests ==============

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

// ============== Events tests ==============

#[test]
fn test_event_creation() {
    let ip: IpAddr = "192.168.1.100".parse().unwrap();

    let event = AppEvent::peer_online(ip, 2425, Some("Alice".to_string()));
    match event {
        AppEvent::PeerOnline {
            ip: e_ip,
            port,
            username,
        } => {
            assert_eq!(e_ip, "192.168.1.100");
            assert_eq!(port, 2425);
            assert_eq!(username, Some("Alice".to_string()));
        }
        _ => panic!("Expected PeerOnline event"),
    }
}

#[test]
fn test_event_emitter() {
    let mut emitter = AppEventEmitter::new();

    assert_eq!(emitter.pending_count(), 0);

    emitter.emit(AppEvent::ConfigChanged);
    emitter.emit(AppEvent::Initialized);

    assert_eq!(emitter.pending_count(), 2);

    let events = emitter.drain();
    assert_eq!(events.len(), 2);
    assert_eq!(emitter.pending_count(), 0);
}

#[test]
fn test_peer_offline_event() {
    let ip: IpAddr = "192.168.1.100".parse().unwrap();
    let event = AppEvent::peer_offline(ip);

    match event {
        AppEvent::PeerOffline { ip } => {
            assert_eq!(ip, "192.168.1.100");
        }
        _ => panic!("Expected PeerOffline event"),
    }
}

#[test]
fn test_error_event() {
    let event = AppEvent::error("Test error".to_string());

    match event {
        AppEvent::Error { message } => {
            assert_eq!(message, "Test error");
        }
        _ => panic!("Expected Error event"),
    }
}

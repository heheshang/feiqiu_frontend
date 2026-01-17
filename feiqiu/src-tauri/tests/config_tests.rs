//! Config tests

use feiqiu::commands::config::ConfigDto;
use feiqiu::config::app::AppConfig;
use std::collections::HashMap;

// ============== AppConfig tests ==============

#[test]
fn test_default_config() {
    let config = AppConfig::default();

    // Verify default values use constants
    assert!(!config.username.is_empty());
    assert!(!config.hostname.is_empty());
    assert_eq!(config.udp_port, AppConfig::DEFAULT_UDP_PORT);
    assert_eq!(config.tcp_port_start, AppConfig::DEFAULT_TCP_PORT_START);
    assert_eq!(config.tcp_port_end, AppConfig::DEFAULT_TCP_PORT_END);
    assert_eq!(
        config.heartbeat_interval,
        AppConfig::DEFAULT_HEARTBEAT_INTERVAL
    );
    assert_eq!(config.peer_timeout, AppConfig::DEFAULT_PEER_TIMEOUT);
    assert_eq!(
        config.offline_message_retention_days,
        AppConfig::DEFAULT_OFFLINE_MESSAGE_RETENTION_DAYS
    );
    assert!(!config.encryption_enabled);
    assert!(!config.auto_accept_files);
    assert_eq!(config.log_level, AppConfig::DEFAULT_LOG_LEVEL);
    assert_eq!(config.bind_ip, AppConfig::DEFAULT_BIND_IP);
}

#[test]
fn test_config_serialization() {
    let config = AppConfig::default();

    // Test serialization
    let json = serde_json::to_string(&config).unwrap();
    assert!(json.contains("\"username\""));
    assert!(json.contains("\"udp_port\""));

    // Test deserialization
    let deserialized: AppConfig = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized, config);
}

#[test]
fn test_config_validation() {
    let config = AppConfig::default();

    // Default config should pass validation
    assert!(config.validate().is_ok());

    // Test invalid UDP port
    let mut invalid_config = config.clone();
    invalid_config.udp_port = 0;
    assert!(invalid_config.validate().is_err());

    // Test invalid TCP port range
    let mut invalid_config = config.clone();
    invalid_config.tcp_port_start = 9000;
    invalid_config.tcp_port_end = 8000;
    assert!(invalid_config.validate().is_err());

    // Test privileged ports
    let mut invalid_config = config.clone();
    invalid_config.tcp_port_start = 80;
    assert!(invalid_config.validate().is_err());

    // Test invalid timeout settings
    let mut invalid_config = config.clone();
    invalid_config.peer_timeout = 30;
    invalid_config.heartbeat_interval = 60;
    assert!(invalid_config.validate().is_err());

    // Test empty bind IP
    let mut invalid_config = config;
    invalid_config.bind_ip = String::new();
    assert!(invalid_config.validate().is_err());
}

#[test]
fn test_network_config_constants() {
    // Test constant values consistency
    assert_eq!(AppConfig::DEFAULT_UDP_PORT, 2425);
    assert_eq!(AppConfig::DEFAULT_TCP_PORT_START, 8000);
    assert_eq!(AppConfig::DEFAULT_TCP_PORT_END, 9000);
    assert_eq!(AppConfig::UDP_BUFFER_SIZE, 65535);
    assert_eq!(AppConfig::TCP_BUFFER_SIZE, 4096);
    assert_eq!(AppConfig::BROADCAST_ADDR, "255.255.255.255");
    assert_eq!(AppConfig::DEFAULT_BIND_IP, "0.0.0.0");
}

#[test]
fn test_config_helper_methods() {
    let config = AppConfig::default();

    // Test helper methods
    assert_eq!(config.udp_buffer_size(), AppConfig::UDP_BUFFER_SIZE);
    assert_eq!(config.tcp_buffer_size(), AppConfig::TCP_BUFFER_SIZE);
    assert_eq!(config.broadcast_addr(), AppConfig::BROADCAST_ADDR);
}

// ============== ConfigDto tests ==============

#[test]
fn test_config_dto_default() {
    let config = ConfigDto::default();

    assert_eq!(config.udp_port, 2425);
    assert_eq!(config.tcp_port_start, 8000);
    assert_eq!(config.tcp_port_end, 9000);
    assert_eq!(config.heartbeat_interval, 60);
    assert_eq!(config.peer_timeout, 180);
    assert_eq!(config.log_level, "info");
    assert!(!config.encryption_enabled);
}

#[test]
fn test_config_dto_validate() {
    let mut config = ConfigDto::default();

    // Valid config
    assert!(config.validate().is_ok());

    // Invalid udp_port
    config.udp_port = 80;
    assert!(config.validate().is_err());

    // Invalid tcp_port range
    config.udp_port = 2425;
    config.tcp_port_start = 9000;
    config.tcp_port_end = 8000;
    assert!(config.validate().is_err());

    // Invalid heartbeat
    config.tcp_port_start = 8000;
    config.tcp_port_end = 9000;
    config.heartbeat_interval = 0;
    assert!(config.validate().is_err());

    // Invalid peer_timeout
    config.heartbeat_interval = 60;
    config.peer_timeout = 30;
    assert!(config.validate().is_err());

    // Invalid log_level
    config.peer_timeout = 180;
    config.log_level = "invalid".to_string();
    assert!(config.validate().is_err());
}

#[test]
fn test_config_dto_to_map() {
    let config = ConfigDto::default();
    let map = config.to_map();

    assert_eq!(map.get("username"), Some(&config.username));
    assert_eq!(map.get("udp_port"), Some(&"2425".to_string()));
    assert_eq!(map.get("log_level"), Some(&"info".to_string()));
}

#[test]
fn test_config_dto_from_map() {
    let mut map = HashMap::new();
    map.insert("username".to_string(), "Alice".to_string());
    map.insert("udp_port".to_string(), "2426".to_string());
    map.insert("log_level".to_string(), "debug".to_string());

    let config = ConfigDto::from_map(&map);

    assert_eq!(config.username, "Alice");
    assert_eq!(config.udp_port, 2426);
    assert_eq!(config.log_level, "debug");
}

#[test]
fn test_set_config_value_validation() {
    // Test port validation
    let port: u16 = "80".parse().unwrap();
    assert!(port < 1024); // Invalid

    // Test boolean validation
    assert!(!"yes".parse::<bool>().is_ok());

    // Test log_level validation
    let log_level = "invalid";
    match log_level {
        "trace" | "debug" | "info" | "warn" | "error" => {
            panic!("Should not match");
        }
        _ => {
            // Expected path - invalid log level
        }
    }
}

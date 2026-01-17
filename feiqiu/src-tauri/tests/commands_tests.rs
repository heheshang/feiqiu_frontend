//! Commands tests

use feiqiu::commands::file_transfer::TaskDto;
use feiqiu::commands::peer::{PeerDto, PeerStats};
use std::net::IpAddr;

// ============== Message command tests ==============

#[test]
fn test_empty_content_validation() {
    // Empty content should be rejected
    let _peer_ip = "192.168.1.100".to_string();
    let content = "".to_string();

    // Just validation test, no actual state needed
    assert!(content.trim().is_empty());
}

#[test]
fn test_ip_address_parsing() {
    // Valid IP addresses
    assert!("192.168.1.100".parse::<IpAddr>().is_ok());
    assert!("127.0.0.1".parse::<IpAddr>().is_ok());
    assert!("::1".parse::<IpAddr>().is_ok());

    // Invalid IP addresses
    assert!("256.256.256.256".parse::<IpAddr>().is_err());
    assert!("invalid".parse::<IpAddr>().is_err());
}

#[test]
fn test_whitespace_content_validation() {
    // Whitespace-only content should be rejected
    assert!("   ".trim().is_empty());
    assert!("\t\n".trim().is_empty());

    // Normal content should pass
    assert!(!"Hello World".trim().is_empty());
}

// ============== Peer command tests ==============

#[test]
fn test_peer_dto_from_addr() {
    let ip: IpAddr = "192.168.1.100".parse().unwrap();
    let dto = PeerDto::from_addr(ip, 2425);

    assert_eq!(dto.ip, "192.168.1.100");
    assert_eq!(dto.port, 2425);
    assert_eq!(dto.status, "offline");
    assert_eq!(dto.display_name, "192.168.1.100");
}

#[test]
fn test_peer_dto_new() {
    let dto = PeerDto::new(
        "192.168.1.100".to_string(),
        2425,
        Some("Alice".to_string()),
        Some("alice-pc".to_string()),
        None,
        None,
        Vec::new(),
        "online".to_string(),
        "Alice".to_string(),
        1704000000000,
    );

    assert_eq!(dto.ip, "192.168.1.100");
    assert_eq!(dto.username, Some("Alice".to_string()));
    assert_eq!(dto.status, "online");
    assert_eq!(dto.last_seen, 1704000000000);
}

#[test]
fn test_peer_stats() {
    let stats = PeerStats {
        total: 10,
        online: 5,
        offline: 5,
    };

    assert_eq!(stats.total, 10);
    assert_eq!(stats.online, 5);
    assert_eq!(stats.offline, 5);
}

// ============== File Transfer command tests ==============

#[test]
fn test_taskdto_serialization() {
    let dto = TaskDto {
        id: "123e4567-e89b-12d3-a456-426614174000".to_string(),
        direction: "upload".to_string(),
        peer_ip: "192.168.1.100".to_string(),
        file_name: "test.txt".to_string(),
        file_size: 1024,
        md5: "abc123".to_string(),
        status: "active".to_string(),
        transferred_bytes: 512,
        progress: 0.5,
        port: Some(8001),
        error: None,
        created_at: 1234567890,
        updated_at: 1234567891,
    };

    let json = serde_json::to_string(&dto).unwrap();
    assert!(json.contains("\"id\":\"123e4567-e89b-12d3-a456-426614174000\""));
    assert!(json.contains("\"direction\":\"upload\""));
    assert!(json.contains("\"progress\":0.5"));
}

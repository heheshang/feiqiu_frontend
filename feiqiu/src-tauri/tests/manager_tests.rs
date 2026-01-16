//! Integration tests for PeerManager
//!
//! This module contains tests for the peer manager layer.

use chrono::{Duration, Utc};

/// Test the conversion from PeerModel to PeerNode
#[test]
fn test_peer_node_from_model() {
    let model = feiqiu::storage::entities::peers::Model {
        id: 1,
        ip: "192.168.1.100".to_string(),
        port: 2425,
        username: Some("Alice".to_string()),
        hostname: Some("alice-pc".to_string()),
        nickname: None,
        avatar: None,
        groups: None,
        last_seen: Utc::now().naive_utc(),
        created_at: Utc::now().naive_utc(),
        updated_at: Some(Utc::now().naive_utc()),
    };

    let node = feiqiu::modules::peer::types::PeerNode::from(&model);

    assert_eq!(node.ip.to_string(), "192.168.1.100");
    assert_eq!(node.port, 2425);
    assert_eq!(node.username, Some("Alice".to_string()));
    assert_eq!(node.hostname, Some("alice-pc".to_string()));
    assert_eq!(
        node.status,
        feiqiu::modules::peer::types::PeerStatus::Online
    );
}

/// Test peer online status calculation from last_seen timestamp
#[test]
fn test_peer_node_online_status_from_last_seen() {
    let recent = Utc::now().naive_utc();
    let old = Utc::now().naive_utc() - Duration::seconds(200);

    assert!(feiqiu::modules::peer::types::PeerNode::is_online_from_last_seen(recent));
    assert!(!feiqiu::modules::peer::types::PeerNode::is_online_from_last_seen(old));
}

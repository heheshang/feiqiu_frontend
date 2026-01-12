// Peer types - node status and information structures
//
// This module defines the core types for peer management:
// - PeerNode: Complete in-memory peer state
// - PeerStatus: Online/Offline/Away status
// - PeerInfo: Lightweight peer info for messages

use serde::{Deserialize, Serialize};
use std::net::IpAddr;
use std::time::SystemTime;

/// Peer node runtime state (in-memory representation)
///
/// This represents a peer's complete state in memory.
/// For database persistence, see `PeerModel` in storage entities.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PeerNode {
    /// IP address
    pub ip: IpAddr,

    /// UDP port
    pub port: u16,

    /// Username (display name)
    pub username: Option<String>,

    /// Hostname
    pub hostname: Option<String>,

    /// Nickname (user-set display name, takes precedence over username)
    pub nickname: Option<String>,

    /// Avatar (base64 or URL)
    pub avatar: Option<String>,

    /// Groups this peer belongs to
    pub groups: Vec<String>,

    /// Current status
    pub status: PeerStatus,

    /// Last seen timestamp
    pub last_seen: SystemTime,
}

impl PeerNode {
    /// Create a new peer node with minimal information
    pub fn new(ip: IpAddr, port: u16) -> Self {
        Self {
            ip,
            port,
            username: None,
            hostname: None,
            nickname: None,
            avatar: None,
            groups: Vec::new(),
            status: PeerStatus::Online,
            last_seen: SystemTime::now(),
        }
    }

    /// Create a new peer node with all information
    pub fn with_details(
        ip: IpAddr,
        port: u16,
        username: Option<String>,
        hostname: Option<String>,
    ) -> Self {
        Self {
            ip,
            port,
            username,
            hostname,
            nickname: None,
            avatar: None,
            groups: Vec::new(),
            status: PeerStatus::Online,
            last_seen: SystemTime::now(),
        }
    }

    /// Get display name (nickname > username > hostname)
    pub fn display_name(&self) -> String {
        self.nickname
            .as_ref()
            .or(self.username.as_ref())
            .or(self.hostname.as_ref())
            .cloned()
            .unwrap_or_else(|| self.ip.to_string())
    }

    /// Check if peer is online
    pub fn is_online(&self) -> bool {
        self.status == PeerStatus::Online
    }

    /// Update last seen timestamp
    pub fn update_last_seen(&mut self) {
        self.last_seen = SystemTime::now();
    }

    /// Mark as offline
    pub fn mark_offline(&mut self) {
        self.status = PeerStatus::Offline;
    }

    /// Mark as online
    pub fn mark_online(&mut self) {
        self.status = PeerStatus::Online;
        self.last_seen = SystemTime::now();
    }
}

/// Peer status
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PeerStatus {
    /// Peer is online and active
    Online,
    /// Peer is offline
    Offline,
    /// Peer is away (idle)
    Away,
}

impl PeerStatus {
    /// Check if status is online
    pub fn is_online(&self) -> bool {
        matches!(self, Self::Online)
    }

    /// Get status as string
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Online => "online",
            Self::Offline => "offline",
            Self::Away => "away",
        }
    }

    /// Parse status from string
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "online" => Some(Self::Online),
            "offline" => Some(Self::Offline),
            "away" => Some(Self::Away),
            _ => None,
        }
    }
}

/// Lightweight peer information (for message passing)
///
/// This is a minimal representation used for:
/// - Network messages
/// - API responses
/// - Quick peer lookups
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PeerInfo {
    /// IP address
    pub ip: IpAddr,

    /// Port
    pub port: u16,

    /// Username
    pub username: Option<String>,
}

impl PeerInfo {
    /// Create new peer info
    pub fn new(ip: IpAddr, port: u16, username: Option<String>) -> Self {
        Self { ip, port, username }
    }

    /// Create from PeerNode (kept for test purposes)
    #[allow(dead_code)]
    pub fn from_node(node: &PeerNode) -> Self {
        Self {
            ip: node.ip,
            port: node.port,
            username: node.username.clone(),
        }
    }

    /// Get socket address (kept for future use)
    #[allow(dead_code)]
    pub fn socket_addr(&self) -> std::net::SocketAddr {
        std::net::SocketAddr::new(self.ip, self.port)
    }
}

/// Peer event (for state change notifications)
#[derive(Clone, Debug, Serialize, Deserialize)]
#[allow(dead_code)]
pub enum PeerEvent {
    /// Peer came online
    Online(PeerInfo),

    /// Peer went offline
    Offline(IpAddr),

    /// Peer status changed
    StatusChanged(IpAddr, PeerStatus),

    /// Peer information updated
    Updated(PeerInfo),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_peer_node_new() {
        let ip = "192.168.1.100".parse().unwrap();
        let node = PeerNode::new(ip, 2425);

        assert_eq!(node.ip, ip);
        assert_eq!(node.port, 2425);
        assert!(node.username.is_none());
        assert!(node.hostname.is_none());
        assert!(node.is_online());
    }

    #[test]
    fn test_peer_node_with_details() {
        let ip = "192.168.1.100".parse().unwrap();
        let node = PeerNode::with_details(
            ip,
            2425,
            Some("Alice".to_string()),
            Some("alice-pc".to_string()),
        );

        assert_eq!(node.ip, ip);
        assert_eq!(node.username, Some("Alice".to_string()));
        assert_eq!(node.hostname, Some("alice-pc".to_string()));
    }

    #[test]
    fn test_display_name() {
        let ip = "192.168.1.100".parse().unwrap();
        let mut node = PeerNode::new(ip, 2425);

        // With only IP
        assert_eq!(node.display_name(), "192.168.1.100");

        // With hostname
        node.hostname = Some("alice-pc".to_string());
        assert_eq!(node.display_name(), "alice-pc");

        // With username (takes precedence)
        node.username = Some("Alice".to_string());
        assert_eq!(node.display_name(), "Alice");

        // With nickname (highest precedence)
        node.nickname = Some("Awesome Alice".to_string());
        assert_eq!(node.display_name(), "Awesome Alice");
    }

    #[test]
    fn test_peer_status() {
        assert!(PeerStatus::Online.is_online());
        assert!(!PeerStatus::Offline.is_online());
        assert!(!PeerStatus::Away.is_online());

        assert_eq!(PeerStatus::Online.as_str(), "online");
        assert_eq!(PeerStatus::Offline.as_str(), "offline");
        assert_eq!(PeerStatus::Away.as_str(), "away");

        assert_eq!(PeerStatus::from_str("online"), Some(PeerStatus::Online));
        assert_eq!(PeerStatus::from_str("OFFLINE"), Some(PeerStatus::Offline));
        assert_eq!(PeerStatus::from_str("invalid"), None);
    }

    #[test]
    fn test_peer_info() {
        let ip = "192.168.1.100".parse().unwrap();
        let info = PeerInfo::new(ip, 2425, Some("Alice".to_string()));

        assert_eq!(info.ip, ip);
        assert_eq!(info.port, 2425);
        assert_eq!(info.username, Some("Alice".to_string()));
    }

    #[test]
    fn test_peer_info_from_node() {
        let ip = "192.168.1.100".parse().unwrap();
        let node = PeerNode::with_details(
            ip,
            2425,
            Some("Alice".to_string()),
            Some("alice-pc".to_string()),
        );

        let info = PeerInfo::from_node(&node);

        assert_eq!(info.ip, ip);
        assert_eq!(info.port, 2425);
        assert_eq!(info.username, Some("Alice".to_string()));
    }

    #[test]
    fn test_mark_offline_online() {
        let ip = "192.168.1.100".parse().unwrap();
        let mut node = PeerNode::new(ip, 2425);

        assert!(node.is_online());

        node.mark_offline();
        assert!(!node.is_online());
        assert_eq!(node.status, PeerStatus::Offline);

        node.mark_online();
        assert!(node.is_online());
        assert_eq!(node.status, PeerStatus::Online);
    }
}

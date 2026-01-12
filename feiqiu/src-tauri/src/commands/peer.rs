// Tauri commands for peer management
//
// This module provides IPC interface between frontend and backend for peer operations.
// All commands are exposed to the frontend via Tauri's invoke system.

use crate::modules::peer::PeerNode;
use crate::state::AppState;
use crate::Result;
use serde::{Deserialize, Serialize};
use std::net::IpAddr;
use std::time::SystemTime;

/// Peer data transfer object (DTO) for frontend serialization
///
/// This struct is designed to be JSON-serializable and contains all peer
/// information that the frontend needs to display.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PeerDto {
    /// IP address
    pub ip: String,

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

    /// Current status ("online", "offline", "away")
    pub status: String,

    /// Display name (computed: nickname > username > hostname > ip)
    pub display_name: String,

    /// Last seen timestamp (Unix milliseconds)
    pub last_seen: i64,
}

impl PeerDto {
    /// Create a new PeerDto (kept for test purposes and future use)
    #[allow(dead_code)]
    pub fn new(
        ip: String,
        port: u16,
        username: Option<String>,
        hostname: Option<String>,
        nickname: Option<String>,
        avatar: Option<String>,
        groups: Vec<String>,
        status: String,
        display_name: String,
        last_seen: i64,
    ) -> Self {
        Self {
            ip,
            port,
            username,
            hostname,
            nickname,
            avatar,
            groups,
            status,
            display_name,
            last_seen,
        }
    }

    /// Create from IP address and port (minimal info) (kept for test purposes)
    #[allow(dead_code)]
    pub fn from_addr(ip: IpAddr, port: u16) -> Self {
        let display_name = ip.to_string();
        let last_seen = system_time_to_millis(SystemTime::now());

        Self {
            ip: ip.to_string(),
            port,
            username: None,
            hostname: None,
            nickname: None,
            avatar: None,
            groups: Vec::new(),
            status: "offline".to_string(),
            display_name,
            last_seen,
        }
    }

    /// Create from PeerNode
    pub fn from_peer_node(node: &PeerNode) -> Self {
        Self {
            ip: node.ip.to_string(),
            port: node.port,
            username: node.username.clone(),
            hostname: node.hostname.clone(),
            nickname: node.nickname.clone(),
            avatar: node.avatar.clone(),
            groups: node.groups.clone(),
            status: node.status.as_str().to_string(),
            display_name: node.display_name(),
            last_seen: system_time_to_millis(node.last_seen),
        }
    }
}

/// Convert SystemTime to Unix milliseconds
fn system_time_to_millis(time: SystemTime) -> i64 {
    time.duration_since(SystemTime::UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or_else(|_| 0)
}

/// Get all peers (both online and offline)
///
/// This command returns all peers currently in the peer manager.
///
/// # Frontend Usage
/// ```typescript
/// import { invoke } from "@tauri-apps/api/core";
/// const peers = await invoke<PeerDto[]>("get_peers");
/// ```
#[tauri::command]
pub fn get_peers(state: tauri::State<AppState>) -> Result<Vec<PeerDto>> {
    tracing::info!("get_peers called");

    let peers = state.get_peers();
    let dtos: Vec<PeerDto> = peers.iter().map(PeerDto::from_peer_node).collect();

    Ok(dtos)
}

/// Get only online peers
///
/// This command returns only peers that are currently online.
///
/// # Frontend Usage
/// ```typescript
/// import { invoke } from "@tauri-apps/api/core";
/// const peers = await invoke<PeerDto[]>("get_online_peers");
/// ```
#[tauri::command]
pub fn get_online_peers(state: tauri::State<AppState>) -> Result<Vec<PeerDto>> {
    tracing::info!("get_online_peers called");

    let peers = state.get_online_peers();
    let dtos: Vec<PeerDto> = peers.iter().map(PeerDto::from_peer_node).collect();

    Ok(dtos)
}

/// Get peer by IP address
///
/// This command returns a single peer by its IP address, or null if not found.
///
/// # Frontend Usage
/// ```typescript
/// import { invoke } from "@tauri-apps/api/core";
/// const peer = await invoke<PeerDto | null>("get_peer_by_ip", { ip: "192.168.1.100" });
/// ```
#[tauri::command]
pub fn get_peer_by_ip(state: tauri::State<AppState>, ip: String) -> Result<Option<PeerDto>> {
    tracing::info!("get_peer_by_ip called with ip: {}", ip);

    let ip_addr: IpAddr = ip.parse().map_err(|e| {
        crate::NeoLanError::Validation(format!("Invalid IP address: {}", e))
    })?;

    let peer = state.get_peer(ip_addr);
    Ok(peer.map(|node| PeerDto::from_peer_node(&node)))
}

/// Get peer count statistics
///
/// This command returns the number of total and online peers.
///
/// # Frontend Usage
/// ```typescript
/// import { invoke } from "@tauri-apps/api/core";
/// const stats = await invoke<{ total: number; online: number; offline: number }>("get_peer_stats");
/// console.log(`Total: ${stats.total}, Online: ${stats.online}`);
/// ```
#[tauri::command]
pub fn get_peer_stats(state: tauri::State<AppState>) -> Result<PeerStats> {
    tracing::info!("get_peer_stats called");

    let stats = state.get_peer_stats();
    Ok(PeerStats {
        total: stats.total,
        online: stats.online,
        offline: stats.offline,
    })
}

/// Peer statistics
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PeerStats {
    /// Total number of peers
    pub total: usize,

    /// Number of online peers
    pub online: usize,

    /// Number of offline peers
    pub offline: usize,
}

/// Network status
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkStatus {
    /// Whether network is connected
    pub is_connected: bool,

    /// Bound IP address
    pub bind_ip: String,

    /// UDP port
    pub udp_port: u16,

    /// Number of online peers
    pub peers_count: usize,

    /// Number of active file transfers
    pub active_transfers: usize,
}

/// Get network status
///
/// This command returns the current network connection status.
///
/// # Frontend Usage
/// ```typescript
/// import { invoke } from "@tauri-apps/api/core";
/// const status = await invoke<NetworkStatus>("get_network_status");
/// console.log(`Connected: ${status.is_connected}, Peers: ${status.peers_count}`);
/// ```
#[tauri::command]
pub fn get_network_status(state: tauri::State<AppState>) -> Result<NetworkStatus> {
    tracing::info!("get_network_status called");

    let config = state.get_config();
    let peers = state.get_online_peers();

    Ok(NetworkStatus {
        is_connected: !peers.is_empty(),
        bind_ip: config.bind_ip,
        udp_port: config.udp_port,
        peers_count: peers.len(),
        active_transfers: 0, // TODO: Track active transfers
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_peerDto_from_addr() {
        let ip: IpAddr = "192.168.1.100".parse().unwrap();
        let dto = PeerDto::from_addr(ip, 2425);

        assert_eq!(dto.ip, "192.168.1.100");
        assert_eq!(dto.port, 2425);
        assert_eq!(dto.status, "offline");
        assert_eq!(dto.display_name, "192.168.1.100");
    }

    #[test]
    fn test_peerDto_new() {
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
    fn test_system_time_to_millis() {
        let time = SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(1704000000);
        let millis = system_time_to_millis(time);
        assert_eq!(millis, 1704000000000);
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
}

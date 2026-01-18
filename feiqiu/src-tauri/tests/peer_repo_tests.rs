//! Integration tests for PeerRepository
//!
//! This module contains tests for the peer repository layer,
//! including mock implementations for unit testing.

use chrono::NaiveDateTime;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

/// In-memory mock PeerRepository for testing
///
/// This provides a mock implementation that stores peers in a HashMap
/// instead of a database, useful for unit testing.
pub struct MockPeerRepository {
    peers: Arc<Mutex<HashMap<String, feiqiu::storage::peer_repo::PeerModel>>>,
}

impl MockPeerRepository {
    /// Create a new in-memory mock repository for testing
    pub fn new() -> Self {
        Self {
            peers: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Insert or update peer by IP address (mock version)
    pub async fn upsert(
        &self,
        ip: String,
        port: i32,
        username: Option<String>,
        hostname: Option<String>,
        last_seen: NaiveDateTime,
    ) -> Result<feiqiu::storage::peer_repo::PeerModel, feiqiu::NeoLanError> {
        let mut peers = self.peers.lock().await;

        let peer = if let Some(existing) = peers.get(&ip) {
            // Update existing
            let mut updated = existing.clone();
            updated.port = port;
            updated.username = username;
            updated.hostname = hostname;
            updated.last_seen = last_seen;
            updated.updated_at = Some(chrono::Utc::now().naive_utc());
            updated
        } else {
            // Insert new
            feiqiu::storage::peer_repo::PeerModel {
                id: peers.len() as i32 + 1,
                user_id: Some(format!("T{:07}", peers.len() + 1)),
                ip: ip.clone(),
                port,
                username,
                hostname,
                nickname: None,
                avatar: None,
                groups: None,
                last_seen,
                created_at: chrono::Utc::now().naive_utc(),
                updated_at: Some(chrono::Utc::now().naive_utc()),
            }
        };

        peers.insert(ip, peer.clone());
        Ok(peer)
    }

    /// Find peer by IP address (mock version)
    pub async fn find_by_ip(
        &self,
        ip: &str,
    ) -> Result<Option<feiqiu::storage::peer_repo::PeerModel>, feiqiu::NeoLanError> {
        let peers = self.peers.lock().await;
        Ok(peers.get(ip).cloned())
    }

    /// Find all peers (mock version)
    pub async fn find_all(
        &self,
    ) -> Result<Vec<feiqiu::storage::peer_repo::PeerModel>, feiqiu::NeoLanError> {
        let peers = self.peers.lock().await;
        Ok(peers.values().cloned().collect())
    }

    /// Find online peers (mock version)
    pub async fn find_online(
        &self,
        timeout_seconds: i64,
    ) -> Result<Vec<feiqiu::storage::peer_repo::PeerModel>, feiqiu::NeoLanError> {
        let cutoff: NaiveDateTime = chrono::Utc::now()
            .checked_sub_signed(chrono::Duration::try_seconds(timeout_seconds).unwrap())
            .unwrap()
            .naive_utc();

        let peers = self.peers.lock().await;
        Ok(peers
            .values()
            .filter(|p| p.last_seen >= cutoff)
            .cloned()
            .collect())
    }

    /// Update last_seen timestamp (mock version)
    pub async fn update_last_seen(&self, ip: &str) -> Result<(), feiqiu::NeoLanError> {
        let mut peers = self.peers.lock().await;
        if let Some(peer) = peers.get_mut(ip) {
            peer.last_seen = chrono::Utc::now().naive_utc();
            peer.updated_at = Some(chrono::Utc::now().naive_utc());
            Ok(())
        } else {
            Err(feiqiu::NeoLanError::PeerNotFound(ip.to_string()))
        }
    }

    /// Delete peer by IP (mock version)
    pub async fn delete_by_ip(&self, ip: &str) -> Result<(), feiqiu::NeoLanError> {
        let mut peers = self.peers.lock().await;
        if peers.remove(ip).is_some() {
            Ok(())
        } else {
            Err(feiqiu::NeoLanError::PeerNotFound(ip.to_string()))
        }
    }
}

impl Default for MockPeerRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_peer_repo_creation() {
        let mock = MockPeerRepository::new();
        let last_seen = chrono::Utc::now().naive_utc();

        let peer = mock
            .upsert(
                "192.168.1.100".to_string(),
                2425,
                Some("Alice".to_string()),
                Some("alice-pc".to_string()),
                last_seen,
            )
            .await
            .unwrap();

        assert_eq!(peer.ip, "192.168.1.100");
        assert_eq!(peer.port, 2425);
        assert_eq!(peer.username, Some("Alice".to_string()));

        let updated = mock
            .upsert(
                "192.168.1.100".to_string(),
                2426,
                Some("Alice Updated".to_string()),
                Some("alice-pc".to_string()),
                last_seen,
            )
            .await
            .unwrap();

        assert_eq!(updated.port, 2426);
        assert_eq!(updated.username, Some("Alice Updated".to_string()));
    }

    #[tokio::test]
    async fn test_mock_repo_upsert() {
        let mock = MockPeerRepository::new();
        let last_seen = chrono::Utc::now().naive_utc();

        // Insert new peer
        let peer = mock
            .upsert(
                "192.168.1.100".to_string(),
                2425,
                Some("Alice".to_string()),
                Some("alice-pc".to_string()),
                last_seen,
            )
            .await
            .unwrap();

        assert_eq!(peer.ip, "192.168.1.100");
        assert_eq!(peer.port, 2425);
        assert_eq!(peer.username, Some("Alice".to_string()));

        // Update existing peer
        let updated = mock
            .upsert(
                "192.168.1.100".to_string(),
                2426,
                Some("Alice Updated".to_string()),
                Some("alice-pc".to_string()),
                last_seen,
            )
            .await
            .unwrap();

        assert_eq!(updated.port, 2426);
        assert_eq!(updated.username, Some("Alice Updated".to_string()));
    }

    #[tokio::test]
    async fn test_mock_repo_find_by_ip() {
        let mock = MockPeerRepository::new();
        let last_seen = chrono::Utc::now().naive_utc();

        mock.upsert(
            "192.168.1.100".to_string(),
            2425,
            Some("Alice".to_string()),
            None,
            last_seen,
        )
        .await
        .unwrap();

        let found = mock.find_by_ip("192.168.1.100").await.unwrap();
        assert!(found.is_some());
        assert_eq!(found.unwrap().username, Some("Alice".to_string()));

        let not_found = mock.find_by_ip("192.168.1.999").await.unwrap();
        assert!(not_found.is_none());
    }
}

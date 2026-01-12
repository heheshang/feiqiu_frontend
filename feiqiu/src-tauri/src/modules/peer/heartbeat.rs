// Heartbeat monitor - periodic heartbeat sending and timeout detection
//
// This module handles:
// - Sending periodic heartbeat packets to LAN
// - Detecting offline peers (no heartbeat within timeout)
// - Maintaining peer online status

use crate::modules::peer::types::PeerNode;
use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::{Duration, SystemTime};

#[allow(dead_code)]
pub fn check_offline_peers(
    peers: &Arc<Mutex<HashMap<IpAddr, PeerNode>>>,
    timeout_seconds: u64,
) -> Result<(), std::io::Error> {
    let now = SystemTime::now();
    let timeout_duration = Duration::from_secs(timeout_seconds);

    let mut peers = match peers.lock() {
        Ok(guard) => guard,
        Err(_) => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Mutex lock poisoned",
            ));
        }
    };

    let mut offline_count = 0;

    for (ip, peer) in peers.iter_mut() {
        if let Ok(duration) = now.duration_since(peer.last_seen) {
            if duration > timeout_duration {
                if peer.is_online() {
                    tracing::info!("Peer timeout: {} (last seen {:?} ago)", ip, duration);
                    peer.mark_offline();
                    offline_count += 1;
                }
            }
        }
    }

    if offline_count > 0 {
        tracing::info!("Marked {} peers as offline", offline_count);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::AppConfig;

    #[test]
    fn test_check_offline_peers() {
        // Create a peer list with one online and one stale peer
        let mut peer_map = HashMap::new();
        let ip1: IpAddr = "192.168.1.100".parse().unwrap();
        let ip2: IpAddr = "192.168.1.101".parse().unwrap();

        // Recent peer (should remain online)
        let mut peer1 = PeerNode::new(ip1, 2425);
        peer1.last_seen = SystemTime::now();
        peer_map.insert(ip1, peer1);

        // Stale peer (should be marked offline)
        let mut peer2 = PeerNode::new(ip2, 2425);
        // Set last_seen to 2 minutes ago
        let past = SystemTime::now() - Duration::from_secs(120);
        peer2.last_seen = past;
        peer_map.insert(ip2, peer2);

        let peers = Arc::new(Mutex::new(peer_map));

        // Check for offline peers with 60 second timeout
        let result = check_offline_peers(&peers, 60);
        assert!(result.is_ok());

        // Verify peer2 is now offline
        let peers_ref = peers.lock().unwrap();
        assert!(peers_ref[&ip1].is_online());
        assert!(!peers_ref[&ip2].is_online());
    }

    #[test]
    fn test_constants() {
        // Constants should match AppConfig defaults (in seconds)
        assert_eq!(AppConfig::DEFAULT_HEARTBEAT_INTERVAL, 60);
        assert_eq!(AppConfig::DEFAULT_PEER_TIMEOUT, 180);
    }
}

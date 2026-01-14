// Application event system
//
// Provides an event emitter for state changes that can be polled by the frontend.

use serde::{Deserialize, Serialize};
use std::net::IpAddr;

/// Application event
///
/// Represents various state changes in the application.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum AppEvent {
    /// Peer came online
    PeerOnline {
        ip: String,
        port: u16,
        username: Option<String>,
    },

    /// Peer went offline
    PeerOffline { ip: String },

    /// Peer status changed
    PeerStatusChanged { ip: String, status: String },

    /// Peer information updated
    PeerUpdated {
        ip: String,
        username: Option<String>,
    },

    /// Configuration changed
    ConfigChanged,

    /// Application initialized
    Initialized,

    /// Error occurred
    Error { message: String },

    /// Message received
    MessageReceived {
        msg_id: String,
        sender_ip: String,
        sender_name: String,
        content: String,
        timestamp: i64,
    },

    /// Message sent
    MessageSent { msg_id: String, receiver_ip: String },
}

impl AppEvent {
    /// Create a peer online event
    pub fn peer_online(ip: IpAddr, port: u16, username: Option<String>) -> Self {
        Self::PeerOnline {
            ip: ip.to_string(),
            port,
            username,
        }
    }

    /// Create a peer offline event
    pub fn peer_offline(ip: IpAddr) -> Self {
        Self::PeerOffline { ip: ip.to_string() }
    }

    /// Create a peer status changed event
    pub fn peer_status_changed(ip: IpAddr, status: &str) -> Self {
        Self::PeerStatusChanged {
            ip: ip.to_string(),
            status: status.to_string(),
        }
    }

    /// Create an error event
    pub fn error(message: String) -> Self {
        Self::Error { message }
    }

    /// Create a message received event
    pub fn message_received(
        msg_id: String,
        sender_ip: IpAddr,
        sender_name: String,
        content: String,
        timestamp: i64,
    ) -> Self {
        Self::MessageReceived {
            msg_id,
            sender_ip: sender_ip.to_string(),
            sender_name,
            content,
            timestamp,
        }
    }

    /// Create a message sent event
    pub fn message_sent(msg_id: String, receiver_ip: IpAddr) -> Self {
        Self::MessageSent {
            msg_id,
            receiver_ip: receiver_ip.to_string(),
        }
    }
}

/// Event emitter
///
/// Buffers events for the frontend to poll.
pub struct AppEventEmitter {
    events: Vec<AppEvent>,
    max_buffer_size: usize,
}

impl AppEventEmitter {
    /// Create a new event emitter
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
            max_buffer_size: 1000,
        }
    }

    /// Emit an event
    pub fn emit(&mut self, event: AppEvent) {
        self.events.push(event);

        // Prevent unbounded growth
        if self.events.len() > self.max_buffer_size {
            self.events.remove(0);
        }
    }

    /// Drain all pending events
    pub fn drain(&mut self) -> Vec<AppEvent> {
        std::mem::take(&mut self.events)
    }

    /// Get the number of pending events
    pub fn pending_count(&self) -> usize {
        self.events.len()
    }

    /// Clear all events
    pub fn clear(&mut self) {
        self.events.clear();
    }
}

impl Default for AppEventEmitter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_event_emitter_buffer_limit() {
        let mut emitter = AppEventEmitter::new();
        emitter.max_buffer_size = 10;

        // Add more events than buffer size
        for _ in 0..15 {
            emitter.emit(AppEvent::ConfigChanged);
        }

        // Should only keep the last 10 events
        assert_eq!(emitter.pending_count(), 10);
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
}

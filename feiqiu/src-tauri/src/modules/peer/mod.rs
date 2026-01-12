// Peer management module - node discovery, heartbeat, group management

pub mod discovery;
pub mod types;
pub mod manager;
pub mod heartbeat;

// Re-export commonly used types
pub use types::PeerNode;
pub use manager::PeerManager;
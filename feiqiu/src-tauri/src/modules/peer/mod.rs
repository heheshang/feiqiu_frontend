// Peer management module - node discovery, heartbeat, group management

pub mod discovery;
pub mod manager;
pub mod types;

// Re-export commonly used types
pub use manager::PeerManager;
pub use types::PeerNode;

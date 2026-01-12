// Application state management
//
// This module provides global state management for the Tauri application,
// including peer management, configuration, and event emission.

pub mod app_state;
pub mod events;

// Re-export commonly used types
pub use app_state::AppState;
pub use events::{AppEvent, AppEventEmitter};

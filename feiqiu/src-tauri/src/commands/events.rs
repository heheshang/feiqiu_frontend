// Event commands for state change notifications
//
// This module provides commands for the frontend to poll for state change events.

use crate::state::AppState;

/// Poll for pending events
///
/// This command returns all pending events and clears the event buffer.
/// The frontend should call this periodically to receive state change notifications.
///
/// # Frontend Usage
/// ```typescript
/// import { invoke } from "@tauri-apps/api/core";
///
/// // Poll for events every 100ms
/// setInterval(async () => {
///     const events = await invoke<AppEvent[]>("poll_events");
///     for (const event of events) {
///         handleEvent(event);
///     }
/// }, 100);
/// ```
#[tauri::command]
pub fn poll_events(state: tauri::State<AppState>) -> Vec<crate::state::AppEvent> {
    state.drain_events()
}

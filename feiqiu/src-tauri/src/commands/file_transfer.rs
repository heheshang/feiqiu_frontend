// File transfer commands - handle file transfer requests from frontend
use crate::state::AppState;
use crate::{NeoLanError, Result};
use std::sync::Arc;
use tauri::State;
use uuid::Uuid;

/// Accept a file transfer request
///
/// # Arguments
/// * `request_id` - UUID of the pending request (as string)
/// * `tcp_port` - TCP port to use for data transfer
/// * `state` - Application state
///
/// # Returns
/// * `Ok(String)` - Task ID for tracking download
/// * `Err(String)` - Accept failed
#[tauri::command]
pub fn accept_file_transfer(
    request_id: String,
    tcp_port: u16,
    state: State<'_, AppState>,
) -> Result<String> {
    tracing::info!("Accepting file transfer request: {}", request_id);

    let uuid = Uuid::parse_str(&request_id)
        .map_err(|_| NeoLanError::Validation(format!("Invalid request ID: {}", request_id)))?;

    let manager = state
        .get_file_transfer_manager()
        .ok_or_else(|| NeoLanError::Other("File transfer manager not initialized".to_string()))?;

    let request = manager.get_pending_request(&uuid).ok_or_else(|| {
        NeoLanError::FileTransfer(format!("Pending request not found: {}", request_id))
    })?;

    let config = state.get_config();
    let response_handler = crate::modules::file_transfer::FileTransferResponse::new(
        Arc::new(manager.clone()),
        config.username.clone(),
        config.hostname.clone(),
    );

    response_handler.send_response(&request, true, Some(tcp_port))?;

    let task = crate::modules::file_transfer::types::TransferTask::new_download(
        request.sender_ip,
        request.file_name.clone(),
        request.file_size,
        request.md5.clone(),
    );
    // Set the TCP port for the task
    let mut task = task;
    task.port = Some(tcp_port);

    let task_id = task.id;

    if let Some(mgr) = state.get_file_transfer_manager() {
        mgr.add_task(task)?;
        mgr.remove_pending_request(&uuid)?;
    }

    let event = request.to_event();
    state.emit_tauri_event(event);

    tracing::info!("File transfer accepted: {}", task_id);
    Ok(task_id.to_string())
}

/// Reject a file transfer request
///
/// # Arguments
/// * `request_id` - UUID of the pending request (as string)
/// * `state` - Application state
///
/// # Returns
/// * `Ok(())` - Rejection sent successfully
/// * `Err(String)` - Reject failed
#[tauri::command]
pub fn reject_file_transfer(request_id: String, state: State<'_, AppState>) -> Result<()> {
    tracing::info!("Rejecting file transfer request: {}", request_id);

    let uuid = Uuid::parse_str(&request_id)
        .map_err(|_| NeoLanError::Validation(format!("Invalid request ID: {}", request_id)))?;

    let manager = state
        .get_file_transfer_manager()
        .ok_or_else(|| NeoLanError::Other("File transfer manager not initialized".to_string()))?;

    let request = manager.get_pending_request(&uuid).ok_or_else(|| {
        NeoLanError::FileTransfer(format!("Pending request not found: {}", request_id))
    })?;

    let config = state.get_config();
    let response_handler = crate::modules::file_transfer::FileTransferResponse::new(
        Arc::new(manager.clone()),
        config.username.clone(),
        config.hostname.clone(),
    );

    response_handler.send_response(&request, false, None)?;

    manager.remove_pending_request(&uuid)?;

    let event = request.to_rejected_event();
    state.emit_tauri_event(event);

    tracing::info!("File transfer rejected: {}", request_id);
    Ok(())
}

/// Get all file transfer tasks
///
/// # Arguments
/// * `state` - Application state
///
/// # Returns
/// * `Vec<TaskDto>` - List of all transfer tasks
#[tauri::command]
pub fn get_file_transfers(state: State<'_, AppState>) -> Vec<TaskDto> {
    let manager = match state.get_file_transfer_manager() {
        Some(mgr) => mgr,
        None => return Vec::new(),
    };

    let all_tasks = manager.get_all_tasks();

    all_tasks
        .into_iter()
        .map(|task| TaskDto {
            id: task.id.to_string(),
            direction: task.direction.to_string(),
            peer_ip: task.peer_ip.to_string(),
            file_name: task.file_name.clone(),
            file_size: task.file_size,
            md5: task.md5.clone(),
            status: task.status.to_string(),
            transferred_bytes: task.transferred_bytes,
            progress: task.progress(),
            port: task.port,
            error: task.error.clone(),
            created_at: task.created_at.timestamp_millis(),
            updated_at: task.updated_at.timestamp_millis(),
        })
        .collect()
}

/// Cancel a file transfer task
///
/// # Arguments
/// * `task_id` - Task ID to cancel
/// * `state` - Application state
///
/// # Returns
/// * `Ok(())` - Task cancelled successfully
/// * `Err(String)` - Cancel failed
#[tauri::command]
pub fn cancel_file_transfer(task_id: String, _state: State<'_, AppState>) -> Result<()> {
    tracing::info!("Cancelling file transfer task: {}", task_id);

    let _uuid = Uuid::parse_str(&task_id)
        .map_err(|_| NeoLanError::Validation(format!("Invalid task ID: {}", task_id)))?;

    // TODO: Call FileTransferManager::cancel_task()

    tracing::warn!("cancel_file_transfer not fully implemented");

    Ok(())
}

/// Data transfer object for transfer tasks
#[derive(Clone, serde::Serialize)]
pub struct TaskDto {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "direction")]
    pub direction: String, // "upload" or "download"
    #[serde(rename = "peerIp")]
    pub peer_ip: String,
    #[serde(rename = "fileName")]
    pub file_name: String,
    #[serde(rename = "fileSize")]
    pub file_size: u64,
    #[serde(rename = "md5")]
    pub md5: String,
    #[serde(rename = "status")]
    pub status: String, // "pending", "active", "paused", "completed", "failed", "cancelled"
    #[serde(rename = "transferredBytes")]
    pub transferred_bytes: u64,
    #[serde(rename = "progress")]
    pub progress: f64, // 0.0 to 1.0
    #[serde(rename = "port")]
    pub port: Option<u16>,
    #[serde(rename = "error")]
    pub error: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: i64,
    #[serde(rename = "updatedAt")]
    pub updated_at: i64,
}

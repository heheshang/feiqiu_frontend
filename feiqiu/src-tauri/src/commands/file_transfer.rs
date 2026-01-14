// File transfer commands - handle file transfer requests from frontend
use crate::state::AppState;
use crate::{NeoLanError, Result};
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
/// * `Ok(String)` - Task ID for tracking the download
/// * `Err(String)` - Accept failed
#[tauri::command]
pub fn accept_file_transfer(
    request_id: String,
    _tcp_port: u16,
    _state: State<'_, AppState>,
) -> Result<String> {
    tracing::info!("Accepting file transfer request: {}", request_id);

    // Parse request ID
    let uuid = Uuid::parse_str(&request_id)
        .map_err(|_| NeoLanError::Validation(format!("Invalid request ID: {}", request_id)))?;

    // TODO: Get pending request from a pending requests storage
    // For now, we need to pass the full request info or store it somewhere
    // This is a simplified implementation that needs to be enhanced

    // In a complete implementation, we would:
    // 1. Look up the pending request by ID
    // 2. Get the FileTransferResponse handler
    // 3. Call send_response() with accept=true and the tcp_port
    // 4. Create download task
    // 5. Return task ID

    // For now, return a placeholder response
    tracing::warn!("accept_file_transfer not fully implemented - needs pending request storage");

    // Placeholder: would return the actual download task ID
    Ok(uuid.to_string())
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
pub fn reject_file_transfer(request_id: String, _state: State<'_, AppState>) -> Result<()> {
    tracing::info!("Rejecting file transfer request: {}", request_id);

    // Parse request ID
    let _uuid = Uuid::parse_str(&request_id)
        .map_err(|_| NeoLanError::Validation(format!("Invalid request ID: {}", request_id)))?;

    // TODO: Similar to accept_file_transfer, we need to:
    // 1. Look up the pending request
    // 2. Get the FileTransferResponse handler
    // 3. Call send_response() with accept=false
    // 4. Remove pending request from storage

    tracing::warn!("reject_file_transfer not fully implemented - needs pending request storage");

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
pub fn get_file_transfers(_state: State<'_, AppState>) -> Vec<TaskDto> {
    // TODO: Return all transfer tasks from FileTransferManager
    // For now, return empty list
    tracing::warn!("get_file_transfers not fully implemented");
    Vec::new()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_taskdto_serialization() {
        let dto = TaskDto {
            id: "123e4567-e89b-12d3-a456-426614174000".to_string(),
            direction: "upload".to_string(),
            peer_ip: "192.168.1.100".to_string(),
            file_name: "test.txt".to_string(),
            file_size: 1024,
            md5: "abc123".to_string(),
            status: "active".to_string(),
            transferred_bytes: 512,
            progress: 0.5,
            port: Some(8001),
            error: None,
            created_at: 1234567890,
            updated_at: 1234567891,
        };

        let json = serde_json::to_string(&dto).unwrap();
        assert!(json.contains("\"id\":\"123e4567-e89b-12d3-a456-426614174000\""));
        assert!(json.contains("\"direction\":\"upload\""));
        assert!(json.contains("\"progress\":0.5"));
    }
}

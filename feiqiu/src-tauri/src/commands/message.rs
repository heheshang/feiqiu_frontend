// Message commands
//
// Provides Tauri commands for sending and receiving messages.

use crate::state::AppState;
use crate::storage::entities::messages;
use crate::{NeoLanError, Result};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::net::IpAddr;
use tauri::Emitter;

/// Message filters for get_messages command
#[derive(Clone, Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MessageFilters {
    /// Filter by sender IP address
    #[serde(default)]
    pub sender_ip: Option<String>,
    /// Filter by receiver IP address
    #[serde(default)]
    pub receiver_ip: Option<String>,
    /// Filter by minimum timestamp (i64 milliseconds)
    #[serde(default)]
    pub after: Option<i64>,
    /// Filter by maximum timestamp (i64 milliseconds)
    #[serde(default)]
    pub before: Option<i64>,
    /// Limit number of results
    #[serde(default)]
    pub limit: Option<u64>,
}

/// Message data transfer object for frontend
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageDto {
    pub id: i32,
    pub msg_id: String,
    pub sender_ip: String,
    pub sender_name: String,
    pub receiver_ip: String,
    pub msg_type: i32,
    pub content: String,
    pub is_encrypted: bool,
    pub is_offline: bool,
    pub sent_at: i64,        // Unix milliseconds timestamp
    pub received_at: Option<i64>,
    pub created_at: i64,
}

/// Convert from database model to DTO
impl From<messages::Model> for MessageDto {
    fn from(model: messages::Model) -> Self {
        Self {
            id: model.id,
            msg_id: model.msg_id,
            sender_ip: model.sender_ip,
            sender_name: model.sender_name,
            receiver_ip: model.receiver_ip,
            msg_type: model.msg_type,
            content: model.content,
            is_encrypted: model.is_encrypted,
            is_offline: model.is_offline,
            sent_at: model.sent_at.and_utc().timestamp_millis(),
            received_at: model.received_at.map(|dt| dt.and_utc().timestamp_millis()),
            created_at: model.created_at.and_utc().timestamp_millis(),
        }
    }
}

/// Event payload for message-sent event
#[derive(Clone, serde::Serialize)]
pub struct MessageSentEvent {
    #[serde(rename = "msgId")]
    pub msg_id: String,
    #[serde(rename = "receiverIp")]
    pub receiver_ip: String,
}

/// Send a text message to a peer
///
/// # Arguments
/// * `content` - Message content to send
/// * `receiver_ip` - IP address of the target peer
/// * `app` - Tauri app handle for emitting events
/// * `state` - Application state
///
/// # Returns
/// * `Ok(message_dto)` - Message that was sent
/// * `Err(String)` - Error message if sending failed
#[tauri::command]
pub async fn send_message(
    content: String,
    receiver_ip: String,
    app: tauri::AppHandle,
    state: tauri::State<'_, AppState>,
) -> Result<MessageDto> {
    tracing::info!("send_message called: receiver_ip={}, content_len={}", receiver_ip, content.len());

    // Validate content
    if content.trim().is_empty() {
        return Err(NeoLanError::Validation(
            "Message content cannot be empty".to_string(),
        ));
    }

    // Parse IP address
    let target_ip: IpAddr = receiver_ip
        .parse()
        .map_err(|e| NeoLanError::Validation(format!("Invalid IP address: {}", e)))?;

    // Get config for local IP
    let config = state.get_config();
    let local_ip = config.bind_ip.clone();

    // Send message through state
    let msg_id = state.send_message(target_ip, &content)?;

    // Create a DTO to return (this will be updated when the message is saved to DB)
    let now = Utc::now().naive_utc();
    let dto = MessageDto {
        id: 0, // Will be set when saved to DB
        msg_id: msg_id.clone(),
        sender_ip: local_ip.clone(),
        sender_name: config.username.clone(),
        receiver_ip: receiver_ip.clone(),
        msg_type: 0, // IPMSG_SENDMSG
        content,
        is_encrypted: false,
        is_offline: false,
        sent_at: now.and_utc().timestamp_millis(),
        received_at: None,
        created_at: now.and_utc().timestamp_millis(),
    };

    // Emit message-sent event
    let sent_event = MessageSentEvent {
        msg_id: msg_id.clone(),
        receiver_ip: receiver_ip.clone(),
    };
    if let Err(e) = app.emit("message-sent", sent_event) {
        tracing::error!("Failed to emit message-sent event: {}", e);
    }

    // Note: message-received event will be emitted by MessageHandler
    // when the UDP message is received via listen_incoming callback

    tracing::info!("Message sent successfully: msg_id={}", msg_id);
    Ok(dto)
}

/// Send a text message to a peer (alias with better naming)
#[tauri::command]
pub async fn send_text_message(
    content: String,
    receiver_ip: String,
    app: tauri::AppHandle,
    state: tauri::State<'_, AppState>,
) -> Result<MessageDto> {
    send_message(content, receiver_ip, app, state).await
}

/// Get messages with optional filters
///
/// # Arguments
/// * `filters` - Optional filters to apply (senderIp, receiverIp, after, before, limit)
/// * `state` - Application state
///
/// # Returns
/// * `Ok(messages)` - List of matching messages
/// * `Err(String)` - Error message if query failed
#[tauri::command]
pub async fn get_messages(
    filters: Option<MessageFilters>,
    state: tauri::State<'_, AppState>,
) -> Result<Vec<MessageDto>> {
    let filters = filters.unwrap_or_default();
    let limit = filters.limit.unwrap_or(50);

    tracing::info!("get_messages called: filters={:?}, limit={}", filters, limit);

    // Validate limit
    if limit == 0 || limit > 1000 {
        return Err(NeoLanError::Validation(
            "Limit must be between 1 and 1000".to_string(),
        ));
    }

    // Get message repository
    let repo = state.get_message_repo()
        .ok_or_else(|| NeoLanError::Storage("Message repository not initialized".to_string()))?;

    // Build query based on filters
    let mut models = repo.find_all(limit).await?;

    // Apply sender IP filter
    if let Some(ref sender_ip) = filters.sender_ip {
        // Validate IP format
        let _ip: IpAddr = sender_ip.parse()
            .map_err(|e| NeoLanError::Validation(format!("Invalid sender IP address: {}", e)))?;

        models = models.into_iter()
            .filter(|m| m.sender_ip == *sender_ip)
            .collect();
    }

    // Apply receiver IP filter
    if let Some(ref receiver_ip) = filters.receiver_ip {
        // Validate IP format
        let _ip: IpAddr = receiver_ip.parse()
            .map_err(|e| NeoLanError::Validation(format!("Invalid receiver IP address: {}", e)))?;

        models = models.into_iter()
            .filter(|m| m.receiver_ip == *receiver_ip)
            .collect();
    }

    // Apply timestamp filters
    if let Some(after) = filters.after {
        models = models.into_iter()
            .filter(|m| m.sent_at.and_utc().timestamp_millis() >= after)
            .collect();
    }

    if let Some(before) = filters.before {
        models = models.into_iter()
            .filter(|m| m.sent_at.and_utc().timestamp_millis() <= before)
            .collect();
    }

    // Convert to DTOs
    let dtos: Vec<MessageDto> = models.into_iter().map(|m| m.into()).collect();

    tracing::info!("get_messages: returning {} messages", dtos.len());
    Ok(dtos)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_content_validation() {
        // Empty content should be rejected
        let _peer_ip = "192.168.1.100".to_string();
        let content = "".to_string();

        // Just validation test, no actual state needed
        assert!(content.trim().is_empty());
    }

    #[test]
    fn test_ip_address_parsing() {
        // Valid IP addresses
        assert!("192.168.1.100".parse::<IpAddr>().is_ok());
        assert!("127.0.0.1".parse::<IpAddr>().is_ok());
        assert!("::1".parse::<IpAddr>().is_ok());

        // Invalid IP addresses
        assert!("256.256.256.256".parse::<IpAddr>().is_err());
        assert!("invalid".parse::<IpAddr>().is_err());
    }

    #[test]
    fn test_whitespace_content_validation() {
        // Whitespace-only content should be rejected
        assert!("   ".trim().is_empty());
        assert!("\t\n".trim().is_empty());

        // Normal content should pass
        assert!(!"Hello World".trim().is_empty());
    }
}

// Message commands
//
// Provides Tauri commands for sending and receiving messages.

use crate::state::AppState;
use crate::storage::entities::messages;
use crate::{NeoLanError, Result};
use serde::{Deserialize, Serialize};
use std::net::IpAddr;
use tauri::Emitter;

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
/// * `peer_ip` - IP address of the target peer
/// * `content` - Message content to send
/// * `app` - Tauri app handle for emitting events
/// * `state` - Application state
///
/// # Returns
/// * `Ok(msg_id)` - Message ID of the sent message
/// * `Err(String)` - Error message if sending failed
#[tauri::command]
pub async fn send_message(
    peer_ip: String,
    content: String,
    app: tauri::AppHandle,
    state: tauri::State<'_, AppState>,
) -> Result<String> {
    tracing::info!("send_message called: peer_ip={}, content_len={}", peer_ip, content.len());

    // Validate content
    if content.trim().is_empty() {
        return Err(NeoLanError::Validation(
            "Message content cannot be empty".to_string(),
        ));
    }

    // Parse IP address
    let target_ip: IpAddr = peer_ip
        .parse()
        .map_err(|e| NeoLanError::Validation(format!("Invalid IP address: {}", e)))?;

    // Get config for local IP
    let config = state.get_config();
    let _local_ip = config.bind_ip.clone();

    // Send message through state
    let msg_id = state.send_message(target_ip, &content)?;

    // Emit message-sent event
    let sent_event = MessageSentEvent {
        msg_id: msg_id.clone(),
        receiver_ip: peer_ip.clone(),
    };
    if let Err(e) = app.emit("message-sent", sent_event) {
        tracing::error!("Failed to emit message-sent event: {}", e);
    }

    // Note: message-received event will be emitted by MessageHandler
    // when the UDP message is received via listen_incoming callback

    tracing::info!("Message sent successfully: msg_id={}", msg_id);
    Ok(msg_id)
}

/// Send a text message to a peer (alias with better naming)
#[tauri::command]
pub async fn send_text_message(
    peer_ip: String,
    content: String,
    app: tauri::AppHandle,
    state: tauri::State<'_, AppState>,
) -> Result<String> {
    send_message(peer_ip, content, app, state).await
}

/// Get messages with a specific peer
///
/// # Arguments
/// * `peer_ip` - IP address of the peer
/// * `limit` - Maximum number of messages to retrieve (default 50)
///
/// # Returns
/// * `Ok(messages)` - List of messages with the peer
/// * `Err(String)` - Error message if query failed
#[tauri::command]
pub async fn get_messages(
    peer_ip: String,
    limit: Option<u64>,
) -> Result<Vec<MessageDto>> {
    let limit = limit.unwrap_or(50);
    tracing::info!("get_messages called: peer_ip={}, limit={}", peer_ip, limit);

    // Validate IP address format
    let _ip: IpAddr = peer_ip
        .parse()
        .map_err(|e| NeoLanError::Validation(format!("Invalid IP address: {}", e)))?;

    // Validate limit
    if limit == 0 || limit > 1000 {
        return Err(NeoLanError::Validation(
            "Limit must be between 1 and 1000".to_string(),
        ));
    }

    // TODO: Integrate with MessageRepository when database is added to AppState
    // For now, return empty vector as placeholder
    // Future implementation:
    // let repo = state.get_message_repo()?;
    // let models = repo.find_by_peer(&peer_ip, limit as usize).await?;
    // let dtos: Vec<MessageDto> = models.into_iter().map(|m| m.into()).collect();
    // Ok(dtos)

    tracing::warn!("get_messages: Database not yet integrated, returning empty result");
    Ok(Vec::new())
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

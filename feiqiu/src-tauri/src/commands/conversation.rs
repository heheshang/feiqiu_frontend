// Conversation commands
//
// Provides Tauri commands for managing conversations.

use crate::state::AppState;
use crate::storage::entities::{conversations, conversation_participants};
use crate::{NeoLanError, Result};
use serde::{Deserialize, Serialize};

/// Conversation data transfer object for frontend
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversationDto {
    pub id: i32,
    pub r#type: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub is_pinned: bool,
    pub is_archived: bool,
    pub is_muted: bool,
    pub unread_count: i32,
    pub last_message_id: Option<i32>,
    pub last_message_at: Option<i64>,
    pub last_message_content: Option<String>,
    pub last_message_type: Option<String>,
    pub participants: Vec<ParticipantDto>,
}

/// Participant data transfer object for frontend
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticipantDto {
    pub id: i32,
    pub conversation_id: i32,
    pub peer_ip: String,
    pub joined_at: i64,
    pub left_at: Option<i64>,
    pub role: String,
}

/// Update conversation input
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateConversationInput {
    pub id: i32,
    #[serde(default)]
    pub is_pinned: Option<bool>,
    #[serde(default)]
    pub is_archived: Option<bool>,
    #[serde(default)]
    pub is_muted: Option<bool>,
}

/// Convert from database model to DTO
impl From<conversations::Model> for ConversationDto {
    fn from(model: conversations::Model) -> Self {
        Self {
            id: model.id,
            r#type: model.r#type,
            created_at: model.created_at.and_utc().timestamp_millis(),
            updated_at: model.updated_at.and_utc().timestamp_millis(),
            is_pinned: model.is_pinned,
            is_archived: model.is_archived,
            is_muted: model.is_muted,
            unread_count: model.unread_count,
            last_message_id: model.last_message_id,
            last_message_at: model.last_message_at.map(|dt| dt.and_utc().timestamp_millis()),
            last_message_content: model.last_message_content,
            last_message_type: model.last_message_type,
            participants: Vec::new(), // Will be populated separately
        }
    }
}

/// Convert from database model to DTO
impl From<conversation_participants::Model> for ParticipantDto {
    fn from(model: conversation_participants::Model) -> Self {
        Self {
            id: model.id,
            conversation_id: model.conversation_id,
            peer_ip: model.peer_ip,
            joined_at: model.joined_at.and_utc().timestamp_millis(),
            left_at: model.left_at.map(|dt| dt.and_utc().timestamp_millis()),
            role: model.role,
        }
    }
}

/// Get all conversations
///
/// Frontend Usage:
/// ```typescript
/// const conversations = await invoke<ConversationDto[]>("get_conversations");
/// ```
#[tauri::command]
pub async fn get_conversations(
    state: tauri::State<'_, AppState>,
) -> Result<Vec<ConversationDto>> {
    tracing::info!("get_conversations called");

    let conv_repo = state
        .get_conversation_repo()
        .ok_or_else(|| NeoLanError::Storage("Conversation repository not initialized".to_string()))?;

    let conversations = conv_repo.find_all().await.map_err(|e| {
        NeoLanError::Storage(format!("Failed to get conversations: {}", e))
    })?;

    let mut result = Vec::new();
    for conv in conversations {
        let participants = conv_repo.get_participants(conv.id).await.map_err(|e| {
            NeoLanError::Storage(format!("Failed to get participants: {}", e))
        })?;

        let mut dto: ConversationDto = conv.into();
        dto.participants = participants.into_iter().map(ParticipantDto::from).collect();
        result.push(dto);
    }

    tracing::info!("get_conversations returned {} conversations", result.len());
    Ok(result)
}

/// Get or create a conversation with a peer
///
/// Frontend Usage:
/// ```typescript
/// const conversation = await invoke<ConversationDto>("get_or_create_conversation", { peerIp: "192.168.1.100" });
/// ```
#[tauri::command]
pub async fn get_or_create_conversation(
    peer_ip: String,
    _app: tauri::AppHandle,
    state: tauri::State<'_, AppState>,
) -> Result<ConversationDto> {
    tracing::info!("get_or_create_conversation called: peer_ip={}", peer_ip);

    let config = state.get_config();
    let my_ip = config.bind_ip;

    let conv_repo = state
        .get_conversation_repo()
        .ok_or_else(|| NeoLanError::Storage("Conversation repository not initialized".to_string()))?;

    let conversation = conv_repo
        .find_or_create_single_conversation(&my_ip, &peer_ip)
        .await
        .map_err(|e| NeoLanError::Storage(format!("Failed to get or create conversation: {}", e)))?;

    let participants = conv_repo.get_participants(conversation.id).await.map_err(|e| {
        NeoLanError::Storage(format!("Failed to get participants: {}", e))
    })?;

    let mut dto: ConversationDto = conversation.into();
    dto.participants = participants.into_iter().map(ParticipantDto::from).collect();

    tracing::info!("get_or_create_conversation returned conversation id={}", dto.id);
    Ok(dto)
}

/// Update conversation metadata
///
/// Frontend Usage:
/// ```typescript
/// const conversation = await invoke<ConversationDto>("update_conversation", {
///   id: 1,
///   isPinned: true
/// });
/// ```
#[tauri::command]
pub async fn update_conversation(
    input: UpdateConversationInput,
    state: tauri::State<'_, AppState>,
) -> Result<ConversationDto> {
    tracing::info!("update_conversation called: id={}", input.id);

    let conv_repo = state
        .get_conversation_repo()
        .ok_or_else(|| NeoLanError::Storage("Conversation repository not initialized".to_string()))?;

    let mut conversation = conv_repo
        .find_by_id(input.id)
        .await?
        .ok_or_else(|| NeoLanError::Storage(format!("Conversation not found: {}", input.id)))?;

    // Update fields if provided
    if let Some(is_pinned) = input.is_pinned {
        conversation.is_pinned = is_pinned;
    }
    if let Some(is_archived) = input.is_archived {
        conversation.is_archived = is_archived;
    }
    if let Some(is_muted) = input.is_muted {
        conversation.is_muted = is_muted;
    }
    conversation.updated_at = chrono::Utc::now().naive_utc();

    conv_repo.update(&conversation).await.map_err(|e| {
        NeoLanError::Storage(format!("Failed to update conversation: {}", e))
    })?;

    let participants = conv_repo.get_participants(conversation.id).await.map_err(|e| {
        NeoLanError::Storage(format!("Failed to get participants: {}", e))
    })?;

    let mut dto: ConversationDto = conversation.into();
    dto.participants = participants.into_iter().map(ParticipantDto::from).collect();

    tracing::info!("update_conversation updated conversation id={}", dto.id);
    Ok(dto)
}

/// Mark conversation as read (clear unread count)
///
/// Frontend Usage:
/// ```typescript
/// await invoke("mark_conversation_read", { conversationId: 1 });
/// ```
#[tauri::command]
pub async fn mark_conversation_read(
    conversation_id: i32,
    state: tauri::State<'_, AppState>,
) -> Result<()> {
    tracing::info!("mark_conversation_read called: conversation_id={}", conversation_id);

    let conv_repo = state
        .get_conversation_repo()
        .ok_or_else(|| NeoLanError::Storage("Conversation repository not initialized".to_string()))?;

    conv_repo.mark_as_read(conversation_id).await.map_err(|e| {
        NeoLanError::Storage(format!("Failed to mark conversation as read: {}", e))
    })?;

    tracing::info!("mark_conversation_read completed for conversation_id={}", conversation_id);
    Ok(())
}

/// Delete a conversation
///
/// Frontend Usage:
/// ```typescript
/// await invoke("delete_conversation", { conversationId: 1, deleteMessages: false });
/// ```
#[tauri::command]
pub async fn delete_conversation(
    conversation_id: i32,
    delete_messages: bool,
    state: tauri::State<'_, AppState>,
) -> Result<()> {
    tracing::info!(
        "delete_conversation called: conversation_id={}, delete_messages={}",
        conversation_id,
        delete_messages
    );

    let conv_repo = state
        .get_conversation_repo()
        .ok_or_else(|| NeoLanError::Storage("Conversation repository not initialized".to_string()))?;

    // If delete_messages is true, also delete associated messages
    if delete_messages {
        // TODO: Implement message deletion for this conversation
        tracing::warn!("delete_messages=true not yet implemented");
    }

    conv_repo.delete(conversation_id).await.map_err(|e| {
        NeoLanError::Storage(format!("Failed to delete conversation: {}", e))
    })?;

    tracing::info!("delete_conversation completed for conversation_id={}", conversation_id);
    Ok(())
}

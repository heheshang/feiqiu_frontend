// src-tauri/src/storage/conversation_repo.rs
use crate::error::{NeoLanError, Result};
use crate::storage::entities::{conversations, conversation_participants};
use sea_orm::*;

pub type ConversationModel = conversations::Model;
pub type ConversationActiveModel = conversations::ActiveModel;
pub type ConversationEntity = conversations::Entity;

pub type ParticipantModel = conversation_participants::Model;
pub type ParticipantActiveModel = conversation_participants::ActiveModel;
pub type ParticipantEntity = conversation_participants::Entity;

/// Conversation data access layer
///
/// Provides CRUD operations for conversations and their participants
#[derive(Clone)]
pub struct ConversationRepository {
    db: DatabaseConnection,
}

impl ConversationRepository {
    /// Create a new ConversationRepository
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// Insert a new conversation
    pub async fn insert(&self, conversation: &ConversationModel) -> Result<i32> {
        let active_model: ConversationActiveModel = conversation.clone().into();

        let result = ConversationEntity::insert(active_model)
            .exec(&self.db)
            .await
            .map_err(|e| NeoLanError::Storage(format!("Failed to insert conversation: {}", e)))?;

        Ok(result.last_insert_id)
    }

    /// Find conversation by ID
    pub async fn find_by_id(&self, id: i32) -> Result<Option<ConversationModel>> {
        let result = ConversationEntity::find()
            .filter(conversations::Column::Id.eq(id))
            .one(&self.db)
            .await
            .map_err(|e| NeoLanError::Storage(format!("Failed to find conversation by ID: {}", e)))?;

        Ok(result)
    }

    /// Find single conversation by peer IPs
    ///
    /// For single chats, finds a conversation where both participants are present
    pub async fn find_single_conversation_by_peer(
        &self,
        my_ip: &str,
        peer_ip: &str,
    ) -> Result<Option<ConversationModel>> {
        // Find all 'single' type conversations
        let conversations = ConversationEntity::find()
            .filter(conversations::Column::Type.eq("single"))
            .all(&self.db)
            .await
            .map_err(|e| NeoLanError::Storage(format!("Failed to find conversation: {}", e)))?;

        for conv in conversations {
            // Check if this conversation has both participants
            let participants = ParticipantEntity::find()
                .filter(conversation_participants::Column::ConversationId.eq(conv.id))
                .filter(conversation_participants::Column::LeftAt.is_null())
                .all(&self.db)
                .await
                .map_err(|e| NeoLanError::Storage(format!("Failed to find participants: {}", e)))?;

            let peer_ips: Vec<String> = participants.iter().map(|p| p.peer_ip.clone()).collect();

            // Check if both my_ip and peer_ip are in the participants
            if peer_ips.contains(&my_ip.to_string()) && peer_ips.contains(&peer_ip.to_string()) {
                return Ok(Some(conv));
            }
        }

        Ok(None)
    }

    /// Get all conversations
    pub async fn find_all(&self) -> Result<Vec<ConversationModel>> {
        let result = ConversationEntity::find()
            .filter(conversations::Column::IsArchived.eq(false))
            .order_by_desc(conversations::Column::IsPinned)
            .order_by_desc(conversations::Column::UpdatedAt)
            .all(&self.db)
            .await
            .map_err(|e| NeoLanError::Storage(format!("Failed to find all conversations: {}", e)))?;

        Ok(result)
    }

    /// Update conversation metadata
    pub async fn update(&self, conversation: &ConversationModel) -> Result<()> {
        let active_model: ConversationActiveModel = conversation.clone().into();

        ConversationEntity::update(active_model)
            .exec(&self.db)
            .await
            .map_err(|e| NeoLanError::Storage(format!("Failed to update conversation: {}", e)))?;

        Ok(())
    }

    /// Delete conversation by ID
    pub async fn delete(&self, id: i32) -> Result<()> {
        ConversationEntity::delete_by_id(id)
            .exec(&self.db)
            .await
            .map_err(|e| NeoLanError::Storage(format!("Failed to delete conversation: {}", e)))?;

        Ok(())
    }

    /// Increment unread count
    pub async fn increment_unread(&self, id: i32) -> Result<()> {
        let conversation = self
            .find_by_id(id)
            .await?
            .ok_or_else(|| NeoLanError::Storage(format!("Conversation not found: {}", id)))?;

        let mut active_model: ConversationActiveModel = conversation.into();
        active_model.unread_count = Set(active_model.unread_count.unwrap() + 1);
        active_model.updated_at = Set(chrono::Utc::now().naive_utc());

        ConversationEntity::update(active_model)
            .exec(&self.db)
            .await
            .map_err(|e| NeoLanError::Storage(format!("Failed to increment unread: {}", e)))?;

        Ok(())
    }

    /// Mark conversation as read (unread_count = 0)
    pub async fn mark_as_read(&self, id: i32) -> Result<()> {
        let conversation = self
            .find_by_id(id)
            .await?
            .ok_or_else(|| NeoLanError::Storage(format!("Conversation not found: {}", id)))?;

        let mut active_model: ConversationActiveModel = conversation.into();
        active_model.unread_count = Set(0);
        active_model.updated_at = Set(chrono::Utc::now().naive_utc());

        ConversationEntity::update(active_model)
            .exec(&self.db)
            .await
            .map_err(|e| NeoLanError::Storage(format!("Failed to mark as read: {}", e)))?;

        Ok(())
    }

    /// Update last message info
    pub async fn update_last_message(
        &self,
        id: i32,
        message_id: i32,
        content: &str,
        msg_type: &str,
        sent_at: chrono::NaiveDateTime,
    ) -> Result<()> {
        let conversation = self
            .find_by_id(id)
            .await?
            .ok_or_else(|| NeoLanError::Storage(format!("Conversation not found: {}", id)))?;

        let mut active_model: ConversationActiveModel = conversation.into();
        active_model.last_message_id = Set(Some(message_id));
        active_model.last_message_content = Set(Some(content.to_string()));
        active_model.last_message_type = Set(Some(msg_type.to_string()));
        active_model.last_message_at = Set(Some(sent_at));
        active_model.updated_at = Set(chrono::Utc::now().naive_utc());

        ConversationEntity::update(active_model)
            .exec(&self.db)
            .await
            .map_err(|e| NeoLanError::Storage(format!("Failed to update last message: {}", e)))?;

        Ok(())
    }

    /// Add participant to conversation
    pub async fn add_participant(
        &self,
        conversation_id: i32,
        peer_ip: &str,
        role: &str,
    ) -> Result<i32> {
        let participant = ParticipantActiveModel {
            conversation_id: Set(conversation_id),
            peer_ip: Set(peer_ip.to_string()),
            joined_at: Set(chrono::Utc::now().naive_utc()),
            left_at: Set(None),
            role: Set(role.to_string()),
            ..Default::default()
        };

        let result = ParticipantEntity::insert(participant)
            .exec(&self.db)
            .await
            .map_err(|e| NeoLanError::Storage(format!("Failed to add participant: {}", e)))?;

        Ok(result.last_insert_id)
    }

    /// Get participants for a conversation
    pub async fn get_participants(&self, conversation_id: i32) -> Result<Vec<ParticipantModel>> {
        let result = ParticipantEntity::find()
            .filter(conversation_participants::Column::ConversationId.eq(conversation_id))
            .filter(conversation_participants::Column::LeftAt.is_null())
            .all(&self.db)
            .await
            .map_err(|e| NeoLanError::Storage(format!("Failed to get participants: {}", e)))?;

        Ok(result)
    }

    /// Create a single conversation with two participants
    pub async fn create_single_conversation(
        &self,
        my_ip: &str,
        peer_ip: &str,
    ) -> Result<i32> {
        let now = chrono::Utc::now().naive_utc();

        let conversation = ConversationModel {
            id: 0, // Auto-assigned
            r#type: "single".to_string(),
            created_at: now,
            updated_at: now,
            is_pinned: false,
            is_archived: false,
            is_muted: false,
            unread_count: 0,
            last_message_id: None,
            last_message_at: None,
            last_message_content: None,
            last_message_type: None,
        };

        let conv_id = self.insert(&conversation).await?;

        // Add both participants
        self.add_participant(conv_id, my_ip, "member").await?;
        self.add_participant(conv_id, peer_ip, "member").await?;

        Ok(conv_id)
    }

    /// Find or create a single conversation between two peers
    pub async fn find_or_create_single_conversation(
        &self,
        my_ip: &str,
        peer_ip: &str,
    ) -> Result<ConversationModel> {
        // Try to find existing conversation
        if let Some(conv) = self.find_single_conversation_by_peer(my_ip, peer_ip).await? {
            return Ok(conv);
        }

        // Create new conversation
        let conv_id = self.create_single_conversation(my_ip, peer_ip).await?;
        self.find_by_id(conv_id)
            .await?
            .ok_or_else(|| NeoLanError::Storage(format!("Failed to retrieve created conversation: {}", conv_id)))
    }
}

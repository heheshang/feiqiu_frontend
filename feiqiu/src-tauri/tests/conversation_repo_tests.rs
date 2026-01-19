//! Integration tests for ConversationRepository
//!
//! This module contains tests for the conversation repository layer,
//! including mock implementations for unit testing.

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

/// In-memory mock ConversationRepository for testing
///
/// This provides a mock implementation that stores conversations in a HashMap
/// instead of a database, useful for unit testing.
pub struct MockConversationRepository {
    conversations: Arc<Mutex<HashMap<i32, feiqiu::storage::conversation_repo::ConversationModel>>>,
    participants: Arc<Mutex<Vec<feiqiu::storage::entities::conversation_participants::Model>>>,
    next_id: Arc<Mutex<i32>>,
}

impl MockConversationRepository {
    /// Create a new in-memory mock repository for testing
    pub fn new() -> Self {
        Self {
            conversations: Arc::new(Mutex::new(HashMap::new())),
            participants: Arc::new(Mutex::new(Vec::new())),
            next_id: Arc::new(Mutex::new(1)),
        }
    }

    /// Insert a new conversation
    pub async fn insert(
        &self,
        r#type: String,
        my_ip: &str,
        peer_ip: &str,
    ) -> Result<feiqiu::storage::conversation_repo::ConversationModel, feiqiu::NeoLanError> {
        let mut conversations = self.conversations.lock().await;
        let mut next_id = self.next_id.lock().await;

        let now = chrono::Utc::now().naive_utc();
        let id = *next_id;
        *next_id += 1;

        let conversation = feiqiu::storage::conversation_repo::ConversationModel {
            id,
            r#type,
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

        // Add participants
        let mut participants = self.participants.lock().await;
        let id1 = participants.len() as i32 + 1;
        let id2 = id1 + 1;

        // Add local user participant
        participants.push(feiqiu::storage::entities::conversation_participants::Model {
            id: id1,
            conversation_id: id,
            peer_ip: my_ip.to_string(),
            joined_at: now,
            left_at: None,
            role: "member".to_string(),
        });

        // Add peer participant
        participants.push(feiqiu::storage::entities::conversation_participants::Model {
            id: id2,
            conversation_id: id,
            peer_ip: peer_ip.to_string(),
            joined_at: now,
            left_at: None,
            role: "member".to_string(),
        });

        conversations.insert(id, conversation.clone());
        Ok(conversation)
    }

    /// Find conversation by ID
    pub async fn find_by_id(
        &self,
        id: i32,
    ) -> Result<Option<feiqiu::storage::conversation_repo::ConversationModel>, feiqiu::NeoLanError> {
        let conversations = self.conversations.lock().await;
        Ok(conversations.get(&id).cloned())
    }

    /// Find or create single conversation by peer IPs
    pub async fn find_or_create_single_conversation(
        &self,
        my_ip: &str,
        peer_ip: &str,
    ) -> Result<feiqiu::storage::conversation_repo::ConversationModel, feiqiu::NeoLanError> {
        let participants = self.participants.lock().await;

        // Try to find existing conversation
        for participant in participants.iter() {
            if participant.peer_ip == peer_ip {
                if let Some(conv) = self.conversations.lock().await.get(&participant.conversation_id) {
                    // Check if this is a single conversation with both participants
                    let has_my_ip = participants.iter().any(|p|
                        p.conversation_id == conv.id && p.peer_ip == my_ip && p.left_at.is_none()
                    );
                    let has_peer_ip = participants.iter().any(|p|
                        p.conversation_id == conv.id && p.peer_ip == peer_ip && p.left_at.is_none()
                    );

                    if has_my_ip && has_peer_ip && conv.r#type == "single" {
                        return Ok(conv.clone());
                    }
                }
            }
        }

        // Drop the lock before creating a new conversation
        drop(participants);

        // Create new conversation
        self.insert("single".to_string(), my_ip, peer_ip).await
    }

    /// Find all conversations
    pub async fn find_all(
        &self,
        limit: u64,
    ) -> Result<Vec<feiqiu::storage::conversation_repo::ConversationModel>, feiqiu::NeoLanError> {
        let conversations = self.conversations.lock().await;
        let mut result: Vec<_> = conversations.values().cloned().collect();
        result.sort_by(|a, b| b.id.cmp(&a.id));
        result.truncate(limit as usize);
        Ok(result)
    }

    /// Update conversation metadata
    pub async fn update(
        &self,
        id: i32,
        is_pinned: Option<bool>,
        is_archived: Option<bool>,
        is_muted: Option<bool>,
    ) -> Result<feiqiu::storage::conversation_repo::ConversationModel, feiqiu::NeoLanError> {
        let mut conversations = self.conversations.lock().await;

        if let Some(conv) = conversations.get_mut(&id) {
            if let Some(pinned) = is_pinned {
                conv.is_pinned = pinned;
            }
            if let Some(archived) = is_archived {
                conv.is_archived = archived;
            }
            if let Some(muted) = is_muted {
                conv.is_muted = muted;
            }
            conv.updated_at = chrono::Utc::now().naive_utc();
            Ok(conv.clone())
        } else {
            Err(feiqiu::NeoLanError::Storage(format!(
                "Conversation with id {} not found",
                id
            )))
        }
    }

    /// Increment unread count
    pub async fn increment_unread(&self, id: i32) -> Result<(), feiqiu::NeoLanError> {
        let mut conversations = self.conversations.lock().await;

        if let Some(conv) = conversations.get_mut(&id) {
            conv.unread_count += 1;
            conv.updated_at = chrono::Utc::now().naive_utc();
            Ok(())
        } else {
            Err(feiqiu::NeoLanError::Storage(format!(
                "Conversation with id {} not found",
                id
            )))
        }
    }

    /// Mark conversation as read
    pub async fn mark_as_read(&self, id: i32) -> Result<(), feiqiu::NeoLanError> {
        let mut conversations = self.conversations.lock().await;

        if let Some(conv) = conversations.get_mut(&id) {
            conv.unread_count = 0;
            conv.updated_at = chrono::Utc::now().naive_utc();
            Ok(())
        } else {
            Err(feiqiu::NeoLanError::Storage(format!(
                "Conversation with id {} not found",
                id
            )))
        }
    }

    /// Update last message info
    pub async fn update_last_message(
        &self,
        id: i32,
        message_id: i32,
        content: &str,
        msg_type: &str,
        timestamp: chrono::NaiveDateTime,
    ) -> Result<(), feiqiu::NeoLanError> {
        let mut conversations = self.conversations.lock().await;

        if let Some(conv) = conversations.get_mut(&id) {
            conv.last_message_id = Some(message_id);
            conv.last_message_content = Some(content.to_string());
            conv.last_message_type = Some(msg_type.to_string());
            conv.last_message_at = Some(timestamp);
            conv.updated_at = chrono::Utc::now().naive_utc();
            Ok(())
        } else {
            Err(feiqiu::NeoLanError::Storage(format!(
                "Conversation with id {} not found",
                id
            )))
        }
    }

    /// Delete conversation
    pub async fn delete(&self, id: i32) -> Result<(), feiqiu::NeoLanError> {
        let mut conversations = self.conversations.lock().await;

        if conversations.remove(&id).is_some() {
            // Also remove participants
            let mut participants = self.participants.lock().await;
            participants.retain(|p| p.conversation_id != id);
            Ok(())
        } else {
            Err(feiqiu::NeoLanError::Storage(format!(
                "Conversation with id {} not found",
                id
            )))
        }
    }

    /// Get participants for a conversation
    pub async fn get_participants(
        &self,
        conversation_id: i32,
    ) -> Result<Vec<feiqiu::storage::entities::conversation_participants::Model>, feiqiu::NeoLanError> {
        let participants = self.participants.lock().await;
        Ok(participants
            .iter()
            .filter(|p| p.conversation_id == conversation_id && p.left_at.is_none())
            .cloned()
            .collect())
    }
}

impl Default for MockConversationRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_conversation_repo_insert() {
        let mock = MockConversationRepository::new();

        let conversation = mock
            .insert("single".to_string(), "192.168.1.1", "192.168.1.100")
            .await
            .unwrap();

        assert_eq!(conversation.id, 1);
        assert_eq!(conversation.r#type, "single");
        assert_eq!(conversation.unread_count, 0);
        assert!(!conversation.is_pinned);
        assert!(!conversation.is_archived);
        assert!(!conversation.is_muted);
    }

    #[tokio::test]
    async fn test_conversation_repo_find_or_create() {
        let mock = MockConversationRepository::new();

        // First call should create
        let conv1 = mock
            .find_or_create_single_conversation("192.168.1.1", "192.168.1.100")
            .await
            .unwrap();
        assert_eq!(conv1.id, 1);

        // Second call should return existing
        let conv2 = mock
            .find_or_create_single_conversation("192.168.1.1", "192.168.1.100")
            .await
            .unwrap();
        assert_eq!(conv2.id, 1); // Same ID

        // Different peer should create new
        let conv3 = mock
            .find_or_create_single_conversation("192.168.1.1", "192.168.1.101")
            .await
            .unwrap();
        assert_eq!(conv3.id, 2); // New ID
    }

    #[tokio::test]
    async fn test_conversation_repo_update() {
        let mock = MockConversationRepository::new();

        let conversation = mock
            .insert("single".to_string(), "192.168.1.1", "192.168.1.100")
            .await
            .unwrap();

        // Update pinned status
        let updated = mock
            .update(conversation.id, Some(true), None, None)
            .await
            .unwrap();
        assert!(updated.is_pinned);

        // Update multiple fields
        let updated = mock
            .update(conversation.id, None, Some(true), Some(true))
            .await
            .unwrap();
        assert!(updated.is_pinned);
        assert!(updated.is_archived);
        assert!(updated.is_muted);
    }

    #[tokio::test]
    async fn test_conversation_repo_unread() {
        let mock = MockConversationRepository::new();

        let conversation = mock
            .insert("single".to_string(), "192.168.1.1", "192.168.1.100")
            .await
            .unwrap();

        // Increment unread
        mock.increment_unread(conversation.id).await.unwrap();
        let found = mock.find_by_id(conversation.id).await.unwrap().unwrap();
        assert_eq!(found.unread_count, 1);

        mock.increment_unread(conversation.id).await.unwrap();
        let found = mock.find_by_id(conversation.id).await.unwrap().unwrap();
        assert_eq!(found.unread_count, 2);

        // Mark as read
        mock.mark_as_read(conversation.id).await.unwrap();
        let found = mock.find_by_id(conversation.id).await.unwrap().unwrap();
        assert_eq!(found.unread_count, 0);
    }

    #[tokio::test]
    async fn test_conversation_repo_last_message() {
        let mock = MockConversationRepository::new();

        let conversation = mock
            .insert("single".to_string(), "192.168.1.1", "192.168.1.100")
            .await
            .unwrap();

        let timestamp = chrono::Utc::now().naive_utc();
        mock.update_last_message(
            conversation.id,
            123,
            "Hello, world!",
            "text",
            timestamp,
        )
        .await
        .unwrap();

        let found = mock.find_by_id(conversation.id).await.unwrap().unwrap();
        assert_eq!(found.last_message_id, Some(123));
        assert_eq!(found.last_message_content, Some("Hello, world!".to_string()));
        assert_eq!(found.last_message_type, Some("text".to_string()));
        assert_eq!(found.last_message_at, Some(timestamp));
    }

    #[tokio::test]
    async fn test_conversation_repo_delete() {
        let mock = MockConversationRepository::new();

        let conversation = mock
            .insert("single".to_string(), "192.168.1.1", "192.168.1.100")
            .await
            .unwrap();

        // Verify it exists
        assert!(mock.find_by_id(conversation.id).await.unwrap().is_some());

        // Delete it
        mock.delete(conversation.id).await.unwrap();

        // Verify it's gone
        assert!(mock.find_by_id(conversation.id).await.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_conversation_repo_get_participants() {
        let mock = MockConversationRepository::new();

        let conversation = mock
            .insert("single".to_string(), "192.168.1.1", "192.168.1.100")
            .await
            .unwrap();

        let participants = mock.get_participants(conversation.id).await.unwrap();
        assert_eq!(participants.len(), 2);

        // Check both participants are present
        let peer_ips: Vec<_> = participants.iter().map(|p| &p.peer_ip).collect();
        assert!(peer_ips.iter().any(|p| *p == "192.168.1.1"));
        assert!(peer_ips.iter().any(|p| *p == "192.168.1.100"));
    }

    #[tokio::test]
    async fn test_conversation_repo_find_all() {
        let mock = MockConversationRepository::new();

        // Create multiple conversations
        mock.insert("single".to_string(), "192.168.1.1", "192.168.1.100")
            .await
            .unwrap();
        mock.insert("single".to_string(), "192.168.1.1", "192.168.1.101")
            .await
            .unwrap();
        mock.insert("single".to_string(), "192.168.1.1", "192.168.1.102")
            .await
            .unwrap();

        // Find all with limit
        let all = mock.find_all(10).await.unwrap();
        assert_eq!(all.len(), 3);

        // Find with limit
        let limited = mock.find_all(2).await.unwrap();
        assert_eq!(limited.len(), 2);
    }

    #[tokio::test]
    async fn test_conversation_repo_error_handling() {
        let mock = MockConversationRepository::new();

        // Test updating non-existent conversation
        let result = mock.update(999, Some(true), None, None).await;
        assert!(result.is_err());

        // Test incrementing unread for non-existent conversation
        let result = mock.increment_unread(999).await;
        assert!(result.is_err());

        // Test marking as read for non-existent conversation
        let result = mock.mark_as_read(999).await;
        assert!(result.is_err());

        // Test deleting non-existent conversation
        let result = mock.delete(999).await;
        assert!(result.is_err());
    }
}

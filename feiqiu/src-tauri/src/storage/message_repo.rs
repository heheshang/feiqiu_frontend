// src-tauri/src/storage/message_repo.rs
use crate::error::{NeoLanError, Result};
use crate::storage::entities::messages;
use chrono::NaiveDateTime;
use sea_orm::*;

pub type MessageModel = messages::Model;
pub type MessageActiveModel = messages::ActiveModel;
pub type MessageEntity = messages::Entity;

/// 消息数据访问层
///
/// 提供 messages 表的 CRUD 操作
#[derive(Clone)]
pub struct MessageRepository {
    db: DatabaseConnection,
}

impl MessageRepository {
    /// 创建新的 MessageRepository
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// 插入新消息
    pub async fn insert(&self, message: &MessageModel) -> Result<i32> {
        let active_model: MessageActiveModel = message.clone().into();

        let result = MessageEntity::insert(active_model)
            .exec(&self.db)
            .await
            .map_err(|e| NeoLanError::Storage(format!("Failed to insert message: {}", e)))?;

        Ok(result.last_insert_id)
    }

    /// 根据 msg_id 查找消息
    pub async fn find_by_msg_id(&self, msg_id: &str) -> Result<Option<MessageModel>> {
        let result = MessageEntity::find()
            .filter(messages::Column::MsgId.eq(msg_id))
            .one(&self.db)
            .await
            .map_err(|e| NeoLanError::Storage(format!("Failed to find message by ID: {}", e)))?;

        Ok(result)
    }

    /// 查找与特定节点的消息
    ///
    /// # 参数
    /// - `peer_ip`: 节点 IP 地址
    /// - `limit`: 限制返回的消息数量
    ///
    /// # 返回
    /// 发送或接收自该节点的消息，按发送时间倒序
    pub async fn find_by_peer(&self, peer_ip: &str, limit: u64) -> Result<Vec<MessageModel>> {
        let result = MessageEntity::find()
            .filter(
                Condition::any()
                    .add(messages::Column::SenderIp.eq(peer_ip))
                    .add(messages::Column::ReceiverIp.eq(peer_ip)),
            )
            .order_by_desc(messages::Column::SentAt)
            .limit(limit)
            .all(&self.db)
            .await
            .map_err(|e| NeoLanError::Storage(format!("Failed to find messages by peer: {}", e)))?;

        Ok(result)
    }

    /// 查找两个节点之间的消息
    pub async fn find_conversation(
        &self,
        peer_ip1: &str,
        peer_ip2: &str,
        limit: u64,
    ) -> Result<Vec<MessageModel>> {
        let result = MessageEntity::find()
            .filter(
                Condition::any()
                    .add(
                        Condition::all()
                            .add(messages::Column::SenderIp.eq(peer_ip1))
                            .add(messages::Column::ReceiverIp.eq(peer_ip2)),
                    )
                    .add(
                        Condition::all()
                            .add(messages::Column::SenderIp.eq(peer_ip2))
                            .add(messages::Column::ReceiverIp.eq(peer_ip1)),
                    ),
            )
            .order_by_desc(messages::Column::SentAt)
            .limit(limit)
            .all(&self.db)
            .await
            .map_err(|e| NeoLanError::Storage(format!("Failed to find conversation: {}", e)))?;

        Ok(result)
    }

    /// 查找离线消息（未送达的消息）
    pub async fn find_offline_messages(&self, peer_ip: &str) -> Result<Vec<MessageModel>> {
        let result = MessageEntity::find()
            .filter(messages::Column::ReceiverIp.eq(peer_ip))
            .filter(messages::Column::IsOffline.eq(true))
            .order_by_asc(messages::Column::SentAt)
            .all(&self.db)
            .await
            .map_err(|e| NeoLanError::Storage(format!("Failed to find offline messages: {}", e)))?;

        Ok(result)
    }

    /// 查找所有离线消息
    pub async fn find_all_offline(&self) -> Result<Vec<MessageModel>> {
        let result = MessageEntity::find()
            .filter(messages::Column::IsOffline.eq(true))
            .order_by_asc(messages::Column::SentAt)
            .all(&self.db)
            .await
            .map_err(|e| NeoLanError::Storage(format!("Failed to find all offline messages: {}", e)))?;

        Ok(result)
    }

    /// 标记消息为已送达
    pub async fn mark_as_delivered(&self, msg_id: &str) -> Result<()> {
        let existing = self
            .find_by_msg_id(msg_id)
            .await?
            .ok_or_else(|| NeoLanError::Storage(format!("Message not found: {}", msg_id)))?;

        let mut active_model: MessageActiveModel = existing.into();
        active_model.is_offline = Set(false);
        active_model.received_at = Set(Some(chrono::Utc::now().naive_utc()));

        MessageEntity::update(active_model)
            .exec(&self.db)
            .await
            .map_err(|e| NeoLanError::Storage(format!("Failed to mark message as delivered: {}", e)))?;

        Ok(())
    }

    /// 批量标记离线消息为已送达
    ///
    /// # 参数
    /// - `peer_ip`: 接收方节点 IP
    ///
    /// # 返回
    /// 更新的消息数量
    pub async fn mark_peer_offline_delivered(&self, peer_ip: &str) -> Result<u64> {
        let now = chrono::Utc::now().naive_utc();

        // 先查找需要更新的消息
        let offline_messages = self.find_offline_messages(peer_ip).await?;

        let mut count = 0;
        for msg in offline_messages {
            let mut active_model: MessageActiveModel = msg.into();
            active_model.is_offline = Set(false);
            active_model.received_at = Set(Some(now));

            MessageEntity::update(active_model)
                .exec(&self.db)
                .await
                .map_err(|e| NeoLanError::Storage(format!("Failed to mark message as delivered: {}", e)))?;

            count += 1;
        }

        Ok(count)
    }

    /// 删除指定时间之前的消息
    ///
    /// # 参数
    /// - `before`: 删除此时间之前的消息
    ///
    /// # 返回
    /// 删除的消息数量
    pub async fn delete_old_messages(&self, before: NaiveDateTime) -> Result<u64> {
        let result = MessageEntity::delete_many()
            .filter(messages::Column::SentAt.lt(before))
            .exec(&self.db)
            .await
            .map_err(|e| NeoLanError::Storage(format!("Failed to delete old messages: {}", e)))?;

        Ok(result.rows_affected)
    }

    /// 删除与特定节点的所有消息
    pub async fn delete_by_peer(&self, peer_ip: &str) -> Result<u64> {
        let result = MessageEntity::delete_many()
            .filter(
                Condition::any()
                    .add(messages::Column::SenderIp.eq(peer_ip))
                    .add(messages::Column::ReceiverIp.eq(peer_ip)),
            )
            .exec(&self.db)
            .await
            .map_err(|e| NeoLanError::Storage(format!("Failed to delete messages by peer: {}", e)))?;

        Ok(result.rows_affected)
    }

    /// 根据 msg_id 删除消息
    pub async fn delete_by_msg_id(&self, msg_id: &str) -> Result<()> {
        let result = MessageEntity::delete_many()
            .filter(messages::Column::MsgId.eq(msg_id))
            .exec(&self.db)
            .await
            .map_err(|e| NeoLanError::Storage(format!("Failed to delete message: {}", e)))?;

        if result.rows_affected == 0 {
            return Err(NeoLanError::Storage(format!("Message not found: {}", msg_id)));
        }

        Ok(())
    }

    /// 查找所有消息
    pub async fn find_all(&self, limit: u64) -> Result<Vec<MessageModel>> {
        let result = MessageEntity::find()
            .order_by_desc(messages::Column::SentAt)
            .limit(limit)
            .all(&self.db)
            .await
            .map_err(|e| NeoLanError::Storage(format!("Failed to find all messages: {}", e)))?;

        Ok(result)
    }

    /// 统计消息数量
    pub async fn count(&self) -> Result<u64> {
        let count = MessageEntity::find()
            .count(&self.db)
            .await
            .map_err(|e| NeoLanError::Storage(format!("Failed to count messages: {}", e)))?;

        Ok(count)
    }

    /// 统计离线消息数量
    pub async fn count_offline(&self) -> Result<u64> {
        let count = MessageEntity::find()
            .filter(messages::Column::IsOffline.eq(true))
            .count(&self.db)
            .await
            .map_err(|e| NeoLanError::Storage(format!("Failed to count offline messages: {}", e)))?;

        Ok(count)
    }

    /// 统计与特定节点的消息数量
    pub async fn count_by_peer(&self, peer_ip: &str) -> Result<u64> {
        let count = MessageEntity::find()
            .filter(
                Condition::any()
                    .add(messages::Column::SenderIp.eq(peer_ip))
                    .add(messages::Column::ReceiverIp.eq(peer_ip)),
            )
            .count(&self.db)
            .await
            .map_err(|e| NeoLanError::Storage(format!("Failed to count messages by peer: {}", e)))?;

        Ok(count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_repo_creation() {
        // 测试 MessageRepository 创建
        // 注意：实际测试需要数据库连接，这里只是编译测试
        // 集成测试将在后续阶段实现
    }
}

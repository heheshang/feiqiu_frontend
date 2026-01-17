// src-tauri/src/storage/peer_repo.rs
use crate::error::{NeoLanError, Result};
use crate::storage::entities::peers;
use chrono::NaiveDateTime;
use sea_orm::*;

pub type PeerModel = peers::Model;
pub type PeerActiveModel = peers::ActiveModel;
pub type PeerEntity = peers::Entity;

/// 节点数据访问层
///
/// 提供 peers 表的 CRUD 操作
#[derive(Clone)]
pub struct PeerRepository {
    db: DatabaseConnection,
}

impl PeerRepository {
    /// 创建新的 PeerRepository
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// Insert or update peer by IP address
    ///
    /// If peer with the IP exists, updates it. Otherwise inserts a new peer.
    /// This method properly handles auto-increment for new records.
    /// Returns the created or updated peer.
    ///
    /// # Arguments
    /// * `ip` - IP address as string
    /// * `port` - UDP port
    /// * `user_id` - User unique ID (optional)
    /// * `username` - Username (optional)
    /// * `hostname` - Hostname (optional)
    /// * `last_seen` - Last activity timestamp
    pub async fn upsert(
        &self,
        ip: String,
        port: i32,
        user_id: Option<String>,
        username: Option<String>,
        hostname: Option<String>,
        last_seen: NaiveDateTime,
    ) -> Result<PeerModel> {
        use sea_orm::ActiveValue::NotSet;
        use sea_orm::ActiveValue::Set;

        if let Some(existing) = self.find_by_ip(&ip).await? {
            // Update existing peer
            let mut active: PeerActiveModel = existing.into();
            active.port = Set(port);
            active.user_id = Set(user_id);
            active.username = Set(username);
            active.hostname = Set(hostname);
            active.last_seen = Set(last_seen);
            active.updated_at = Set(Some(chrono::Utc::now().naive_utc()));

            PeerEntity::update(active)
                .exec(&self.db)
                .await
                .map_err(|e| NeoLanError::Storage(format!("Failed to update peer: {}", e)))?;

            // Return the updated peer (fetch again to get updated values)
            let updated = self
                .find_by_ip(&ip)
                .await?
                .ok_or_else(|| NeoLanError::Other("Failed to retrieve updated peer".to_string()))?;
            Ok(updated)
        } else {
            // Insert new peer with NotSet for id (auto-increment)
            let active = PeerActiveModel {
                id: NotSet,
                user_id: Set(user_id),
                ip: Set(ip.clone()),
                port: Set(port),
                username: Set(username),
                hostname: Set(hostname),
                nickname: Set(None),
                avatar: Set(None),
                groups: Set(None),
                last_seen: Set(last_seen),
                created_at: Set(chrono::Utc::now().naive_utc()),
                updated_at: Set(Some(chrono::Utc::now().naive_utc())),
            };

            PeerEntity::insert(active)
                .exec(&self.db)
                .await
                .map_err(|e| NeoLanError::Storage(format!("Failed to insert peer: {}", e)))?;

            // Fetch the inserted peer to get its ID
            let inserted = self.find_by_ip(&ip).await?.ok_or_else(|| {
                NeoLanError::Other("Failed to retrieve inserted peer".to_string())
            })?;
            Ok(inserted)
        }
    }

    /// 插入新节点
    ///
    /// 如果 IP 已存在，则更新现有记录
    pub async fn insert(&self, peer: &PeerModel) -> Result<()> {
        let active_model: PeerActiveModel = peer.clone().into();

        // 使用 on_insert 处理重复 IP 的情况
        // 先检查是否已存在
        if let Some(existing) = self.find_by_ip(&peer.ip).await? {
            // 如果已存在，更新它
            let mut active: PeerActiveModel = existing.into();
            active.port = Set(peer.port);
            active.username = Set(peer.username.clone());
            active.hostname = Set(peer.hostname.clone());
            active.nickname = Set(peer.nickname.clone());
            active.avatar = Set(peer.avatar.clone());
            active.groups = Set(peer.groups.clone());
            active.last_seen = Set(peer.last_seen);
            active.updated_at = Set(Some(chrono::Utc::now().naive_utc()));

            PeerEntity::update(active)
                .exec(&self.db)
                .await
                .map_err(|e| NeoLanError::Storage(format!("Failed to update peer: {}", e)))?;
        } else {
            // 插入新记录
            PeerEntity::insert(active_model)
                .exec(&self.db)
                .await
                .map_err(|e| NeoLanError::Storage(format!("Failed to insert peer: {}", e)))?;
        }

        Ok(())
    }

    /// 更新节点信息
    pub async fn update(&self, peer: &PeerModel) -> Result<()> {
        // 先检查节点是否存在
        let existing = self
            .find_by_ip(&peer.ip)
            .await?
            .ok_or_else(|| NeoLanError::PeerNotFound(peer.ip.clone()))?;

        let mut active_model: PeerActiveModel = existing.into();
        active_model.port = Set(peer.port);
        active_model.username = Set(peer.username.clone());
        active_model.hostname = Set(peer.hostname.clone());
        active_model.nickname = Set(peer.nickname.clone());
        active_model.avatar = Set(peer.avatar.clone());
        active_model.groups = Set(peer.groups.clone());
        active_model.last_seen = Set(peer.last_seen);
        active_model.updated_at = Set(Some(chrono::Utc::now().naive_utc()));

        PeerEntity::update(active_model)
            .exec(&self.db)
            .await
            .map_err(|e| NeoLanError::Storage(format!("Failed to update peer: {}", e)))?;

        Ok(())
    }

    /// 根据 IP 查找节点
    pub async fn find_by_ip(&self, ip: &str) -> Result<Option<PeerModel>> {
        let result = PeerEntity::find()
            .filter(peers::Column::Ip.eq(ip))
            .one(&self.db)
            .await
            .map_err(|e| NeoLanError::Storage(format!("Failed to find peer by IP: {}", e)))?;

        Ok(result)
    }

    /// 查找所有节点
    pub async fn find_all(&self) -> Result<Vec<PeerModel>> {
        let result = PeerEntity::find()
            .all(&self.db)
            .await
            .map_err(|e| NeoLanError::Storage(format!("Failed to find all peers: {}", e)))?;

        Ok(result)
    }

    /// 查找在线节点（最近 60 秒内有活动）
    pub async fn find_online(&self, timeout_seconds: i64) -> Result<Vec<PeerModel>> {
        let cutoff: NaiveDateTime = chrono::Utc::now()
            .checked_sub_signed(chrono::Duration::seconds(timeout_seconds))
            .unwrap()
            .naive_utc();

        let result = PeerEntity::find()
            .filter(peers::Column::LastSeen.gte(cutoff))
            .all(&self.db)
            .await
            .map_err(|e| NeoLanError::Storage(format!("Failed to find online peers: {}", e)))?;

        Ok(result)
    }

    /// 查找离线节点（超过指定时间未活动）
    pub async fn find_offline(&self, timeout_seconds: i64) -> Result<Vec<PeerModel>> {
        let cutoff: NaiveDateTime = chrono::Utc::now()
            .checked_sub_signed(chrono::Duration::seconds(timeout_seconds))
            .unwrap()
            .naive_utc();

        let result = PeerEntity::find()
            .filter(peers::Column::LastSeen.lt(cutoff))
            .all(&self.db)
            .await
            .map_err(|e| NeoLanError::Storage(format!("Failed to find offline peers: {}", e)))?;

        Ok(result)
    }

    /// 根据 IP 删除节点
    pub async fn delete_by_ip(&self, ip: &str) -> Result<()> {
        let result = PeerEntity::delete_many()
            .filter(peers::Column::Ip.eq(ip))
            .exec(&self.db)
            .await
            .map_err(|e| NeoLanError::Storage(format!("Failed to delete peer: {}", e)))?;

        if result.rows_affected == 0 {
            return Err(NeoLanError::PeerNotFound(ip.to_string()));
        }

        Ok(())
    }

    /// 清理离线节点（删除超过指定时间未活动的节点）
    pub async fn cleanup_offline(&self, timeout_seconds: i64) -> Result<u64> {
        let cutoff: NaiveDateTime = chrono::Utc::now()
            .checked_sub_signed(chrono::Duration::seconds(timeout_seconds))
            .unwrap()
            .naive_utc();

        let result = PeerEntity::delete_many()
            .filter(peers::Column::LastSeen.lt(cutoff))
            .exec(&self.db)
            .await
            .map_err(|e| NeoLanError::Storage(format!("Failed to cleanup offline peers: {}", e)))?;

        Ok(result.rows_affected)
    }

    /// 更新节点的 last_seen 时间戳
    pub async fn update_last_seen(&self, ip: &str) -> Result<()> {
        let existing = self
            .find_by_ip(ip)
            .await?
            .ok_or_else(|| NeoLanError::PeerNotFound(ip.to_string()))?;

        let mut active_model: PeerActiveModel = existing.into();
        active_model.last_seen = Set(chrono::Utc::now().naive_utc());
        active_model.updated_at = Set(Some(chrono::Utc::now().naive_utc()));

        PeerEntity::update(active_model)
            .exec(&self.db)
            .await
            .map_err(|e| NeoLanError::Storage(format!("Failed to update last_seen: {}", e)))?;

        Ok(())
    }
}

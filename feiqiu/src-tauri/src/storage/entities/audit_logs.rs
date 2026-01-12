// src-tauri/src/storage/entities/audit_logs.rs
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "audit_logs")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32,

    #[sea_orm(column_type = "Text")]
    pub event_type: String, // "peer_online", "peer_offline", "message_sent", etc.

    #[sea_orm(column_type = "Text", nullable)]
    pub peer_ip: Option<String>,

    #[sea_orm(column_type = "Text", nullable)]
    pub event_data: Option<String>, // JSON字符串

    #[sea_orm(column_type = "BigInteger")]
    pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

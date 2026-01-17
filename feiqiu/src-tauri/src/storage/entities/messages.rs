// src-tauri/src/storage/entities/messages.rs
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "messages")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32,

    /// User ID of the sender (for message association)
    #[sea_orm(column_type = "Text", nullable)]
    pub user_id: Option<String>,

    #[sea_orm(column_type = "Text", unique)]
    pub msg_id: String,

    #[sea_orm(column_type = "Text")]
    pub sender_ip: String,

    #[sea_orm(column_type = "Text")]
    pub sender_name: String,

    #[sea_orm(column_type = "Text")]
    pub receiver_ip: String,

    #[sea_orm(column_type = "Integer")]
    pub msg_type: i32,

    #[sea_orm(column_type = "Text")]
    pub content: String,

    #[sea_orm(column_type = "Boolean", default_value = "false")]
    pub is_encrypted: bool,

    #[sea_orm(column_type = "Boolean", default_value = "false")]
    pub is_offline: bool,

    #[sea_orm(column_type = "BigInteger")]
    pub sent_at: DateTime,

    #[sea_orm(column_type = "BigInteger", nullable)]
    pub received_at: Option<DateTime>,

    #[sea_orm(column_type = "BigInteger")]
    pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

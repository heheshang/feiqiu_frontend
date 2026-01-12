// src-tauri/src/storage/entities/transfers.rs
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "transfers")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32,

    #[sea_orm(column_type = "Text", unique)]
    pub task_id: String,

    #[sea_orm(column_type = "Text")]
    pub direction: String, // "upload" or "download"

    #[sea_orm(column_type = "Text")]
    pub file_name: String,

    #[sea_orm(column_type = "BigInteger")]
    pub file_size: i64,

    #[sea_orm(column_type = "Text")]
    pub file_md5: String,

    #[sea_orm(column_type = "Text")]
    pub peer_ip: String,

    #[sea_orm(column_type = "Text")]
    pub peer_name: String,

    #[sea_orm(column_type = "Text")]
    pub status: String, // "pending", "transferring", "paused", "completed", "failed", "cancelled"

    #[sea_orm(column_type = "BigInteger", default_value = "0")]
    pub transferred_size: i64,

    #[sea_orm(column_type = "BigInteger", nullable)]
    pub started_at: Option<DateTime>,

    #[sea_orm(column_type = "BigInteger", nullable)]
    pub completed_at: Option<DateTime>,

    #[sea_orm(column_type = "BigInteger")]
    pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

// src-tauri/src/storage/entities/contacts.rs
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "contacts")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32,

    #[sea_orm(column_type = "Integer", nullable)]
    pub peer_id: Option<i32>,

    #[sea_orm(column_type = "Text")]
    pub is_online: bool,

    #[sea_orm(column_type = "Text")]
    pub peer_ip: Option<String>,

    #[sea_orm(column_type = "BigInteger")]
    pub last_seen: Option<DateTime>,

    #[sea_orm(column_type = "BigInteger")]
    pub created_at: DateTime,

    #[sea_orm(column_type = "BigInteger", nullable)]
    pub updated_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

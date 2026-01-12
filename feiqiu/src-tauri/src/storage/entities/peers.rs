// src-tauri/src/storage/entities/peers.rs
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "peers")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32,

    #[sea_orm(column_type = "Text")]
    pub ip: String,

    #[sea_orm(column_type = "Integer")]
    pub port: i32,

    #[sea_orm(column_type = "Text", nullable)]
    pub username: Option<String>,

    #[sea_orm(column_type = "Text", nullable)]
    pub hostname: Option<String>,

    #[sea_orm(column_type = "Text", nullable)]
    pub nickname: Option<String>,

    #[sea_orm(column_type = "Text", nullable)]
    pub avatar: Option<String>,

    #[sea_orm(column_type = "Text", nullable)]
    pub groups: Option<String>, // JSON字符串: "[\"tech-team\", \"friends\"]"

    #[sea_orm(column_type = "BigInteger")]
    pub last_seen: DateTime,

    #[sea_orm(column_type = "BigInteger")]
    pub created_at: DateTime,

    #[sea_orm(column_type = "BigInteger", nullable)]
    pub updated_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

// src-tauri/src/storage/entities/conversation_participants.rs
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "conversation_participants")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32,

    #[sea_orm(column_type = "Integer")]
    pub conversation_id: i32,

    #[sea_orm(column_type = "Text")]
    pub peer_ip: String,

    #[sea_orm(column_type = "BigInteger")]
    pub joined_at: DateTime,

    #[sea_orm(column_type = "BigInteger", nullable)]
    pub left_at: Option<DateTime>,

    #[sea_orm(column_type = "Text", default_value = "'member'")]
    pub role: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::conversations::Entity",
        from = "Column::ConversationId",
        to = "super::conversations::Column::Id",
        on_delete = "Cascade"
    )]
    Conversations,
}

impl Related<super::conversations::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Conversations.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

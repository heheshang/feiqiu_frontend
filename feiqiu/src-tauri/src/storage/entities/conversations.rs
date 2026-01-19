// src-tauri/src/storage/entities/conversations.rs
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "conversations")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32,

    /// Conversation type: 'single' or 'group'
    #[sea_orm(column_type = "Text")]
    pub r#type: String,

    #[sea_orm(column_type = "BigInteger")]
    pub created_at: DateTime,

    #[sea_orm(column_type = "BigInteger")]
    pub updated_at: DateTime,

    #[sea_orm(column_type = "Boolean", default_value = "false")]
    pub is_pinned: bool,

    #[sea_orm(column_type = "Boolean", default_value = "false")]
    pub is_archived: bool,

    #[sea_orm(column_type = "Boolean", default_value = "false")]
    pub is_muted: bool,

    #[sea_orm(column_type = "Integer", default_value = "0")]
    pub unread_count: i32,

    #[sea_orm(column_type = "Integer", nullable)]
    pub last_message_id: Option<i32>,

    #[sea_orm(column_type = "BigInteger", nullable)]
    pub last_message_at: Option<DateTime>,

    #[sea_orm(column_type = "Text", nullable)]
    pub last_message_content: Option<String>,

    #[sea_orm(column_type = "Text", nullable)]
    pub last_message_type: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        has_many = "super::conversation_participants::Entity"
    )]
    ConversationParticipants,
}

impl Related<super::conversation_participants::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ConversationParticipants.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

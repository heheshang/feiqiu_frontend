// src-tauri/src/storage/entities/contact_groups.rs
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "contact_groups")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32,

    #[sea_orm(column_type = "Text")]
    pub name: String,

    #[sea_orm(column_type = "Text", nullable)]
    pub color: Option<String>,

    #[sea_orm(column_type = "Text", nullable)]
    pub icon: Option<String>,

    #[sea_orm(column_type = "Integer")]
    pub sort_order: i32,

    #[sea_orm(column_type = "BigInteger")]
    pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::contact_group_members::Entity")]
    ContactGroupMembers,
}

impl Related<super::contact_group_members::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ContactGroupMembers.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

// src-tauri/src/storage/entities/groups.rs
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "groups")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32,

    #[sea_orm(column_type = "Text", unique)]
    pub name: String,

    #[sea_orm(column_type = "Text", nullable)]
    pub color: Option<String>,

    #[sea_orm(column_type = "Integer", default_value = "0")]
    pub sort_order: i32,

    #[sea_orm(column_type = "BigInteger")]
    pub created_at: DateTime,

    #[sea_orm(column_type = "BigInteger")]
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

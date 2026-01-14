// src-tauri/src/storage/entities/contact_group_members.rs
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "contact_group_members")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32,

    #[sea_orm(column_type = "Integer")]
    pub contact_id: i32,

    #[sea_orm(column_type = "Integer")]
    pub group_id: i32,

    #[sea_orm(column_type = "BigInteger")]
    pub joined_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::contacts::Entity",
        from = "Column::ContactId",
        to = "super::contacts::Column::Id"
    )]
    Contact,

    #[sea_orm(
        belongs_to = "super::contact_groups::Entity",
        from = "Column::GroupId",
        to = "super::contact_groups::Column::Id"
    )]
    Group,
}

impl Related<super::contacts::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Contact.def()
    }
}

impl Related<super::contact_groups::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Group.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

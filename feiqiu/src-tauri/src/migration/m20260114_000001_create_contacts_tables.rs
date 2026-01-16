// src-tauri/src/migration/m20260114_000001_create_contacts_tables.rs
use sea_orm_migration::prelude::*;
use sea_orm_migration::schema::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 创建 contacts 表
        manager
            .create_table(
                Table::create()
                    .table(Contacts::Table)
                    .comment("")
                    .if_not_exists()
                    .col(pk_auto(Contacts::Id))
                    .comment("id")
                    .col(integer_null(Contacts::PeerId))
                    .col(string(Contacts::Name))
                    .col(string_null(Contacts::Nickname))
                    .col(string_null(Contacts::Avatar))
                    .col(string_null(Contacts::Phone))
                    .col(string_null(Contacts::Email))
                    .col(string_null(Contacts::Department))
                    .col(string_null(Contacts::Position))
                    .col(string_null(Contacts::Notes))
                    .col(boolean(Contacts::IsFavorite).default(false))
                    .col(string_null(Contacts::Pinyin))
                    .col(boolean(Contacts::IsOnline).default(true))
                    .col(timestamp_null(Contacts::LastSeen))
                    .col(timestamp(Contacts::CreatedAt))
                    .col(timestamp_null(Contacts::UpdatedAt))
                    .to_owned(),
            )
            .await?;

        // 创建 contacts 表索引
        manager
            .create_index(
                Index::create()
                    .name("idx_contacts_name")
                    .table(Contacts::Table)
                    .col(Contacts::Name)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_contacts_pinyin")
                    .table(Contacts::Table)
                    .col(Contacts::Pinyin)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_contacts_department")
                    .table(Contacts::Table)
                    .col(Contacts::Department)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_contacts_peer_id")
                    .table(Contacts::Table)
                    .col(Contacts::PeerId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_contacts_is_favorite")
                    .table(Contacts::Table)
                    .col(Contacts::IsFavorite)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_contacts_is_online")
                    .table(Contacts::Table)
                    .col(Contacts::IsOnline)
                    .to_owned(),
            )
            .await?;

        // 创建 contact_groups 表
        manager
            .create_table(
                Table::create()
                    .table(ContactGroups::Table)
                    .if_not_exists()
                    .col(pk_auto(ContactGroups::Id))
                    .col(string(ContactGroups::Name))
                    .col(string_null(ContactGroups::Color))
                    .col(string_null(ContactGroups::Icon))
                    .col(integer(ContactGroups::SortOrder).default(0))
                    .col(timestamp(ContactGroups::CreatedAt))
                    .to_owned(),
            )
            .await?;

        // 创建 contact_groups 表索引
        manager
            .create_index(
                Index::create()
                    .name("idx_contact_groups_name")
                    .table(ContactGroups::Table)
                    .col(ContactGroups::Name)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_contact_groups_sort_order")
                    .table(ContactGroups::Table)
                    .col(ContactGroups::SortOrder)
                    .to_owned(),
            )
            .await?;

        // 创建 contact_group_members 表
        // Note: Foreign keys removed due to sea-orm bug with enum-based table references
        // The ContactsRef and ContactGroupsRef enums generate "contacts_ref" and "contact_groups_ref"
        // which don't exist. Application-level validation handles referential integrity.
        manager
            .create_table(
                Table::create()
                    .table(ContactGroupMembers::Table)
                    .if_not_exists()
                    .col(pk_auto(ContactGroupMembers::Id))
                    .col(integer(ContactGroupMembers::ContactId))
                    .col(integer(ContactGroupMembers::GroupId))
                    .col(timestamp(ContactGroupMembers::JoinedAt))
                    .to_owned(),
            )
            .await?;

        // 创建 contact_group_members 表索引
        manager
            .create_index(
                Index::create()
                    .name("idx_contact_group_members_contact_id")
                    .table(ContactGroupMembers::Table)
                    .col(ContactGroupMembers::ContactId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_contact_group_members_group_id")
                    .table(ContactGroupMembers::Table)
                    .col(ContactGroupMembers::GroupId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_contact_group_members_contact_group")
                    .table(ContactGroupMembers::Table)
                    .col(ContactGroupMembers::ContactId)
                    .col(ContactGroupMembers::GroupId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 按相反顺序删除表
        manager
            .drop_table(Table::drop().table(ContactGroupMembers::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(ContactGroups::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Contacts::Table).to_owned())
            .await?;

        Ok(())
    }
}

// Contacts 表枚举
#[derive(DeriveIden)]
enum Contacts {
    Table,
    Id,
    PeerId,
    Name,
    Nickname,
    Avatar,
    Phone,
    Email,
    Department,
    Position,
    Notes,
    IsFavorite,
    Pinyin,
    IsOnline,
    LastSeen,
    CreatedAt,
    UpdatedAt,
}

// ContactGroups 表枚举
#[derive(DeriveIden)]
enum ContactGroups {
    Table,
    Id,
    Name,
    Color,
    Icon,
    SortOrder,
    CreatedAt,
}

// ContactGroupMembers 表枚举
#[derive(DeriveIden)]
enum ContactGroupMembers {
    Table,
    Id,
    ContactId,
    GroupId,
    JoinedAt,
}

// src-tauri/src/migration/m20260114_000001_add_peer_ip_to_contacts.rs
//
// Migration to add peer_ip column to contacts table
// This allows linking contacts table with peers table by IP address
//
// Reason: When receiving a message from a new peer, we need to check if the sender
// is already in contacts. Previously we could only match by name, but now we can also
// match by IP address which is a more reliable unique identifier.

use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260114_000001_add_peer_ip_to_contacts"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::new()
                    .table(Contacts::Table)
                    .add_column(ColumnDef::new(
                            contacts::PeerIp,
                            Alias::new("peer_ip"),
                            ColumnType::Text,
                            true,
                    ))
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::from(Contacts::Table))
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum Contacts {
    pub const Table: TableRef = TableRef;
}

#[derive(Iden)]
pub enum ContactsColumns {
    pub const Id: Iden = "id";
    pub const Name: Iden = "name";
    pub const Nickname: Iden = "nickname";
    pub const Avatar: Iden = "avatar";
    pub const Phone: Iden = "phone";
    pub const Email: Iden = "email";
    pub const Department: Iden = "department";
    pub const Position: Iden = "position";
    pub const Notes: Iden = "notes";
    pub const IsFavorite: Iden = "is_favorite";
    pub const Pinyin: Iden = "pinyin";
    pub const IsOnline: Iden = "is_online";
    pub const LastSeen: Iden = "last_seen";
    pub const CreatedAt: Iden = "created_at";
    pub const UpdatedAt: Iden = "updated_at";
    pub const PeerId: Iden = "peer_id";
    pub const Groups: Iden = "groups";
    pub const PeerIp: Iden = "peer_ip";
}

#[derive(DeriveIden)]
pub enum ContactsTable {
    pub const Table: TableRef = TableRef;
}

#[derive(Iden)]
pub enum PeerIp {
    pub const Id: Iden = "peer_ip";
}

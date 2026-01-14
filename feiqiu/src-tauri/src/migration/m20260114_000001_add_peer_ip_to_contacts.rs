// src-tauri/src/migration/m20260114_000001_add_peer_ip_to_contacts.rs
//
// Migration to add peer_ip column to contacts table
// This allows linking contacts table with peers table by IP address

use sea_orm_migration::prelude::*;
use sea_orm_migration::schema::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260114_000001_add_peer_ip_to_contacts"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Add peer_ip column to contacts table
        manager
            .alter_table(
                Table::alter()
                    .table(Contacts::Table)
                    .add_column(string_null(Contacts::PeerIp))
                    .to_owned(),
            )
            .await?;

        // Create index on peer_ip for faster lookups
        manager
            .create_index(
                Index::create()
                    .name("idx_contacts_peer_ip")
                    .table(Contacts::Table)
                    .col(Contacts::PeerIp)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Remove the peer_ip column
        manager
            .alter_table(
                Table::alter()
                    .table(Contacts::Table)
                    .drop_column(Contacts::PeerIp)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(Iden)]
enum Contacts {
    Table,
    PeerIp,
}

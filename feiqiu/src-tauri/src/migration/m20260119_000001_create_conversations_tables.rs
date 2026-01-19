// src-tauri/src/migration/m20260119_000001_create_conversations_tables.rs
use sea_orm_migration::prelude::*;
use sea_orm_migration::schema::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create conversations table
        manager
            .create_table(
                Table::create()
                    .table(Conversations::Table)
                    .comment("")
                    .if_not_exists()
                    .col(pk_auto(Conversations::Id))
                    .comment("id")
                    .col(string(Conversations::Type))
                    .col(timestamp(Conversations::CreatedAt))
                    .col(timestamp(Conversations::UpdatedAt))
                    .col(boolean(Conversations::IsPinned).default(false))
                    .col(boolean(Conversations::IsArchived).default(false))
                    .col(boolean(Conversations::IsMuted).default(false))
                    .col(integer(Conversations::UnreadCount).default(0))
                    .col(integer_null(Conversations::LastMessageId))
                    .col(timestamp_null(Conversations::LastMessageAt))
                    .col(string_null(Conversations::LastMessageContent))
                    .col(string_null(Conversations::LastMessageType))
                    .to_owned(),
            )
            .await?;

        // Create indexes for conversations table
        manager
            .create_index(
                Index::create()
                    .name("idx_conversations_type")
                    .table(Conversations::Table)
                    .col(Conversations::Type)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_conversations_updated_at")
                    .table(Conversations::Table)
                    .col(Conversations::UpdatedAt)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_conversations_is_pinned")
                    .table(Conversations::Table)
                    .col(Conversations::IsPinned)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_conversations_is_archived")
                    .table(Conversations::Table)
                    .col(Conversations::IsArchived)
                    .to_owned(),
            )
            .await?;

        // Create conversation_participants table
        manager
            .create_table(
                Table::create()
                    .table(ConversationParticipants::Table)
                    .if_not_exists()
                    .col(pk_auto(ConversationParticipants::Id))
                    .col(integer(ConversationParticipants::ConversationId))
                    .col(string(ConversationParticipants::PeerIp))
                    .col(timestamp(ConversationParticipants::JoinedAt))
                    .col(timestamp_null(ConversationParticipants::LeftAt))
                    .col(string(ConversationParticipants::Role).default("member"))
                    .to_owned(),
            )
            .await?;

        // Create indexes for conversation_participants table
        manager
            .create_index(
                Index::create()
                    .name("idx_conversation_participants_conversation_id")
                    .table(ConversationParticipants::Table)
                    .col(ConversationParticipants::ConversationId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_conversation_participants_peer_ip")
                    .table(ConversationParticipants::Table)
                    .col(ConversationParticipants::PeerIp)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_conversation_participants_conversation_peer")
                    .table(ConversationParticipants::Table)
                    .col(ConversationParticipants::ConversationId)
                    .col(ConversationParticipants::PeerIp)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop tables in reverse order
        manager
            .drop_table(
                Table::drop()
                    .table(ConversationParticipants::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(Conversations::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Conversations {
    Table,
    Id,
    Type,
    CreatedAt,
    UpdatedAt,
    IsPinned,
    IsArchived,
    IsMuted,
    UnreadCount,
    LastMessageId,
    LastMessageAt,
    LastMessageContent,
    LastMessageType,
}

#[derive(DeriveIden)]
enum ConversationParticipants {
    Table,
    Id,
    ConversationId,
    PeerIp,
    JoinedAt,
    LeftAt,
    Role,
}

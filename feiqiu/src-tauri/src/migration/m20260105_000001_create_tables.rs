// src-tauri/src/migration/m20260105_000001_create_tables.rs
use sea_orm_migration::prelude::*;
use sea_orm_migration::schema::*;

#[derive(DeriveMigrationName)]
#[allow(dead_code)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 创建 peers 表
        manager
            .create_table(
                Table::create()
                    .table(Peers::Table)
                    .if_not_exists()
                    .col(pk_auto(Peers::Id))
                    .col(string(Peers::Ip))
                    .col(integer(Peers::Port))
                    .col(string_null(Peers::Username))
                    .col(string_null(Peers::Hostname))
                    .col(string_null(Peers::Nickname))
                    .col(string_null(Peers::Avatar))
                    .col(string_null(Peers::Groups))
                    .col(timestamp(Peers::LastSeen))
                    .col(timestamp(Peers::CreatedAt))
                    .col(timestamp_null(Peers::UpdatedAt))
                    .to_owned(),
            )
            .await?;

        // 创建 peers 表索引
        manager
            .create_index(
                Index::create()
                    .name("idx_peers_ip")
                    .table(Peers::Table)
                    .col(Peers::Ip)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_peers_last_seen")
                    .table(Peers::Table)
                    .col(Peers::LastSeen)
                    .to_owned(),
            )
            .await?;

        // 创建 messages 表
        manager
            .create_table(
                Table::create()
                    .table(Messages::Table)
                    .if_not_exists()
                    .col(pk_auto(Messages::Id))
                    .col(string(Messages::MsgId).unique_key())
                    .col(string(Messages::SenderIp))
                    .col(string(Messages::SenderName))
                    .col(string(Messages::ReceiverIp))
                    .col(integer(Messages::MsgType))
                    .col(string(Messages::Content))
                    .col(boolean(Messages::IsEncrypted).default(false))
                    .col(boolean(Messages::IsOffline).default(false))
                    .col(timestamp(Messages::SentAt))
                    .col(timestamp_null(Messages::ReceivedAt))
                    .col(timestamp(Messages::CreatedAt))
                    .to_owned(),
            )
            .await?;

        // 创建 messages 表索引
        manager
            .create_index(
                Index::create()
                    .name("idx_messages_sender")
                    .table(Messages::Table)
                    .col(Messages::SenderIp)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_messages_receiver")
                    .table(Messages::Table)
                    .col(Messages::ReceiverIp)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_messages_sent_at")
                    .table(Messages::Table)
                    .col(Messages::SentAt)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_messages_msg_type")
                    .table(Messages::Table)
                    .col(Messages::MsgType)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_messages_is_offline")
                    .table(Messages::Table)
                    .col(Messages::IsOffline)
                    .to_owned(),
            )
            .await?;

        // 创建 transfers 表
        manager
            .create_table(
                Table::create()
                    .table(Transfers::Table)
                    .if_not_exists()
                    .col(pk_auto(Transfers::Id))
                    .col(string(Transfers::TaskId).unique_key())
                    .col(string(Transfers::Direction))
                    .col(string(Transfers::FileName))
                    .col(big_integer(Transfers::FileSize))
                    .col(string(Transfers::FileMd5))
                    .col(string(Transfers::PeerIp))
                    .col(string(Transfers::PeerName))
                    .col(string(Transfers::Status))
                    .col(big_integer(Transfers::TransferredSize).default(0))
                    .col(timestamp_null(Transfers::StartedAt))
                    .col(timestamp_null(Transfers::CompletedAt))
                    .col(timestamp(Transfers::CreatedAt))
                    .to_owned(),
            )
            .await?;

        // 创建 transfers 表索引
        manager
            .create_index(
                Index::create()
                    .name("idx_transfers_task_id")
                    .table(Transfers::Table)
                    .col(Transfers::TaskId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_transfers_peer_ip")
                    .table(Transfers::Table)
                    .col(Transfers::PeerIp)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_transfers_status")
                    .table(Transfers::Table)
                    .col(Transfers::Status)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_transfers_created_at")
                    .table(Transfers::Table)
                    .col(Transfers::CreatedAt)
                    .to_owned(),
            )
            .await?;

        // 创建 groups 表
        manager
            .create_table(
                Table::create()
                    .table(Groups::Table)
                    .if_not_exists()
                    .col(pk_auto(Groups::Id))
                    .col(string(Groups::Name).unique_key())
                    .col(string_null(Groups::Color))
                    .col(integer(Groups::SortOrder).default(0))
                    .col(timestamp(Groups::CreatedAt))
                    .col(timestamp(Groups::UpdatedAt))
                    .to_owned(),
            )
            .await?;

        // 创建 groups 表索引
        manager
            .create_index(
                Index::create()
                    .name("idx_groups_sort_order")
                    .table(Groups::Table)
                    .col(Groups::SortOrder)
                    .to_owned(),
            )
            .await?;

        // 创建 settings 表
        manager
            .create_table(
                Table::create()
                    .table(Settings::Table)
                    .if_not_exists()
                    .col(string(Settings::Key).primary_key())
                    .col(string(Settings::Value))
                    .col(timestamp(Settings::UpdatedAt))
                    .to_owned(),
            )
            .await?;

        // 创建 audit_logs 表
        manager
            .create_table(
                Table::create()
                    .table(AuditLogs::Table)
                    .if_not_exists()
                    .col(pk_auto(AuditLogs::Id))
                    .col(string(AuditLogs::EventType))
                    .col(string_null(AuditLogs::PeerIp))
                    .col(string_null(AuditLogs::EventData))
                    .col(timestamp(AuditLogs::CreatedAt))
                    .to_owned(),
            )
            .await?;

        // 创建 audit_logs 表索引
        manager
            .create_index(
                Index::create()
                    .name("idx_audit_logs_event_type")
                    .table(AuditLogs::Table)
                    .col(AuditLogs::EventType)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_audit_logs_peer_ip")
                    .table(AuditLogs::Table)
                    .col(AuditLogs::PeerIp)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_audit_logs_created_at")
                    .table(AuditLogs::Table)
                    .col(AuditLogs::CreatedAt)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 按相反顺序删除表
        manager
            .drop_table(Table::drop().table(AuditLogs::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Settings::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Groups::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Transfers::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Messages::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Peers::Table).to_owned())
            .await?;

        Ok(())
    }
}

// 表和字段枚举定义
#[derive(DeriveIden)]
#[allow(dead_code)]
enum Peers {
    Table,
    Id,
    Ip,
    Port,
    Username,
    Hostname,
    Nickname,
    Avatar,
    Groups,
    LastSeen,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
#[allow(dead_code)]
enum Messages {
    Table,
    Id,
    MsgId,
    SenderIp,
    SenderName,
    ReceiverIp,
    MsgType,
    Content,
    IsEncrypted,
    IsOffline,
    SentAt,
    ReceivedAt,
    CreatedAt,
}

#[derive(DeriveIden)]
#[allow(dead_code)]
enum Transfers {
    Table,
    Id,
    TaskId,
    Direction,
    FileName,
    FileSize,
    FileMd5,
    PeerIp,
    PeerName,
    Status,
    TransferredSize,
    StartedAt,
    CompletedAt,
    CreatedAt,
}

#[derive(DeriveIden)]
#[allow(dead_code)]
enum Groups {
    Table,
    Id,
    Name,
    Color,
    SortOrder,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
#[allow(dead_code)]
enum Settings {
    Table,
    Key,
    Value,
    UpdatedAt,
}

#[derive(DeriveIden)]
#[allow(dead_code)]
enum AuditLogs {
    Table,
    Id,
    EventType,
    PeerIp,
    EventData,
    CreatedAt,
}

// src-tauri/src/migration/m20260110_000001_add_composite_indexes.rs
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
#[allow(dead_code)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 添加 messages 表的复合索引
        // 这些索引优化了常见的查询模式：
        // 1. find_conversation: (sender_ip, receiver_ip, sent_at)
        // 2. find_by_peer: (sender_ip, sent_at), (receiver_ip, sent_at)
        // 3. find_offline_messages: (receiver_ip, is_offline, sent_at)

        // 复合索引: (sender_ip, receiver_ip, sent_at) - 优化对话查询
        manager
            .create_index(
                Index::create()
                    .name("idx_messages_sender_receiver_sent")
                    .table(Messages::Table)
                    .col(Messages::SenderIp)
                    .col(Messages::ReceiverIp)
                    .col(Messages::SentAt)
                    .to_owned(),
            )
            .await?;

        // 复合索引: (receiver_ip, sender_ip, sent_at) - 优化反向对话查询
        manager
            .create_index(
                Index::create()
                    .name("idx_messages_receiver_sender_sent")
                    .table(Messages::Table)
                    .col(Messages::ReceiverIp)
                    .col(Messages::SenderIp)
                    .col(Messages::SentAt)
                    .to_owned(),
            )
            .await?;

        // 复合索引: (receiver_ip, is_offline, sent_at) - 优化离线消息查询
        manager
            .create_index(
                Index::create()
                    .name("idx_messages_receiver_offline_sent")
                    .table(Messages::Table)
                    .col(Messages::ReceiverIp)
                    .col(Messages::IsOffline)
                    .col(Messages::SentAt)
                    .to_owned(),
            )
            .await?;

        // 添加 transfers 表的复合索引
        // 优化按状态和创建时间查询
        manager
            .create_index(
                Index::create()
                    .name("idx_transfers_status_created")
                    .table(Transfers::Table)
                    .col(Transfers::Status)
                    .col(Transfers::CreatedAt)
                    .to_owned(),
            )
            .await?;

        // 添加 peers 表的复合索引
        // 按在线状态排序的查询
        manager
            .create_index(
                Index::create()
                    .name("idx_peers_online_status")
                    .table(Peers::Table)
                    .col(Peers::LastSeen)
                    .col(Peers::Ip)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 按相反顺序删除索引
        manager
            .drop_index(Index::drop().name("idx_peers_online_status").to_owned())
            .await?;
        manager
            .drop_index(
                Index::drop()
                    .name("idx_transfers_status_created")
                    .to_owned(),
            )
            .await?;
        manager
            .drop_index(
                Index::drop()
                    .name("idx_messages_receiver_offline_sent")
                    .to_owned(),
            )
            .await?;
        manager
            .drop_index(
                Index::drop()
                    .name("idx_messages_receiver_sender_sent")
                    .to_owned(),
            )
            .await?;
        manager
            .drop_index(
                Index::drop()
                    .name("idx_messages_sender_receiver_sent")
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

// 表和字段枚举定义 (需要从主迁移文件导入或重新定义)
#[derive(DeriveIden)]
#[allow(dead_code)]
enum Messages {
    Table,
    SenderIp,
    ReceiverIp,
    SentAt,
    IsOffline,
}

#[derive(DeriveIden)]
#[allow(dead_code)]
enum Transfers {
    Table,
    Status,
    CreatedAt,
}

#[derive(DeriveIden)]
#[allow(dead_code)]
enum Peers {
    Table,
    LastSeen,
    Ip,
}

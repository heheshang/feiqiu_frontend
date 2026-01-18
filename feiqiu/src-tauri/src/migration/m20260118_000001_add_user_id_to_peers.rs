// src-tauri/src/migration/m20260118_000001_add_user_id_to_peers.rs
//
// Migration to add user_id column to peers table
// The entity model expects user_id but it was missing from the initial migration

use sea_orm::{ConnectionTrait, DbBackend, Statement};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // Add user_id column to peers table (nullable, TEXT type)
        db.execute(Statement::from_string(
            DbBackend::Sqlite,
            "ALTER TABLE peers ADD COLUMN user_id TEXT NULL;".to_string(),
        ))
        .await?;

        // Create index on user_id for faster lookups
        db.execute(Statement::from_string(
            DbBackend::Sqlite,
            "CREATE INDEX IF NOT EXISTS idx_peers_user_id ON peers (user_id);".to_string(),
        ))
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // SQLite doesn't support DROP COLUMN directly in older versions
        // We'll recreate the table without the column
        db.execute(Statement::from_string(
            DbBackend::Sqlite,
            r#"
            CREATE TABLE peers_new (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                ip VARCHAR NOT NULL,
                port INTEGER NOT NULL,
                username VARCHAR NULL,
                hostname VARCHAR NULL,
                nickname VARCHAR NULL,
                avatar VARCHAR NULL,
                groups VARCHAR NULL,
                last_seen TIMESTAMP_TEXT NOT NULL,
                created_at TIMESTAMP_TEXT NOT NULL,
                updated_at TIMESTAMP_TEXT NULL
            );
            "#
            .to_string(),
        ))
        .await?;

        // Copy data without user_id
        db.execute(Statement::from_string(
            DbBackend::Sqlite,
            r#"
            INSERT INTO peers_new (id, ip, port, username, hostname, nickname, avatar, groups, last_seen, created_at, updated_at)
            SELECT id, ip, port, username, hostname, nickname, avatar, groups, last_seen, created_at, updated_at
            FROM peers;
            "#.to_string(),
        ))
        .await?;

        // Drop old table
        db.execute(Statement::from_string(
            DbBackend::Sqlite,
            "DROP TABLE peers;".to_string(),
        ))
        .await?;

        // Rename new table
        db.execute(Statement::from_string(
            DbBackend::Sqlite,
            "ALTER TABLE peers_new RENAME TO peers;".to_string(),
        ))
        .await?;

        // Recreate indexes
        db.execute(Statement::from_string(
            DbBackend::Sqlite,
            r#"
            CREATE INDEX IF NOT EXISTS idx_peers_ip ON peers (ip);
            CREATE INDEX IF NOT EXISTS idx_peers_last_seen ON peers (last_seen);
            "#
            .to_string(),
        ))
        .await?;

        Ok(())
    }
}

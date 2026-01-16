// src-tauri/src/migration/m20260116_000001_remove_invalid_contacts_foreign_key.rs
//
// Migration to remove invalid foreign key constraints from contacts and contact_group_members tables
// The old migration incorrectly created:
//   - FOREIGN KEY (peer_id) REFERENCES peers_ref (id)
//   - FOREIGN KEY (contact_id) REFERENCES contacts_ref (id)
//   - FOREIGN KEY (group_id) REFERENCES contact_groups_ref (id)
// Since these ref tables don't exist, INSERT operations fail silently

use sea_orm_migration::prelude::*;
use sea_orm::{ConnectionTrait, Statement, DbBackend};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // Fix contacts table
        fix_contacts_table(db).await?;

        // Fix contact_group_members table
        fix_contact_group_members_table(db).await?;

        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        // No need to rollback - we're removing invalid constraints
        Ok(())
    }
}

async fn fix_contacts_table(db: &dyn ConnectionTrait) -> Result<(), DbErr> {
    // Step 1: Create a new contacts table without the invalid foreign key
    db.execute(Statement::from_string(
        DbBackend::Sqlite,
        r#"
        CREATE TABLE contacts_new (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            peer_id INTEGER NULL,
            name VARCHAR NOT NULL,
            nickname VARCHAR NULL,
            avatar VARCHAR NULL,
            phone VARCHAR NULL,
            email VARCHAR NULL,
            department VARCHAR NULL,
            position VARCHAR NULL,
            notes VARCHAR NULL,
            is_favorite BOOLEAN NOT NULL DEFAULT FALSE,
            pinyin VARCHAR NULL,
            is_online BOOLEAN NOT NULL DEFAULT TRUE,
            last_seen TIMESTAMP_TEXT NULL,
            created_at TIMESTAMP_TEXT NOT NULL,
            updated_at TIMESTAMP_TEXT NULL,
            peer_ip VARCHAR NULL
        );
        "#.to_string(),
    )).await?;

    // Step 2: Copy data from old contacts table to new one
    db.execute(Statement::from_string(
        DbBackend::Sqlite,
        r#"
        INSERT INTO contacts_new
        SELECT id, peer_id, name, nickname, avatar, phone, email, department, position, notes,
               is_favorite, pinyin, is_online, last_seen, created_at, updated_at, peer_ip
        FROM contacts;
        "#.to_string(),
    )).await?;

    // Step 3: Drop the old contacts table
    db.execute(Statement::from_string(
        DbBackend::Sqlite,
        "DROP TABLE contacts;".to_string(),
    )).await?;

    // Step 4: Rename the new table to contacts
    db.execute(Statement::from_string(
        DbBackend::Sqlite,
        "ALTER TABLE contacts_new RENAME TO contacts;".to_string(),
    )).await?;

    // Step 5: Recreate indexes
    db.execute(Statement::from_string(
        DbBackend::Sqlite,
        r#"
        CREATE INDEX idx_contacts_name ON contacts (name);
        CREATE INDEX idx_contacts_pinyin ON contacts (pinyin);
        CREATE INDEX idx_contacts_department ON contacts (department);
        CREATE INDEX idx_contacts_peer_id ON contacts (peer_id);
        CREATE INDEX idx_contacts_is_favorite ON contacts (is_favorite);
        CREATE INDEX idx_contacts_is_online ON contacts (is_online);
        CREATE INDEX idx_contacts_peer_ip ON contacts (peer_ip);
        "#.to_string(),
    )).await?;

    Ok(())
}

async fn fix_contact_group_members_table(db: &dyn ConnectionTrait) -> Result<(), DbErr> {
    // Step 1: Create a new contact_group_members table without the invalid foreign keys
    db.execute(Statement::from_string(
        DbBackend::Sqlite,
        r#"
        CREATE TABLE contact_group_members_new (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            contact_id INTEGER NOT NULL,
            group_id INTEGER NOT NULL,
            joined_at TIMESTAMP_TEXT NOT NULL
        );
        "#.to_string(),
    )).await?;

    // Step 2: Copy data from old table to new one
    db.execute(Statement::from_string(
        DbBackend::Sqlite,
        r#"
        INSERT INTO contact_group_members_new
        SELECT id, contact_id, group_id, joined_at
        FROM contact_group_members;
        "#.to_string(),
    )).await?;

    // Step 3: Drop the old table
    db.execute(Statement::from_string(
        DbBackend::Sqlite,
        "DROP TABLE contact_group_members;".to_string(),
    )).await?;

    // Step 4: Rename the new table
    db.execute(Statement::from_string(
        DbBackend::Sqlite,
        "ALTER TABLE contact_group_members_new RENAME TO contact_group_members;".to_string(),
    )).await?;

    // Step 5: Recreate indexes
    db.execute(Statement::from_string(
        DbBackend::Sqlite,
        r#"
        CREATE INDEX idx_contact_group_members_contact_id ON contact_group_members (contact_id);
        CREATE INDEX idx_contact_group_members_group_id ON contact_group_members (group_id);
        CREATE INDEX idx_contact_group_members_contact_group ON contact_group_members (contact_id, group_id);
        "#.to_string(),
    )).await?;

    Ok(())
}

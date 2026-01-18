// src-tauri/src/migration/mod.rs
pub use sea_orm_migration::prelude::*;

mod m20260105_000001_create_tables;
mod m20260110_000001_add_composite_indexes;
mod m20260114_000001_add_peer_ip_to_contacts;
mod m20260114_000001_create_contacts_tables;
mod m20260116_000001_remove_invalid_contacts_foreign_key;
mod m20260118_000001_add_user_id_to_peers;

#[allow(dead_code)]
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260105_000001_create_tables::Migration),
            Box::new(m20260110_000001_add_composite_indexes::Migration),
            Box::new(m20260114_000001_create_contacts_tables::Migration),
            Box::new(m20260114_000001_add_peer_ip_to_contacts::Migration),
            Box::new(m20260116_000001_remove_invalid_contacts_foreign_key::Migration),
            Box::new(m20260118_000001_add_user_id_to_peers::Migration),
        ]
    }
}

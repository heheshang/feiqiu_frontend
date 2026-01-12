// src-tauri/src/migration/mod.rs
pub use sea_orm_migration::prelude::*;

mod m20260105_000001_create_tables;
mod m20260110_000001_add_composite_indexes;

#[allow(dead_code)]
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260105_000001_create_tables::Migration),
            Box::new(m20260110_000001_add_composite_indexes::Migration),
        ]
    }
}

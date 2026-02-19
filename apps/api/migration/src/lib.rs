pub use sea_orm_migration::prelude::*;

mod m20260217_000001_init_schema;

/// Registri urutan migrasi schema database.
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20260217_000001_init_schema::Migration)]
    }
}

use sea_orm_migration::prelude::*;

/// Titik masuk CLI migrasi SeaORM.
#[tokio::main]
async fn main() {
    cli::run_cli(migration::Migrator).await;
}

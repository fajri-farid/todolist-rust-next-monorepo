use std::time::Duration;

use anyhow::Result;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

use crate::config::database::DatabaseSettings;

/// Membuka koneksi SeaORM dengan konfigurasi pool dari `DatabaseSettings`.
pub async fn connect_database(settings: &DatabaseSettings) -> Result<DatabaseConnection> {
    let mut options = ConnectOptions::new(settings.database_url.clone());
    options
        .max_connections(settings.max_connections)
        .min_connections(settings.min_connections)
        .connect_timeout(Duration::from_secs(settings.connect_timeout_secs))
        .idle_timeout(Duration::from_secs(settings.idle_timeout_secs))
        .max_lifetime(Duration::from_secs(settings.max_lifetime_secs))
        .sqlx_logging(false);

    let conn = Database::connect(options).await?;
    Ok(conn)
}

use anyhow::{Context, Result, bail};

/// Konfigurasi koneksi database yang dibaca saat startup.
#[derive(Debug, Clone)]
pub struct DatabaseSettings {
    pub database_url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connect_timeout_secs: u64,
    pub idle_timeout_secs: u64,
    pub max_lifetime_secs: u64,
}

impl DatabaseSettings {
    /// Membaca dan memvalidasi konfigurasi database dari environment variable.
    pub fn from_env() -> Result<Self> {
        let raw_url = std::env::var("DATABASE_URL").context("missing DATABASE_URL")?;
        let ssl_mode = std::env::var("DATABASE_SSL_MODE").unwrap_or_else(|_| "disable".to_string());
        let database_url = with_ssl_mode(&raw_url, &ssl_mode);

        let max_connections = parse_u32_with_default("DATABASE_MAX_CONNECTIONS", 20)?;
        let min_connections = parse_u32_with_default("DATABASE_MIN_CONNECTIONS", 2)?;
        if min_connections > max_connections {
            bail!("DATABASE_MIN_CONNECTIONS cannot be greater than DATABASE_MAX_CONNECTIONS");
        }

        Ok(Self {
            database_url,
            max_connections,
            min_connections,
            connect_timeout_secs: parse_u64_with_default("DATABASE_CONNECT_TIMEOUT_SECS", 8)?,
            idle_timeout_secs: parse_u64_with_default("DATABASE_IDLE_TIMEOUT_SECS", 600)?,
            max_lifetime_secs: parse_u64_with_default("DATABASE_MAX_LIFETIME_SECS", 1800)?,
        })
    }

    /// Versi aman untuk log: password pada connection string disamarkan.
    pub fn redacted_database_url(&self) -> String {
        redact_connection_string(&self.database_url)
    }
}

// Helper functions untuk parsing dan manipulasi string konfigurasi database.
fn parse_u32_with_default(key: &str, default: u32) -> Result<u32> {
    match std::env::var(key) {
        Ok(raw) => raw.parse::<u32>().with_context(|| format!("invalid value for {key}: {raw}")),
        Err(_) => Ok(default),
    }
}

// Sama seperti `parse_u32_with_default` tapi untuk tipe `u64`.
fn parse_u64_with_default(key: &str, default: u64) -> Result<u64> {
    match std::env::var(key) {
        Ok(raw) => raw.parse::<u64>().with_context(|| format!("invalid value for {key}: {raw}")),
        Err(_) => Ok(default),
    }
}

// Menambahkan parameter sslmode pada connection string bila belum ada, sesuai dengan konfigurasi yang diberikan.
fn with_ssl_mode(database_url: &str, ssl_mode: &str) -> String {
    if database_url.contains("sslmode=") {
        return database_url.to_string();
    }

    if database_url.contains('?') {
        format!("{database_url}&sslmode={ssl_mode}")
    } else {
        format!("{database_url}?sslmode={ssl_mode}")
    }
}

// Menyembunyikan informasi sensitif (seperti password) dari connection string untuk keperluan logging.
fn redact_connection_string(database_url: &str) -> String {
    match database_url.split_once('@') {
        Some((prefix, host_part)) => {
            let username = prefix.rsplit_once('/').map(|(_, right)| right).unwrap_or("user");
            format!("postgresql://{username}:***@{host_part}")
        }
        None => "postgresql://***".to_string(),
    }
}

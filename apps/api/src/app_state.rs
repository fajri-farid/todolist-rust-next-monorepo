use sea_orm::DatabaseConnection;
use uuid::Uuid;

/// State bersama yang di-inject ke seluruh handler Axum.
#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    /// User default untuk mode no-auth.
    pub default_user_id: Uuid,
}

impl AppState {
    /// Membentuk state aplikasi setelah dependency startup siap.
    pub fn new(db: DatabaseConnection, default_user_id: Uuid) -> Self {
        Self {
            db,
            default_user_id,
        }
    }
}

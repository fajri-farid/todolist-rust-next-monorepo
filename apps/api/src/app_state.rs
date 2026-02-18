use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct AppState {
    #[allow(dead_code)]
    pub db: DatabaseConnection,
}

impl AppState {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

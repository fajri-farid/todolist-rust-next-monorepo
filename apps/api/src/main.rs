use anyhow::Result;
use axum::{
    http::{HeaderValue, Method},
    Json, Router,
    routing::get,
};
use serde::Serialize;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

mod app_state;
mod config;
mod infrastructure;

use app_state::AppState;
use config::database::DatabaseSettings;
use infrastructure::db::connection::connect_database;

#[derive(Debug, Serialize)]
struct HealthResponse {
    status: &'static str,
}

#[derive(Debug, Serialize)]
struct HelloResponse {
    message: &'static str,
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok" })
}

async fn hello() -> Json<HelloResponse> {
    Json(HelloResponse {
        message: "hello, i am from rust",
    })
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "api=debug,tower_http=debug".to_string()),
        )
        .init();

    let api_host = std::env::var("API_HOST").unwrap_or_else(|_| "localhost".to_string());
    let api_port = std::env::var("API_PORT").unwrap_or_else(|_| "8080".to_string());
    let web_origin =
        std::env::var("WEB_ORIGIN").unwrap_or_else(|_| "http://localhost:3000".to_string());
    let cors_origin = web_origin.parse::<HeaderValue>()?;
    let db_settings = DatabaseSettings::from_env()?;
    let db = connect_database(&db_settings).await?;
    let app_state = AppState::new(db);

    let app = Router::new()
        .route("/hello", get(hello))
        .route("/health", get(health))
        .with_state(app_state)
        .layer(
            CorsLayer::new()
                .allow_origin(cors_origin)
                .allow_methods([Method::GET]),
        )
        .layer(TraceLayer::new_for_http());

    let bind_addr = format!("{api_host}:{api_port}");
    let listener = tokio::net::TcpListener::bind(&bind_addr).await?;

    tracing::info!("API listening on http://{bind_addr}");
    tracing::info!("CORS allowed origin: {web_origin}");
    tracing::info!("Database connected: {}", db_settings.redacted_database_url());
    axum::serve(listener, app).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn health_should_return_ok() {
        let response = health().await;
        assert_eq!(response.0.status, "ok");
    }

    #[tokio::test]
    async fn hello_should_return_expected_message() {
        let response = hello().await;
        assert_eq!(response.0.message, "hello, i am from rust");
    }
}

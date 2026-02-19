use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
};
use tracing::{error, info, warn};
use uuid::Uuid;

use crate::app_state::AppState;
use crate::common::error::ApiError;

use super::dto::{CreateTodoRequest, SuccessResponse, TodoResponse, UpdateTodoRequest};
use super::service::{self, TodoError};

/// Registrasi route todo.
///
/// - `POST /` buat todo
/// - `GET /` daftar todo
/// - `GET /{id}` detail todo
/// - `PATCH /{id}` update todo
/// - `DELETE /{id}` hapus todo
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", post(create_todo).get(list_todos))
        .route("/{id}", get(get_todo).patch(update_todo).delete(delete_todo))
}

/// POST `/todos`
///
/// Input: `CreateTodoRequest`.
/// Output: `201 Created` + `SuccessResponse<TodoResponse>`.
pub async fn create_todo(
    State(state): State<AppState>,
    Json(payload): Json<CreateTodoRequest>,
) -> Result<(StatusCode, Json<SuccessResponse<TodoResponse>>), ApiError> {
    let todo = service::create_todo(&state.db, state.default_user_id, payload)
        .await
        .map_err(map_todo_error)?;
    info!(todo_id = %todo.id, "todo created");
    Ok((StatusCode::CREATED, Json(SuccessResponse { data: todo.into() })))
}

/// GET `/todos`
/// Output: `200 OK` + daftar todo milik user default.
pub async fn list_todos(
    State(state): State<AppState>,
) -> Result<Json<SuccessResponse<Vec<TodoResponse>>>, ApiError> {
    let todos = service::list_todos(&state.db, state.default_user_id)
        .await
        .map_err(map_todo_error)?;
    info!(count = todos.len(), "todos listed");
    let data = todos.into_iter().map(TodoResponse::from).collect();
    Ok(Json(SuccessResponse { data }))
}

/// GET `/todos/{id}`
/// Output: `200 OK` bila ditemukan, `404` bila tidak ada.
pub async fn get_todo(
    State(state): State<AppState>,
    Path(todo_id): Path<Uuid>,
) -> Result<Json<SuccessResponse<TodoResponse>>, ApiError> {
    let todo = service::get_todo(&state.db, state.default_user_id, todo_id)
        .await
        .map_err(map_todo_error)?;
    info!(todo_id = %todo_id, "todo detail fetched");
    Ok(Json(SuccessResponse { data: todo.into() }))
}

/// PATCH `/todos/{id}`
/// Input: `UpdateTodoRequest` (parsial).
/// Output: `200 OK` dengan data todo terbaru.
pub async fn update_todo(
    State(state): State<AppState>,
    Path(todo_id): Path<Uuid>,
    Json(payload): Json<UpdateTodoRequest>,
) -> Result<Json<SuccessResponse<TodoResponse>>, ApiError> {
    let todo = service::update_todo(&state.db, state.default_user_id, todo_id, payload)
        .await
        .map_err(map_todo_error)?;
    info!(todo_id = %todo_id, "todo updated");
    Ok(Json(SuccessResponse { data: todo.into() }))
}

/// DELETE `/todos/{id}`
/// Output: `204 No Content` bila sukses, `404` bila id tidak ditemukan.
pub async fn delete_todo(
    State(state): State<AppState>,
    Path(todo_id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    service::delete_todo(&state.db, state.default_user_id, todo_id)
        .await
        .map_err(map_todo_error)?;
    info!(todo_id = %todo_id, "todo deleted");
    Ok(StatusCode::NO_CONTENT)
}

/// Mapping error domain todo ke `ApiError` agar response error konsisten.
fn map_todo_error(err: TodoError) -> ApiError {
    match err {
        TodoError::Validation(message) => {
            warn!(reason = %message, "todo validation failed");
            ApiError::bad_request(message)
        }
        TodoError::NotFound => {
            warn!("todo not found");
            ApiError::not_found("todo not found")
        }
        TodoError::Database(db_err) => {
            error!(error = %db_err, "todo database operation failed");
            ApiError::internal("unexpected database error")
        }
    }
}

// --- IGNORE ---
// Modul tests untuk integrasi end-to-end API todo.
#[cfg(test)]
mod tests {
    use std::env;

    use axum::{
        Router,
        body::{Body, to_bytes},
        http::Request,
    };
    use sea_orm::Database;
    use serde_json::{Value, json};
    use tower::ServiceExt;
    use uuid::Uuid;

    use crate::{app_state::AppState, modules::todo::repository};

    use super::*;

    async fn build_test_app() -> Router {
        dotenvy::dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must exist for integration tests");
        let default_user_id = env::var("DEFAULT_USER_ID")
            .ok()
            .and_then(|v| Uuid::parse_str(&v).ok())
            .unwrap_or_else(|| Uuid::parse_str("00000000-0000-0000-0000-000000000001").expect("valid uuid"));
        let db = Database::connect(db_url)
            .await
            .expect("database must be reachable for integration tests");
        repository::ensure_default_user(&db, default_user_id)
            .await
            .expect("default user seed must succeed");
        Router::new()
            .nest("/todos", routes())
            .with_state(AppState::new(db, default_user_id))
    }

    async fn read_json(response: axum::response::Response) -> Value {
        let body = to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("body must be readable");
        serde_json::from_slice(&body).expect("body must be valid json")
    }

    #[tokio::test]
    #[ignore = "requires running local database"]
    async fn todo_crud_flow_should_succeed() {
        let app = build_test_app().await;

        let create_req = Request::builder()
            .method("POST")
            .uri("/todos")
            .header("content-type", "application/json")
            .body(Body::from(
                json!({ "title": "integration todo", "desc": "from test" }).to_string(),
            ))
            .expect("request should be valid");
        let create_res = app.clone().oneshot(create_req).await.expect("response expected");
        assert_eq!(create_res.status(), StatusCode::CREATED);
        let create_body = read_json(create_res).await;
        let todo_id = create_body["data"]["id"]
            .as_str()
            .expect("todo id must exist")
            .to_string();

        let detail_req = Request::builder()
            .method("GET")
            .uri(format!("/todos/{todo_id}"))
            .body(Body::empty())
            .expect("request should be valid");
        let detail_res = app.clone().oneshot(detail_req).await.expect("response expected");
        assert_eq!(detail_res.status(), StatusCode::OK);

        let update_req = Request::builder()
            .method("PATCH")
            .uri(format!("/todos/{todo_id}"))
            .header("content-type", "application/json")
            .body(Body::from(
                json!({ "title": "integration todo updated", "iscompleted": true }).to_string(),
            ))
            .expect("request should be valid");
        let update_res = app.clone().oneshot(update_req).await.expect("response expected");
        assert_eq!(update_res.status(), StatusCode::OK);
        let update_body = read_json(update_res).await;
        assert_eq!(update_body["data"]["iscompleted"], true);

        let delete_req = Request::builder()
            .method("DELETE")
            .uri(format!("/todos/{todo_id}"))
            .body(Body::empty())
            .expect("request should be valid");
        let delete_res = app.clone().oneshot(delete_req).await.expect("response expected");
        assert_eq!(delete_res.status(), StatusCode::NO_CONTENT);

        let deleted_detail_req = Request::builder()
            .method("GET")
            .uri(format!("/todos/{todo_id}"))
            .body(Body::empty())
            .expect("request should be valid");
        let deleted_detail_res = app
            .clone()
            .oneshot(deleted_detail_req)
            .await
            .expect("response expected");
        assert_eq!(deleted_detail_res.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    #[ignore = "requires running local database"]
    async fn delete_unknown_todo_should_return_404() {
        let app = build_test_app().await;
        let unknown_id = Uuid::new_v4();

        let delete_req = Request::builder()
            .method("DELETE")
            .uri(format!("/todos/{unknown_id}"))
            .body(Body::empty())
            .expect("request should be valid");
        let response = app.oneshot(delete_req).await.expect("response expected");
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    #[ignore = "requires running local database"]
    async fn update_invalid_payload_should_return_400() {
        let app = build_test_app().await;

        let create_req = Request::builder()
            .method("POST")
            .uri("/todos")
            .header("content-type", "application/json")
            .body(Body::from(json!({ "title": "todo for invalid update" }).to_string()))
            .expect("request should be valid");
        let create_res = app.clone().oneshot(create_req).await.expect("response expected");
        assert_eq!(create_res.status(), StatusCode::CREATED);
        let create_body = read_json(create_res).await;
        let todo_id = create_body["data"]["id"]
            .as_str()
            .expect("todo id must exist")
            .to_string();

        let update_req = Request::builder()
            .method("PATCH")
            .uri(format!("/todos/{todo_id}"))
            .header("content-type", "application/json")
            .body(Body::from(json!({ "title": "   " }).to_string()))
            .expect("request should be valid");
        let response = app.oneshot(update_req).await.expect("response expected");
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}

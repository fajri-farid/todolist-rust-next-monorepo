use sea_orm::{DatabaseConnection, DbErr};
use uuid::Uuid;

use super::dto::{CreateTodoRequest, UpdateTodoPayload, UpdateTodoRequest};
use super::entity::Model;
use super::repository;

const MAX_TITLE_LEN: usize = 200;
const MAX_DESC_LEN: usize = 2000;

/// Error domain todo yang nantinya dipetakan ke HTTP error di layer handler.
#[derive(Debug)]
pub enum TodoError {
    Validation(String),
    NotFound,
    Database(DbErr),
}

impl From<DbErr> for TodoError {
    fn from(value: DbErr) -> Self {
        Self::Database(value)
    }
}

/// Membuat todo baru setelah normalisasi title dan desc.
pub async fn create_todo(
    conn: &DatabaseConnection,
    default_user_id: Uuid,
    request: CreateTodoRequest,
) -> Result<Model, TodoError> {
    let title = normalize_title(request.title)?;
    let desc = normalize_desc(request.desc)?;

    repository::create_todo(conn, default_user_id, title, desc)
        .await
        .map_err(TodoError::from)
}

/// Mengambil seluruh todo user default.
pub async fn list_todos(conn: &DatabaseConnection, default_user_id: Uuid) -> Result<Vec<Model>, TodoError> {
    repository::list_todos(conn, default_user_id)
        .await
        .map_err(TodoError::from)
}

/// Mengambil detail todo by id.
/// Mengembalikan `NotFound` bila id tidak ada pada scope user.
pub async fn get_todo(
    conn: &DatabaseConnection,
    default_user_id: Uuid,
    todo_id: Uuid,
) -> Result<Model, TodoError> {
    let todo = repository::find_todo_by_id(conn, default_user_id, todo_id)
        .await
        .map_err(TodoError::from)?;
    todo.ok_or(TodoError::NotFound)
}

/// Update parsial todo setelah validasi payload.
pub async fn update_todo(
    conn: &DatabaseConnection,
    default_user_id: Uuid,
    todo_id: Uuid,
    request: UpdateTodoRequest,
) -> Result<Model, TodoError> {
    let existing = repository::find_todo_by_id(conn, default_user_id, todo_id)
        .await
        .map_err(TodoError::from)?
        .ok_or(TodoError::NotFound)?;

    let changes = normalize_update_payload(request)?;
    repository::update_todo(conn, existing, changes)
        .await
        .map_err(TodoError::from)
}

/// Menghapus todo by id.
/// Mengembalikan `NotFound` bila tidak ada data yang terhapus.
pub async fn delete_todo(
    conn: &DatabaseConnection,
    default_user_id: Uuid,
    todo_id: Uuid,
) -> Result<(), TodoError> {
    let deleted = repository::delete_todo(conn, default_user_id, todo_id)
        .await
        .map_err(TodoError::from)?;
    if !deleted {
        return Err(TodoError::NotFound);
    }
    Ok(())
}

/// Normalisasi dan validasi judul todo.
pub fn normalize_title(raw_title: String) -> Result<String, TodoError> {
    let title = raw_title.trim();
    if title.is_empty() {
        return Err(TodoError::Validation("title must not be empty".to_string()));
    }
    if title.len() > MAX_TITLE_LEN {
        return Err(TodoError::Validation(format!(
            "title must be at most {MAX_TITLE_LEN} characters"
        )));
    }
    Ok(title.to_string())
}

/// Validasi panjang deskripsi todo bila field dikirim.
pub fn normalize_desc(raw_desc: Option<String>) -> Result<Option<String>, TodoError> {
    match raw_desc {
        Some(desc) => {
            if desc.len() > MAX_DESC_LEN {
                Err(TodoError::Validation(format!(
                    "desc must be at most {MAX_DESC_LEN} characters"
                )))
            } else {
                Ok(Some(desc))
            }
        }
        None => Ok(None),
    }
}

/// Menyusun payload update yang sudah melalui validasi field.
pub fn normalize_update_payload(request: UpdateTodoRequest) -> Result<UpdateTodoPayload, TodoError> {
    let title = match request.title {
        Some(value) => Some(normalize_title(value)?),
        None => None,
    };
    let desc = normalize_desc(request.desc)?;

    Ok(UpdateTodoPayload {
        title,
        desc,
        iscompleted: request.iscompleted,
    })
}

// --- IGNORE ---
// Modul tests untuk unit testing fungsi normalisasi dan validasi pada service todo.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_title_should_fail_when_empty() {
        let result = normalize_title("   ".to_string());
        assert!(matches!(result, Err(TodoError::Validation(_))));
    }

    #[test]
    fn normalize_update_payload_should_support_partial_update() {
        let payload = normalize_update_payload(UpdateTodoRequest {
            title: Some(" Updated ".to_string()),
            desc: None,
            iscompleted: Some(true),
        })
        .expect("payload should be valid");

        assert_eq!(payload.title, Some("Updated".to_string()));
        assert_eq!(payload.desc, None);
        assert_eq!(payload.iscompleted, Some(true));
    }
}

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::entity;

/// Payload request untuk membuat todo baru.
#[derive(Debug, Deserialize)]
pub struct CreateTodoRequest {
    pub title: String,
    /// Deskripsi opsional; jika tidak dikirim akan bernilai `None`.
    #[serde(default)]
    pub desc: Option<String>,
}

/// Payload request untuk update parsial todo.
#[derive(Debug, Deserialize)]
pub struct UpdateTodoRequest {
    /// Judul baru; `None` berarti tidak diubah.
    #[serde(default)]
    pub title: Option<String>,
    /// Deskripsi baru; `None` berarti tidak diubah.
    #[serde(default)]
    pub desc: Option<String>,
    /// Status selesai; `None` berarti tidak diubah.
    #[serde(default)]
    pub iscompleted: Option<bool>,
}

/// Bentuk data todo yang dikirim kembali ke client.
#[derive(Debug, Serialize)]
pub struct TodoResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub desc: Option<String>,
    pub iscompleted: bool,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
}

/// Wrapper sukses untuk semua response API.
#[derive(Debug, Serialize)]
pub struct SuccessResponse<T> {
    pub data: T,
}

/// Payload update hasil normalisasi sebelum dikirim ke repository.
#[derive(Debug, Clone)]
pub struct UpdateTodoPayload {
    pub title: Option<String>,
    pub desc: Option<String>,
    pub iscompleted: Option<bool>,
}

impl From<UpdateTodoRequest> for UpdateTodoPayload {
    /// Konversi langsung request update ke payload internal.
    fn from(value: UpdateTodoRequest) -> Self {
        Self {
            title: value.title,
            desc: value.desc,
            iscompleted: value.iscompleted,
        }
    }
}

impl From<entity::Model> for TodoResponse {
    /// Mapping model database ke response HTTP.
    fn from(value: entity::Model) -> Self {
        Self {
            id: value.id,
            user_id: value.user_id,
            title: value.title,
            desc: value.desc,
            iscompleted: value.iscompleted,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

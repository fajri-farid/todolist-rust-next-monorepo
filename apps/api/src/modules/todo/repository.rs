use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbBackend, DbErr, EntityTrait, Order,
    QueryFilter, QueryOrder, Set, Statement, ConnectionTrait,
};
use uuid::Uuid;

use super::dto::UpdateTodoPayload;
use super::entity::{self, ActiveModel, Column, Entity, Model};

/// Menjamin user default tersedia untuk mode no-auth.
/// Query bersifat idempotent melalui `ON CONFLICT (id) DO NOTHING`.
pub async fn ensure_default_user(conn: &DatabaseConnection, user_id: Uuid) -> Result<(), DbErr> {
    let email = format!("default-{user_id}@local.todo");
    let sql = format!(
        "INSERT INTO users (id, email, password_hash, full_name, created_at, updated_at) \
         VALUES ('{user_id}', '{email}', 'no-auth-yet', 'Default User', NOW(), NOW()) \
         ON CONFLICT (id) DO NOTHING;"
    );
    conn.execute(Statement::from_string(DbBackend::Postgres, sql))
        .await?;
    Ok(())
}

/// Menyimpan todo baru ke database untuk user tertentu.
pub async fn create_todo(
    conn: &DatabaseConnection,
    user_id: Uuid,
    title: String,
    desc: Option<String>,
) -> Result<Model, DbErr> {
    // `created_at` dan `updated_at` diset sama saat insert awal.
    let now = Utc::now().fixed_offset();
    let active = ActiveModel {
        id: Set(Uuid::new_v4()),
        user_id: Set(user_id),
        title: Set(title),
        desc: Set(desc),
        iscompleted: Set(false),
        created_at: Set(now),
        updated_at: Set(now),
    };
    active.insert(conn).await
}

/// Mengambil daftar todo milik user, urut terbaru lebih dulu.
pub async fn list_todos(conn: &DatabaseConnection, user_id: Uuid) -> Result<Vec<Model>, DbErr> {
    // Semua query dibatasi per user agar data antar user tidak tercampur.
    Entity::find()
        .filter(Column::UserId.eq(user_id))
        .order_by(Column::CreatedAt, Order::Desc)
        .all(conn)
        .await
}

/// Mengambil satu todo berdasarkan `todo_id` dalam scope user tertentu.
pub async fn find_todo_by_id(
    conn: &DatabaseConnection,
    user_id: Uuid,
    todo_id: Uuid,
) -> Result<Option<Model>, DbErr> {
    Entity::find()
        .filter(Column::UserId.eq(user_id))
        .filter(Column::Id.eq(todo_id))
        .one(conn)
        .await
}

/// Menerapkan perubahan parsial ke todo yang sudah ada.
pub async fn update_todo(
    conn: &DatabaseConnection,
    existing: Model,
    changes: UpdateTodoPayload,
) -> Result<Model, DbErr> {
    let mut active: entity::ActiveModel = existing.into();
    if let Some(title) = changes.title {
        active.title = Set(title);
    }
    if let Some(desc) = changes.desc {
        active.desc = Set(Some(desc));
    }
    if let Some(iscompleted) = changes.iscompleted {
        active.iscompleted = Set(iscompleted);
    }
    active.updated_at = Set(Utc::now().fixed_offset());
    active.update(conn).await
}

/// Menghapus todo berdasarkan id dan user.
/// Mengembalikan `true` bila ada baris terhapus, `false` bila todo tidak ditemukan.
pub async fn delete_todo(
    conn: &DatabaseConnection,
    user_id: Uuid,
    todo_id: Uuid,
) -> Result<bool, DbErr> {
    let result = Entity::delete_many()
        .filter(Column::UserId.eq(user_id))
        .filter(Column::Id.eq(todo_id))
        .exec(conn)
        .await?;
    Ok(result.rows_affected > 0)
}

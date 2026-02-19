use sea_orm::entity::prelude::*;
use serde::Serialize;
use uuid::Uuid;
use chrono::{DateTime, FixedOffset};

/// Entity SeaORM untuk tabel `todos`.
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "todos")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    /// `desc` dipakai sebagai nama kolom agar kompatibel dengan schema yang ada.
    #[sea_orm(column_name = "desc")]
    pub desc: Option<String>,
    /// Tetap memakai nama kolom `iscompleted` agar sesuai kontrak data existing.
    #[sea_orm(column_name = "iscompleted")]
    pub iscompleted: bool,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

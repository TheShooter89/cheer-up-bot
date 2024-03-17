use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

use crate::http::error::Error;
use crate::http::http::Result;

use super::users;

#[derive(Debug, Serialize, Deserialize)]
pub struct Note {
    pub id: i64,
    pub user_id: i64,
    pub file_name: String,
}

#[derive(Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
struct UpdateNote {
    id: Option<i64>,
    user_id: Option<i64>,
    file_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewNote {
    user_id: i64,
    file_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct NoteBody<T> {
    note: T,
}

#[derive(Debug, Serialize, Deserialize)]
struct NoteListBody<T> {
    note: Vec<T>,
}

pub fn router(pool: SqlitePool) -> Router<()> {
    Router::new()
        .route("/api/notes", get(get_notes_list))
        .route("/api/note/:note_id", get(get_note))
        .with_state(pool)
}

async fn get_note(
    Path(note_id): Path<String>,
    State(pool): State<SqlitePool>,
) -> Result<Json<NoteBody<Note>>> {
    let note: Note = sqlx::query_as!(
        Note,
        r#"
SELECT id, user_id, file_name
FROM notes
WHERE id = ?
    "#,
        note_id
    )
    .fetch_one(&pool)
    .await?;

    Ok(Json(NoteBody { note }))
}

pub async fn get_notes_list(State(pool): State<SqlitePool>) -> Result<Json<NoteListBody<Note>>> {
    let users: Vec<Note> = sqlx::query_as!(
        Note,
        r#"
SELECT id, user_id, file_name
FROM notes
ORDER BY id
    "#,
    )
    .fetch_all(&pool)
    .await?;

    Ok(Json(NoteListBody { note: users }))
}

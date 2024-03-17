use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

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
struct DeleteNote {
    id: Option<i64>,
    user_id: Option<i64>,
    file_name: Option<String>,
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
    notes: Vec<T>,
}

pub fn router(pool: SqlitePool) -> Router<()> {
    Router::new()
        .route("/api/notes", get(get_notes_list))
        // TODO: sure as hell there's a better way to do this without duplication
        .route("/api/notes/", get(get_notes_list))
        .route("/api/notes/:note_id", get(get_note).delete(delete_note))
        .route(
            "/api/notes/user/:user_id",
            get(get_notes_list_by_user).delete(delete_all_user_notes),
        )
        .route("/api/notes/random", get(get_random_note))
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

async fn delete_note(
    Path(note_id): Path<String>,
    State(pool): State<SqlitePool>,
) -> Result<Json<NoteBody<String>>> {
    let _note = sqlx::query_as!(
        Note,
        r#"
DELETE FROM notes
WHERE id = ?
    "#,
        note_id
    )
    .execute(&pool)
    .await?;

    Ok(Json(NoteBody { note: note_id }))
}

async fn get_notes_list(State(pool): State<SqlitePool>) -> Result<Json<NoteListBody<Note>>> {
    let notes: Vec<Note> = sqlx::query_as!(
        Note,
        r#"
SELECT id, user_id, file_name
FROM notes
ORDER BY id
    "#,
    )
    .fetch_all(&pool)
    .await?;

    Ok(Json(NoteListBody { notes }))
}

async fn get_notes_list_by_user(
    Path(user_id): Path<String>,
    State(pool): State<SqlitePool>,
) -> Result<Json<NoteListBody<Note>>> {
    let notes: Vec<Note> = sqlx::query_as!(
        Note,
        r#"
SELECT id, user_id, file_name
FROM notes
WHERE user_id = ?
ORDER BY id
    "#,
        user_id,
    )
    .fetch_all(&pool)
    .await?;

    Ok(Json(NoteListBody { notes }))
}

async fn delete_all_user_notes(
    Path(user_id): Path<String>,
    State(pool): State<SqlitePool>,
) -> Result<Json<NoteBody<String>>> {
    let _notes = sqlx::query_as!(
        Note,
        r#"
DELETE FROM notes
WHERE user_id = ?
    "#,
        user_id,
    )
    .execute(&pool)
    .await?;

    Ok(Json(NoteBody { note: user_id }))
}

async fn get_random_note(State(pool): State<SqlitePool>) -> Result<Json<NoteBody<Note>>> {
    let note: Note = sqlx::query_as!(
        Note,
        r#"
SELECT id, user_id, file_name
FROM notes
ORDER BY RANDOM()
LIMIT 1
    "#,
    )
    .fetch_one(&pool)
    .await?;

    Ok(Json(NoteBody { note }))
}

use axum::{
    extract::State,
    routing::{get, post},
    Extension, Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};

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

pub fn router(pool: SqlitePool) -> Router<()> {
    Router::new()
        .route("/api/note", get(get_user))
        .with_state(pool)
}

pub async fn get_user(State(pool): State<SqlitePool>) -> Result<Json<NoteBody<Note>>> {
    let users = sqlx::query_as!(
        Note,
        r#"
SELECT id, user_id, file_name
FROM notes
ORDER BY id
    "#,
    )
    .fetch_one(&pool)
    .await?;

    Ok(Json(NoteBody { note: users }))

    // Ok(Json(NoteBody {
    //     note: Note {
    //         id: 9,
    //         user_id: 28,
    //         file_name: "harlock".to_string(),
    //     },
    // }))
}

use axum::{
    extract::State,
    routing::{get, post},
    Extension, Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};

use crate::http::error::Error;
use crate::http::http::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    id: i64,
    telegram_id: i64,
    username: String,
    first_name: String,
    last_name: Option<String>,
}

#[derive(Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
struct UpdateUser {
    id: Option<i64>,
    telegram_id: Option<i64>,
    username: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewUser {
    telegram_id: i64,
    username: String,
    first_name: String,
    last_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct UserBody<T> {
    user: T,
}

pub fn router(pool: SqlitePool) -> Router<()> {
    Router::new()
        .route("/api/user", get(get_user))
        .with_state(pool)
}

pub async fn get_user(State(pool): State<SqlitePool>) -> Result<Json<UserBody<User>>> {
    Ok(Json(UserBody {
        user: User {
            id: 2,
            telegram_id: 12,
            username: "tanqueshen".to_string(),
            first_name: "francesco".to_string(),
            last_name: None,
        },
    }))
}

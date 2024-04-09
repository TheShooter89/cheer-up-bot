use axum::{
    extract::{Path, State},
    routing::{get, post},
    Extension, Json, Router,
};
use log::debug;
use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};

use crate::http::error::Error;
use crate::http::http::Result;
use crate::http::locale::Locale;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, sqlx::Type)]
pub struct User {
    id: i64,
    telegram_id: i64,
    username: String,
    first_name: String,
    last_name: Option<String>,
    locale: Locale,
}

#[derive(Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
struct UpdateUser {
    id: Option<i64>,
    telegram_id: Option<i64>,
    username: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
    locale: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, sqlx::Type)]
pub struct NewUser {
    telegram_id: i64,
    username: String,
    first_name: String,
    last_name: Option<String>,
    locale: Locale,
}

#[derive(Debug, Serialize, Deserialize)]
struct UserBody<T> {
    user: T,
}

#[derive(Debug, Serialize, Deserialize)]
struct UserListBody<T> {
    users: Vec<T>,
}

pub fn router(pool: SqlitePool) -> Router<()> {
    Router::new()
        .route("/api/users", get(get_users_list).post(create_user))
        .route("/api/users/", get(get_users_list).post(create_user))
        .route("/api/users/:user_id", get(get_user).delete(delete_user))
        .route(
            "/api/users/name/:username",
            get(get_user_by_telegram_username),
        )
        .with_state(pool)
}

async fn get_users_list(State(pool): State<SqlitePool>) -> Result<Json<UserListBody<User>>> {
    let users: Vec<User> = sqlx::query_as!(
        User,
        r#"
SELECT u.id, u.telegram_id, u.username, u.first_name, u.last_name, l.language AS "locale: Locale"
FROM users AS u
INNER JOIN locales AS l ON u.locale = l.id
ORDER BY u.id
    "#,
    )
    .fetch_all(&pool)
    .await?;

    Ok(Json(UserListBody { users }))
}

async fn get_user(
    Path(user_id): Path<String>,
    State(pool): State<SqlitePool>,
) -> Result<Json<UserBody<User>>> {
    let user: User = sqlx::query_as!(
        User,
        r#"
SELECT u.id, u.telegram_id, u.username, u.first_name, u.last_name, l.language AS "locale: Locale"
FROM users AS u
INNER JOIN locales AS l ON u.locale = l.id
WHERE u.id = ?
    "#,
        user_id
    )
    .fetch_one(&pool)
    .await?;

    Ok(Json(UserBody { user }))
}

async fn get_user_by_telegram_username(
    Path(username): Path<String>,
    State(pool): State<SqlitePool>,
) -> Result<Json<UserBody<User>>> {
    let user: User = sqlx::query_as!(
        User,
        r#"
SELECT u.id, u.telegram_id, u.username, u.first_name, u.last_name, l.language AS "locale: Locale"
FROM users AS u
INNER JOIN locales AS l ON u.locale = l.id
WHERE u.username = ?
    "#,
        username
    )
    .fetch_one(&pool)
    .await?;

    Ok(Json(UserBody { user }))
}

async fn create_user(
    State(pool): State<SqlitePool>,
    Json(user): Json<NewUser>,
) -> Result<Json<UserBody<User>>> {
    let loc = user.locale;
    let user: User = sqlx::query_as!(
        User,
                r#"
        INSERT INTO users (telegram_id, username, first_name, last_name, locale)
        VALUES (?, ?, ?, ?, (SELECT id FROM locales WHERE language = ?));

        SELECT u.id, u.telegram_id, u.username, u.first_name, u.last_name, l.language AS "locale: Locale"
        FROM users AS u
        INNER JOIN locales AS l ON u.locale = l.id
        WHERE u.id = last_insert_rowid()
            "#,
        user.telegram_id,
        user.username,
        user.first_name,
        user.last_name,
        loc
    )
    .fetch_one(&pool)
    .await?;

    Ok(Json(UserBody { user }))
}

async fn delete_user(
    Path(user_id): Path<String>,
    State(pool): State<SqlitePool>,
) -> Result<Json<UserBody<String>>> {
    let _user = sqlx::query_as!(
        User,
        r#"
DELETE FROM users
WHERE id = ?
    "#,
        user_id
    )
    .execute(&pool)
    .await?;

    Ok(Json(UserBody { user: user_id }))
}

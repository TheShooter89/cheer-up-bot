use axum::{
    extract::{Path, State},
    routing::{get, post},
    Extension, Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};

use crate::http::error::Error;
use crate::http::http::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct Stats {
    total_videonotes: i32,
    users: Vec<UserStats>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserStats {
    username: String,
    videonotes: i64,
}

#[derive(Debug, Serialize, Deserialize)]
struct StatsBody<T> {
    stats: T,
}

pub fn router(pool: SqlitePool) -> Router<()> {
    Router::new()
        .route("/api/stats", get(get_stats))
        .route("/api/stats/", get(get_stats))
        .route("/api/stats/user/:user_id", get(get_user_stats))
        .with_state(pool)
}

async fn get_stats(State(pool): State<SqlitePool>) -> Result<Json<StatsBody<Stats>>> {
    let users: Vec<UserStats> = sqlx::query_as!(
        UserStats,
        r#"
SELECT u.username, COUNT(n.id) AS videonotes
FROM users u
LEFT JOIN notes n ON u.id = n.user_id
GROUP BY u.id
    "#,
    )
    .fetch_all(&pool)
    .await?;

    let total_videonotes: i32 = sqlx::query_scalar!(
        r#"
SELECT COUNT(n.id) AS total_videonotes
FROM notes n
    "#,
    )
    .fetch_one(&pool)
    .await?;

    let stats = Stats {
        total_videonotes,
        users,
    };

    Ok(Json(StatsBody { stats }))
}

async fn get_user_stats(
    Path(user_id): Path<String>,
    State(pool): State<SqlitePool>,
) -> Result<Json<StatsBody<UserStats>>> {
    let stats: UserStats = sqlx::query_as!(
        UserStats,
        r#"
SELECT u.username, COUNT(n.id) AS videonotes
FROM users u
LEFT JOIN notes n ON u.id = n.user_id
WHERE u.id = ?
GROUP BY u.id
    "#,
        user_id
    )
    .fetch_one(&pool)
    .await?;

    Ok(Json(StatsBody { stats }))
}

use axum::{Router, ServiceExt};
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;
use std::time::Duration;
use tokio::net::TcpListener;

use crate::http::error::Error;

use crate::http::{notes, users};

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub async fn serve(db_pool: SqlitePool) -> Result<()> {
    let app = api_router(db_pool);

    let listener = TcpListener::bind("0.0.0.0:1989")
        .await
        .expect("error establishing TcpListener");

    axum::serve(listener, app)
        .await
        .expect("error starting server");
    Ok(())
}

fn api_router(pool: SqlitePool) -> Router {
    Router::new()
        .merge(users::router(pool.clone()))
        .merge(notes::router(pool))
}

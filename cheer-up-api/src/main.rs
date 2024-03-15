extern crate pretty_env_logger;

use anyhow::Ok;
use sqlx::{pool, sqlite::SqlitePoolOptions};
// use time::Duration;
use std::time::Duration;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

// mod http;
use cheer_up_api::http::error;
use cheer_up_api::http::http;

#[macro_use]
extern crate log;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    pretty_env_logger::init();

    // tracing_subscriber::registry()
    //     .with(EnvFilter::new(std::env::var("RUST_LOG").unwrap_or_else(
    //         |_| "axum_login=debug,tower_sessions=debug,sqlx=warn,tower_http=debug".into(),
    //     )))
    //     .with(tracing_subscriber::fmt::layer())
    //     .try_init()?;

    info!("Hello, world!");

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect("sqlite://../_common_data/data/database.db?mode=rwc")
        .await
        .expect("cannot connect to database");

    sqlx::migrate!().run(&pool).await?;

    let app = http::serve(pool).await?;
    Ok(())
}

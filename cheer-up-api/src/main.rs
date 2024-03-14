extern crate pretty_env_logger;

use anyhow::Ok;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[macro_use]
extern crate log;

fn main() -> Result<(), anyhow::Error> {
    pretty_env_logger::init();

    tracing_subscriber::registry()
        .with(EnvFilter::new(std::env::var("RUST_LOG").unwrap_or_else(
            |_| "axum_login=debug,tower_sessions=debug,sqlx=warn,tower_http=debug".into(),
        )))
        .with(tracing_subscriber::fmt::layer())
        .try_init()?;

    info!("Hello, world!");
    Ok(())
}

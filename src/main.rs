mod config;
mod routes;
mod models;
mod db;

use std::sync::Arc;

use config::Config;
use db::connection::Database;
use dotenv::dotenv;
use routes::create_router;
use anyhow::{Context, Result};
use log::error;


#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    env_logger::init();

    let config = Config::setup().context("Failed to load configuration")
    .unwrap_or_else(|err| {
        error!("{:?}", err);
        std::process::exit(1);
    });
    let db = Database::new(&config.db_url)
        .await
        .context("Failed to create database connection")?;
    let db = Arc::new(db);

    let app = create_router(db.clone());

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", config.host, config.port))
        .await
        .context("Failed to bind to address")?;

    log::info!("Server starting on {}:{}", config.host, config.port);

    axum::serve(listener, app)
        .await
        .context("Server error occurred")?;

    Ok(())
}

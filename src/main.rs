mod config;
mod routes;
mod models;
mod db;

use std::sync::Arc;

use config::Config;
use db::connection::Database;
use routes::create_router;

#[tokio::main]
async fn main() {
    let config = Config::setup().unwrap();

    let db = Database::new(&config.db_url).await.expect("Failed to create database connection");
    let db = Arc::new(db);

    let app = create_router(db.clone());

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", config.host, config.port)).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

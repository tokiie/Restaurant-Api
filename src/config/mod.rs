use std::env;

use anyhow::{Context, Result};
use log::info;


#[derive(Debug)]
pub struct Config {
    pub host: String,
    pub port: String,
    pub db_url: String,
}


impl Config {
    pub fn setup() -> Result<Self> {
        let host = env::var("HOST").context("HOST environment variable is not set")?;
        let port = env::var("PORT")
            .context("PORT environment variable is not set")?;
        let db_url = env::var("DB_URL")
            .context("DB_URL environment variable is not set")?;

        info!("Configuration loaded: host={}, port={}, db_url={}", host, port, db_url);

        Ok(Config {
            host,
            port,
            db_url,
        })
    }
}


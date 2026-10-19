pub mod jobs;

use std::time::Duration;
use sqlx::{postgres::PgPoolOptions, PgPool};

pub async fn connect(database_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new().max_connections(5).acquire_timeout(Duration::from_secs(5)).connect(database_url).await
}

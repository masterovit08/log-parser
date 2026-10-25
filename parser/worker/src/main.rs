mod handler;

use anyhow::{Context, Result};
use handler::handle_job;
use tracing::{error, info};
use worker::{RedisQueue, connect};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let redis_url =
        std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string());
    let worker_id = std::env::var("WORKER_ID").unwrap_or_else(|_| "worker-1".to_string());
    let mut queue = RedisQueue::connect(&redis_url, &worker_id).await?;
    info!("Connected to Redis");

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://log-parser:log-parser_dev@127.0.0.1:5433".to_string());
    let db_pool = connect(&database_url)
        .await
        .context("Failed to connect to database")?;
    info!("Connected to PostgreSQL");

    info!(worker = %worker_id, "Worker started");

    loop {
        let Some(job_message) = queue.next().await? else {
            continue;
        };

        let job_id = job_message.job_id;

        info!(job_id = %job_id, "Job received");

        match handle_job(&db_pool, job_id).await {
            Ok(Some(_job)) => {
                queue.ack(&job_message.message_id).await?;

                info!(job_id = %job_id, "Job acknowledged");
            }

            Ok(None) => {
                error!(job_id = %job_id, "Job was not found");
                queue.ack(&job_message.message_id).await?;
            }

            Err(err) => {
                error!(job_id = %job_id, error = %err, "Job failed");
            }
        }
    }
}

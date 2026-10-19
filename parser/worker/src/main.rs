mod handler;

use anyhow::{Result, Context};
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

    let database_url = std::env::var("DATABASE_URL").context("DATABASE_URL not set")?;
    let _db_pool = connect(&database_url).await.context("Failed to connect to database")?;
    info!("Connected to PostgreSQL");

    info!(worker = %worker_id, "Worker started");

    loop {
        let Some(job) = queue.next().await? else {
            continue;
        };

        info!(job_id = %job.job_id, "Job received");

        match handler::handle_job(&job.job_id).await {
            Ok(()) => {
                queue.ack(&job.message_id).await?;

                info!(job_id = %job.job_id, "Job acknowledged");
            }

            Err(error) => {
                error!(job_id = %job.job_id, %error, "Job failed");
            }
        }
    }
}

use anyhow::Result;
use sqlx::PgPool;
use tracing::{error, info};
use uuid::Uuid;
use worker::claim;

pub async fn handle_job(db_pool: &PgPool, job_id: Uuid) -> Result<()> {
    info!(%job_id, "Processing job");

    match claim(db_pool, job_id).await {
        Ok(true) => {
            info!(job_id = %job_id, "Job status has been successfully changed")
        }

        Ok(false) => {
            error!(job_id = %job_id, "Job has not been registered");
        }

        Err(_e) => {
            error!(job_id = %job_id, "Failed to load job");
        }
    }

    Ok(())
}

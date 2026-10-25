use anyhow::Result;
use job_model::Job;
use sqlx::PgPool;
use tracing::{error, info};
use uuid::Uuid;
use worker::claim;

pub async fn handle_job(db_pool: &PgPool, job_id: Uuid) -> Result<Option<Job>> {
    info!(%job_id, "Processing job");

    let Some(job) = claim(db_pool, job_id).await? else {
        error!(job_id = %job_id, "Job has not been registered");
        return Ok(None);
    };

    info!(job_id = %job_id, "Job status has been successfully changed");

    Ok(Some(job))
}

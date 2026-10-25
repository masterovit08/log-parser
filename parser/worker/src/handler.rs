use anyhow::Result;
use job_model::Job;
use sqlx::PgPool;
use tracing::{error, info};
use uuid::Uuid;
use worker::{claim, complete};

pub async fn handle_job(db_pool: &PgPool, job_id: Uuid) -> Result<Option<Job>> {
    info!(%job_id, "Processing job");

    let Some(job) = claim(db_pool, job_id).await? else {
        error!(job_id = %job_id, "Job has not been registered");
        return Ok(None);
    };

    info!(job_id = %job_id, job_status = %job.status, "Job status has been successfully changed");

    // reading from MinIO and parsing

    let completed = complete(db_pool, job_id).await?;
    info!(job_id = %job_id, "Job has been completed");

    Ok(completed)
}

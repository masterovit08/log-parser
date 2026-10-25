use anyhow::Result;
use job_model::{Job, JobStatus};
use sqlx::PgPool;
use uuid::Uuid;

async fn change_status(
    pool: &PgPool,
    job_id: Uuid,
    status: JobStatus,
    old_status: JobStatus,
) -> Result<Option<Job>> {
    let job = sqlx::query_as::<_, Job>(
        r#"
            UPDATE jobs
            SET
                status = $2,
                started_at = NOW()
            WHERE
                id = $1
                AND status = $3
            RETURNING id, status, minio_key, created_at, started_at, finished_at"#,
    )
    .bind(job_id)
    .bind(status.as_str())
    .bind(old_status.as_str())
    .fetch_optional(pool)
    .await?;

    Ok(job)
}

pub async fn claim(pool: &PgPool, job_id: Uuid) -> Result<Option<Job>> {
    change_status(pool, job_id, JobStatus::Processing, JobStatus::Queued).await
}

pub async fn complete(pool: &PgPool, job_id: Uuid) -> Result<Option<Job>> {
    change_status(pool, job_id, JobStatus::Completed, JobStatus::Processing).await
}

pub async fn fail(pool: &PgPool, job_id: Uuid) -> Result<Option<Job>> {
    change_status(pool, job_id, JobStatus::Failed, JobStatus::Processing).await
}

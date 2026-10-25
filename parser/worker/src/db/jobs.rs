use anyhow::Result;
use job_model::{Job, JobStatus};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn claim(pool: &PgPool, job_id: Uuid) -> Result<Option<Job>> {
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
    .bind(JobStatus::Processing.as_str())
    .bind(JobStatus::Queued.as_str())
    .fetch_optional(pool)
    .await?;

    Ok(job)
}

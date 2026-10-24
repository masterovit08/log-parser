use sqlx::PgPool;
use uuid::Uuid;

pub async fn claim(pool: &PgPool, job_id: Uuid) -> Result<bool, sqlx::Error> {
    let job: Option<Uuid> = sqlx::query_scalar(
        r#"
            UPDATE jobs
            SET
                status = 'preprocessing',
                started_at = NOW()
            WHERE
                id = $1
                AND status = 'queued'
            RETURNING id"#,
    )
    .bind(job_id)
    .fetch_optional(pool)
    .await?;

    Ok(job.is_some())
}

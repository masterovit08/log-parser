use anyhow::Context;
use job_model::JobStatus;
use redis::{AsyncCommands, streams::StreamPendingReply};
use std::env;
use uuid::Uuid;
use worker::{RedisQueue, claim, complete, connect};

#[tokio::test]
#[ignore = "requires a running Redis and PostgreSQL nstance"]
pub async fn integration_test() -> anyhow::Result<()> {
    let db_url = env::var("DATABASE_TEST_URL").context("DATABASE_TEST_URL not set")?;
    let redis_url = env::var("REDIS_TEST_URL").context("REDIS_TEST_URL not set")?;
    let stream = format!("log-parser:test:{}", Uuid::now_v7());
    let group = "test-workers";
    let job_id = Uuid::now_v7();
    let job_id_str = job_id.to_string();

    let db_pool = connect(&db_url).await.context("failed to connect to db")?;
    let redis_client = redis::Client::open(redis_url.as_str())?;
    let mut redis_connection = redis_client
        .get_multiplexed_async_connection()
        .await
        .context("failed to get redis connection")?;

    sqlx::query(
        r#"
        create table if not exists jobs
            (
                id          uuid                                                         not null
                    primary key,
                status      varchar(32)              default 'queued'::character varying not null,
                minio_key   text                                                         not null,
                created_at  timestamp with time zone default now()                       not null,
                started_at  timestamp with time zone,
                finished_at timestamp with time zone
            );"#,
    )
    .execute(&db_pool)
    .await?;

    let test_minio_key = format!("raw-logs/{}.log", job_id);

    sqlx::query("INSERT INTO jobs (id, status, minio_key) VALUES ($1, $2, $3)")
        .bind(job_id)
        .bind("queued")
        .bind(&test_minio_key)
        .execute(&db_pool)
        .await?;

    let _: () = redis_connection
        .xgroup_create_mkstream(&stream, group, "0")
        .await?;
    let _: String = redis_connection
        .xadd(&stream, "*", &[("job_id", &job_id_str)])
        .await?;
    let mut queue = RedisQueue::connect_named(&redis_url, "test-worker", &stream, group).await?;
    let received = queue.next().await?.context("No job received")?;

    assert_eq!(String::from(received.job_id), job_id_str);

    let pending: StreamPendingReply = redis_connection.xpending(&stream, group).await?;
    assert_eq!(pending.count(), 1);

    let claimed_job = claim(&db_pool, received.job_id)
        .await?
        .context("job claim failed")?;

    assert_eq!(claimed_job.id, job_id);
    assert_eq!(claimed_job.status, JobStatus::Processing);
    assert_eq!(claimed_job.minio_key, test_minio_key);
    assert!(claimed_job.started_at.is_some());
    assert!(claimed_job.finished_at.is_none());

    let second_claim = claim(&db_pool, received.job_id).await?;
    assert!(second_claim.is_none());

    let completed = complete(&db_pool, received.job_id)
        .await?
        .context("job completion failed")?;
    assert_eq!(completed.status, JobStatus::Completed);

    let second_completed = complete(&db_pool, received.job_id).await?;
    assert!(second_completed.is_none());

    queue.ack(&received.message_id).await?;

    let pending: StreamPendingReply = redis_connection.xpending(&stream, group).await?;
    assert_eq!(pending.count(), 0);

    sqlx::query("DELETE FROM jobs WHERE id = $1")
        .bind(job_id)
        .execute(&db_pool)
        .await
        .ok();
    let _: usize = redis_connection.del(&stream).await?;

    Ok(())
}

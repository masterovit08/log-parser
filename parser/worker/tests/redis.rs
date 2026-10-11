use worker::RedisQueue;

use anyhow::Context;
use redis::{streams::StreamPendingReply, AsyncCommands};
use uuid::Uuid;

#[tokio::test]
#[ignore = "requires a running Redis instance"]
async fn test_redis() -> anyhow::Result<()> {
    let url = std::env::var("REDIS_TEST_URL").context("REDIS_TEST_URL not found")?;
    let stream = format!("log-parser:test:{}", Uuid::now_v7());
    let group = "test-workers";
    let job_id = Uuid::now_v7().to_string();

    let client = redis::Client::open(url.as_str())?;
    let mut connection = client.get_multiplexed_async_connection().await?;

    let _: () = connection.xgroup_create_mkstream(&stream, group, "0").await?;
    let _: String = connection.xadd(&stream, "*", &[("job_id", job_id.as_str())]).await?;
    let mut queue = RedisQueue::connect_named(&url, "test-worker", &stream, group).await?;

    let received = queue.next().await?.context("No job received")?;

    assert_eq!(received.job_id, job_id);

    let pending: StreamPendingReply = connection.xpending(&stream, group).await?;
    assert_eq!(pending.count(), 1);

    queue.ack(&received.message_id).await?;

    let pending: StreamPendingReply = connection.xpending(&stream, group).await?;
    assert_eq!(pending.count(), 0);

    let _: usize = connection.del(&stream).await?;

    Ok(())
}
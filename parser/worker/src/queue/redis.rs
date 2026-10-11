use anyhow::{Result, Context};
use redis::{aio::MultiplexedConnection, streams::{StreamReadOptions, StreamReadReply}, AsyncCommands, AsyncConnectionConfig};
use std::time::Duration;

pub struct QueuedJob {
    pub message_id: String,
    pub job_id: String,
}

pub struct RedisQueue {
    connection: MultiplexedConnection,
    stream: String,
    group: String,
    consumer: String,
}

impl RedisQueue {
    pub async fn connect(url: &str, consumer: &str) -> Result<Self> {
        let client = redis::Client::open(url)?;
        let config = AsyncConnectionConfig::new()
            .set_connection_timeout(Some(Duration::from_secs(5)))
            .set_response_timeout(Some(Duration::from_secs(10)));

        let connection = client.get_multiplexed_async_connection_with_config(&config).await?;

        Ok(Self {
            connection,
            stream: "log-parser:jobs".into(),
            group: "log-parser-workers".into(),
            consumer: consumer.into(),
        })
    }

    pub async fn next(&mut self) -> Result<Option<QueuedJob>> {
        let options = StreamReadOptions::default().group(&self.group, &self.consumer).count(1).block(5000);
        let reply: Option<StreamReadReply> = self.connection.xread_options(
            &[self.stream.as_str()],
            &[">"],
            &options
        ).await?;

        let Some(reply) = reply else { return Ok(None); };
        let Some(message) = reply.keys.into_iter().flat_map(|stream| stream.ids).next() else { return Ok(None); };

        let job_id: String = message.get("job_id").context("job_id is missing")?;

        Ok(Some(QueuedJob { message_id: message.id, job_id }))
    }

    pub async fn ack(&mut self, message_id: &str) -> Result<()> {
        let acknowledged: bool = self.connection.xack(&self.stream, &self.group, &[message_id]).await?;

        if acknowledged != true {
            anyhow::bail!("message wasn't acknowledged");
        }

        Ok(())
    }
}



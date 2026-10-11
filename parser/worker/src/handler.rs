use anyhow::Result;
use uuid::Uuid;

pub async fn handle_job(job_id: &str) -> Result<()> {
    let job_id = Uuid::parse_str(job_id)?;
    tracing::info!(%job_id, "Processing job");

    Ok(())
}

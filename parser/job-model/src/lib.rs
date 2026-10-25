use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Row, postgres::PgRow};
use std::fmt;
use std::str::FromStr;
use uuid::Uuid;

pub struct QueuedJob {
    pub message_id: String,
    pub job_id: Uuid,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Job {
    pub id: Uuid,
    pub status: JobStatus,
    pub minio_key: String,
    pub created_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub finished_at: Option<DateTime<Utc>>,
}

impl<'r> FromRow<'r, PgRow> for Job {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        let status_raw: String = row.try_get("status")?;
        let status: JobStatus =
            JobStatus::from_str(&status_raw).map_err(|err| sqlx::Error::Decode(err.into()))?;

        Ok(Self {
            id: row.try_get("id")?,
            status,
            minio_key: row.try_get("minio_key")?,
            created_at: row.try_get("created_at")?,
            started_at: row.try_get("started_at")?,
            finished_at: row.try_get("finished_at")?,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum JobType {
    AnalyzeLogs,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Copy)]
#[serde(rename_all = "snake_case")]
pub enum JobStatus {
    Queued,
    Processing,
    Completed,
    Failed,
    Cancelled,
}

impl JobStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            JobStatus::Queued => "queued",
            JobStatus::Processing => "processing",
            JobStatus::Completed => "completed",
            JobStatus::Failed => "failed",
            JobStatus::Cancelled => "cancelled",
        }
    }
}

impl fmt::Display for JobStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for JobStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "queued" => Ok(JobStatus::Queued),
            "processing" => Ok(JobStatus::Processing),
            "completed" => Ok(JobStatus::Completed),
            "failed" => Ok(JobStatus::Failed),
            "cancelled" => Ok(JobStatus::Cancelled),
            other => Err(format!("Unknown job status: {}", other)),
        }
    }
}

impl TryFrom<String> for JobStatus {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::from_str(&value)
    }
}

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("invalid JSON: {0}")]
    InvalidJson(#[from] serde_json::Error),

    #[error("invalid log format")]
    InvalidFormat,

    #[error("invalid nginx timestamp: {0}")]
    InvalidTimestamp(#[from] chrono::ParseError),

    #[error("unsupported log format")]
    UnsupportedFormat,
}
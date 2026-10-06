use crate::error::ParseError;
use crate::model::LogEntry;

pub trait LogParser {
    fn parse_line(&self, line: &str) -> Result<LogEntry, ParseError>;
}

pub mod json;
pub mod nginx;
pub mod syslog;
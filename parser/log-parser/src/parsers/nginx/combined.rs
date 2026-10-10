use chrono::DateTime;
use regex::Regex;

use crate::{
    error::ParseError,
    model::LogEntry,
    parsers::LogParser
};

pub struct NginxCombinedParser {
    regex: Regex,
}

impl NginxCombinedParser {
    pub fn new() -> NginxCombinedParser {
        let pattern = concat!(
            r#"^(?P<ip>\S+) \S+ (?P<user>\S+) "#,
            r#"\[(?P<timestamp>[^\]]+)\] "#,
            r#""(?P<request>(?:\\.|[^"\\])*)" "#,
            r#"(?P<status>\d{3}) (?P<bytes>\d+|-) "#,
            r#""(?P<referer>(?:\\.|[^"\\])*)" "#,
            r#""(?P<agent>(?:\\.|[^"\\])*)"$"#,
        );

        let regex = Regex::new(&pattern).expect("Invalid nginx combined regex");

        Self { regex }
    }
}

impl LogParser for NginxCombinedParser {
    fn parse_line(&self, line: &str) -> Result<LogEntry, ParseError> {
        let captures = self.regex.captures(line).ok_or(ParseError::InvalidFormat)?;
        let timestamp = DateTime::parse_from_str(&captures["timestamp"], "%d/%b/%Y:%H:%M:%S %z")?.to_rfc3339();
        let request = &captures["request"];
        let mut parts = request.split(" ");
        let method = parts.next().ok_or(ParseError::InvalidFormat)?;
        let path = parts.next().ok_or(ParseError::InvalidFormat)?;
        let protocol = parts.next().ok_or(ParseError::InvalidFormat)?;

        if !protocol.starts_with("HTTP/") {return Err(ParseError::InvalidFormat);}

        let status = captures["status"].parse::<u16>().map_err(|_| ParseError::InvalidFormat)?;

        if !(100..=599).contains(&status) {
            return Err(ParseError::InvalidFormat);
        }

        Ok(LogEntry {
            timestamp: Some(timestamp),
            level: None,
            service: Some("nginx".to_owned()),
            message: request.to_owned(),
            method: Some(method.to_owned()),
            path: Some(path.to_owned()),
            status_code: Some(status as u64),
            duration_ms: Option::None,
            request_id: None,
            raw: line.to_owned(),
        })

    }
}
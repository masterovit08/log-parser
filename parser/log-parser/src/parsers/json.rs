use crate::{error::ParseError, model::LogEntry, parsers::LogParser, LogLevel};
use serde_json::Value;

fn get_string(value: &Value, keys: &[&str]) -> Option<String> {
    keys.iter().find_map(|key| value.get(*key).and_then(Value::as_str)).map(str::to_owned)
}

fn get_u64(value: &Value, keys: &[&str]) -> Option<u64> {
    keys.iter().find_map(|key| value.get(*key).and_then(Value::as_u64))
}

fn get_f64(value: &Value, keys: &[&str]) -> Option<f64> {
    keys.iter().find_map(|key| value.get(*key).and_then(Value::as_f64))
}

pub struct JsonParser;

impl LogParser for JsonParser {
    fn parse_line(&self, line: &str) -> Result<LogEntry, ParseError> {
        let value: Value = serde_json::from_str(line)?;

        let timestamp = get_string(&value, &["timestamp", "time", "@timestamp"]);
        let level = get_string(&value, &["level", "log_level", "status", "severity"])
            .and_then(|value| LogLevel::parse(&value));
        let service = get_string(&value, &["service", "service_type", "service_name", "app"]);
        let message = get_string(&value, &["message", "msg", "event", "event_message", "log_message", "text"]).unwrap_or_default();
        let status_code = get_u64(&value, &["status_code", "status", "code"]);
        let duration_ms = get_f64(&value, &["duration", "ms", "duration_ms", "duration-ms"]);
        let request_id = get_string(&value, &["request_id", "request", "requestId", "request-id"]);

        Ok(LogEntry {
            timestamp,
            level,
            service,
            message,
            method: None,
            path: None,
            status_code,
            duration_ms,
            request_id,
            raw: line.to_owned()
        })


    }
}

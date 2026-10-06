#[derive(Debug, Clone, PartialEq)]
pub struct LogEntry {
    pub timestamp: Option<String>,
    pub level: Option<LogLevel>,
    pub service: Option<String>,
    pub message: String,
    pub method: Option<String>,
    pub path: Option<String>,
    pub status_code: Option<u64>,
    pub duration_ms: Option<f64>,
    pub request_id: Option<String>,
    pub raw: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Fatal
}

impl LogLevel {
    pub fn parse(value: &str) -> Option<Self> {
        match value.trim().to_ascii_lowercase().as_str() {
            "trace" => Some(LogLevel::Trace),
            "debug" => Some(LogLevel::Debug),
            "info" => Some(LogLevel::Info),
            "warn" | "warning" => Some(LogLevel::Warn),
            "error" | "err" => Some(LogLevel::Error),
            "fatal" | "critical" | "crit" => Some(LogLevel::Fatal),
            _ => None
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum LogType {
    JsonLines,
    NginxCombined,
    Syslog
}
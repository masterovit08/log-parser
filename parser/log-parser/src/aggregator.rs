use crate::{
    analysis::AnalysisResult,
    model::{LogEntry, LogLevel},
};

#[derive(Debug, Default)]
pub struct Aggregator {
    pub result: AnalysisResult
}

impl Aggregator {
    pub fn process(&mut self, entry: &LogEntry) {
        self.result.total += 1;

        if matches!(entry.level, Some(LogLevel::Error | LogLevel::Fatal)) {
            self.result.errors += 1;
        }

        match entry.status_code {
            Some(200..=299) => self.result.http_2xx += 1,
            Some(400..=499) => self.result.http_4xx += 1,
            Some(500..=599) => { self.result.http_5xx += 1; self.result.errors += 1; },
            _ => {}
        }

        self.result.total_duration_ms += entry.duration_ms.unwrap();
        self.result.duration_count += 1;
    }

    pub fn result(&self) -> &AnalysisResult {
        &self.result
    }
    pub fn finish(&self) -> AnalysisResult { self.result }
}
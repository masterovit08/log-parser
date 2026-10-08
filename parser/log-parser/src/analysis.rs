#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct AnalysisResult {
    pub total: u64,
    pub errors: u64,

    pub http_2xx: u64,
    pub http_4xx: u64,
    pub http_5xx: u64,

    pub total_duration_ms: f64,
    pub duration_count: u64
}

impl AnalysisResult {
    pub fn average_duration_ms(&self) -> Option<f64> {
        if self.duration_count > 0 {
            Some(self.total_duration_ms / self.duration_count as f64)
        } else {
            None
        }
    }
}
use std::io::{self, BufRead};
use crate::{aggregator::Aggregator, analysis::AnalysisResult, parsers::LogParser};

#[derive(Debug, PartialEq)]
pub struct LineError {
    pub line: usize,
    pub message: String,
}

#[derive(Debug, PartialEq)]
pub struct FileAnalysis {
    pub result: AnalysisResult,
    pub total_lines: u64,
    pub parsed_lines: u64,
    pub failed_lines: u64,
    pub errors: Vec<LineError>
}

pub fn analyze<R: BufRead>(reader: R, parser: &dyn LogParser) -> io::Result<FileAnalysis> {
    let mut aggregator = Aggregator::default();
    let mut total_lines = 0;
    let mut parsed_lines = 0;
    let mut failed_lines = 0;
    let mut errors = Vec::new();

    for (index, line) in reader.lines().enumerate() {
        let line = line?;
        total_lines += 1;

        if line.trim().is_empty() {
            continue;
        }

        match parser.parse_line(&line) {
            Ok(entry) => {
                aggregator.process(&entry);
                parsed_lines += 1;
            }
            Err(error) => {
                failed_lines += 1;

                if errors.len() < 20 {
                    errors.push(LineError{
                        line: index + 1,
                        message: error.to_string()
                    })
                }
            }
        }
    }

    Ok(FileAnalysis {
        result: aggregator.finish(),
        total_lines,
        parsed_lines,
        failed_lines,
        errors
    })
}
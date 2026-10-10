use std::{fs::File, io::BufReader};
use log_parser::{analyze, FileAnalysis, AnalysisResult, LogEntry, LogLevel, parser, LogType};

#[test]
fn parse_jsonl_line() -> Result<(), Box<dyn std::error::Error>> {
    let parser = parser(LogType::JsonLines)?;
    let line = "{\"timestamp\": \"2026-01-15T09:00:05Z\", \"level\": \"INFO\", \"service\": \"api\", \"request_id\": \"req-46048\", \"user_id\": 251, \"action\": \"logout\", \"duration_ms\": 35.31, \"status\": 200}";
    let result = parser.parse_line(line)?;

    //println!("result: {:#?}", result);

    assert_eq!(result,
               LogEntry {
                   timestamp: Some(
                       "2026-01-15T09:00:05Z".to_owned(),
                   ),
                   level: Some(
                       LogLevel::Info,
                   ),
                   service: Some(
                       "api".to_owned(),
                   ),
                   message: "".to_owned(),
                   method: None,
                   path: None,
                   status_code: Some(
                       200,
                   ),
                   duration_ms: Some(
                       35.31,
                   ),
                   request_id: Some(
                       "req-46048".to_owned(),
                   ),
                   raw: "{\"timestamp\": \"2026-01-15T09:00:05Z\", \"level\": \"INFO\", \"service\": \"api\", \"request_id\": \"req-46048\", \"user_id\": 251, \"action\": \"logout\", \"duration_ms\": 35.31, \"status\": 200}".to_owned(),
               }
    );

    Ok(())
}

#[test]
fn parse_jsonl() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("tests/artifacts/sample-ndjson.jsonl").expect("Unable to open artifact");
    let reader = BufReader::new(file);
    let parser = parser(LogType::JsonLines)?;
    let result = analyze(reader, parser.as_ref())?;

    //println!("{:#?}", result);

    assert_eq!(result, FileAnalysis {
        result: AnalysisResult {
            total: 50,
            errors: 19,
            http_2xx: 36,
            http_4xx: 7,
            http_5xx: 7,
            total_duration_ms: 5815.859999999999,
            duration_count: 50,
        },
        total_lines: 50,
        parsed_lines: 50,
        failed_lines: 0,
        errors: vec![],
    });

    Ok(())
}
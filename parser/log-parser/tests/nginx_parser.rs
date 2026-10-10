use std::fs::File;
use std::io::BufReader;
use log_parser::{analyze, FileAnalysis, AnalysisResult, LineError, LogEntry, parser, LogType};

#[test]
fn parse_nginx_combined_line() -> Result<(), Box<dyn std::error::Error>> {
    let parser = parser(LogType::NginxCombined)?;
    let line = "192.168.1.10 - - [10/Oct/2026:12:30:45 +0000] \"GET /api/users HTTP/1.1\" 200 1543 \"-\" \"Mozilla/5.0\"";
    let result = parser.parse_line(line)?;

    //println!("{:#?}", result);

    assert_eq!(result,
               LogEntry {
                   timestamp: Some(
                       "2026-10-10T12:30:45+00:00".to_owned(),
                   ),
                   level: None,
                   service: Some(
                       "nginx".to_owned(),
                   ),
                   message: "GET /api/users HTTP/1.1".to_owned(),
                   method: Some(
                       "GET".to_owned(),
                   ),
                   path: Some(
                       "/api/users".to_owned(),
                   ),
                   status_code: Some(
                       200,
                   ),
                   duration_ms: None,
                   request_id: None,
                   raw: "192.168.1.10 - - [10/Oct/2026:12:30:45 +0000] \"GET /api/users HTTP/1.1\" 200 1543 \"-\" \"Mozilla/5.0\"".to_owned(),
               }
    );

    Ok(())
}

#[test]
fn parse_nginx_combined() -> Result<(), Box<dyn std::error::Error>>{
    let parser = parser(LogType::NginxCombined)?;

    let file = File::open("tests/artifacts/nginx_access.log").expect("Unable to open artifact");
    let reader = BufReader::new(file);
    let result = analyze(reader, parser.as_ref())?;

    //println!("{:#?}", result);

    assert_eq!(result,
               FileAnalysis {
                   result: AnalysisResult {
                       total: 3,
                       errors: 1,
                       http_2xx: 1,
                       http_4xx: 1,
                       http_5xx: 1,
                       total_duration_ms: 0.0,
                       duration_count: 0,
                   },
                   total_lines: 4,
                   parsed_lines: 3,
                   failed_lines: 1,
                   errors: vec![
                       LineError {
                           line: 4,
                           message: "invalid log format".to_owned(),
                       },
                   ],
               }
    );

    Ok(())
}
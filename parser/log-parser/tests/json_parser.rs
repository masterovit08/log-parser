#[allow(unused_imports, unused)]

use std::{fs::File, io::{BufRead, BufReader}};
use log_parser::{JsonParser, LogEntry, LogLevel::{Debug, Info, Warn, Error, Fatal}, LogParser, Aggregator};

#[test]
fn parse_jsonl() {
    let file = File::open("tests/artifacts/sample-ndjson.jsonl").expect("Unable to open artifact");
    let reader = BufReader::new(file);
    let parser = JsonParser;
    let mut aggregator = Aggregator::default();
    let mut entries = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Unable to read line");

        match parser.parse_line(&line) {
            Ok(entry) => {
                aggregator.process(&entry);
                entries.push(entry);
            },
            Err(err) => eprintln!("{}", err)
        }
    }

    println!("{:#?}", aggregator.result());

    assert_eq!(entries.len(), 50);
    assert_eq!(entries[0], LogEntry { timestamp: Some(String::from("2026-01-15T09:00:05Z")), level: Some(Info), service: Some(String::from("api")), message: String::from(""), method: None, path: None, status_code: Some(200), duration_ms: Some(35.31), request_id: Some(String::from("req-46048")), raw: String::from("{\"timestamp\": \"2026-01-15T09:00:05Z\", \"level\": \"INFO\", \"service\": \"api\", \"request_id\": \"req-46048\", \"user_id\": 251, \"action\": \"logout\", \"duration_ms\": 35.31, \"status\": 200}") });
    assert_eq!(entries[1], LogEntry { timestamp: Some(String::from("2026-01-15T09:00:08Z")), level: Some(Info), service: Some(String::from("db")), message: String::from(""), method: None, path: None, status_code: Some(201), duration_ms: Some(23.88), request_id: Some(String::from("req-65302")), raw: String::from("{\"timestamp\": \"2026-01-15T09:00:08Z\", \"level\": \"INFO\", \"service\": \"db\", \"request_id\": \"req-65302\", \"user_id\": 33, \"action\": \"login\", \"duration_ms\": 23.88, \"status\": 201}") });
}
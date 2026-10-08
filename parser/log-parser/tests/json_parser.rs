#[allow(unused_imports, unused)]

use std::{fs::File, io::{BufRead, BufReader}};
use log_parser::{analyze, JsonParser, FileAnalysis, AnalysisResult};

#[test]
fn parse_jsonl() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("tests/artifacts/sample-ndjson.jsonl").expect("Unable to open artifact");
    let reader = BufReader::new(file);
    let parser = JsonParser;
    let result = analyze(reader, &parser)?;

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
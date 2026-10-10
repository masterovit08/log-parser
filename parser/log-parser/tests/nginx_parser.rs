use std::fs::File;
use std::io::BufReader;
use log_parser::{
    NginxCombinedParser,
    analyze,
    FileAnalysis,
    AnalysisResult,
    LineError
};

#[test]
fn parse_nginx_combined() -> Result<(), Box<dyn std::error::Error>>{
    let parser = NginxCombinedParser::new();

    let file = File::open("tests/artifacts/nginx_access.log").expect("Unable to open artifact");
    let reader = BufReader::new(file);
    let result = analyze(reader, &parser)?;

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
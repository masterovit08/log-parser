use crate::{
    error::ParseError,
    model::LogType,
    parsers::{LogParser, json::JsonParser, nginx::NginxCombinedParser},
};

pub fn parser(log_type: LogType) -> Result<Box<dyn LogParser>, ParseError> {
    match log_type {
        LogType::JsonLines => Ok(Box::new(JsonParser)),

        LogType::NginxCombined => Ok(Box::new(NginxCombinedParser::new())),

        _ => Err(ParseError::UnsupportedFormat),
    }
}

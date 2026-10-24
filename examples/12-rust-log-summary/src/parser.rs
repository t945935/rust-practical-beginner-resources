use std::fmt;

use crate::record::{LogRecord, Severity};

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct ParseError {
    pub(crate) line_number: usize,
    pub(crate) reason: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "第 {} 行：{}", self.line_number, self.reason)
    }
}

impl std::error::Error for ParseError {}

pub(crate) fn parse_record(
    line: &str,
    line_number: usize,
) -> Result<Option<LogRecord>, ParseError> {
    if line.trim().is_empty() {
        return Ok(None);
    }

    let Some((level, message)) = line.split_once('|') else {
        return Err(ParseError {
            line_number,
            reason: "缺少 | 分隔符".to_owned(),
        });
    };

    let level = level.trim();
    let message = message.trim();
    let severity = match level {
        "INFO" => Severity::Info,
        "WARN" => Severity::Warn,
        "ERROR" => Severity::Error,
        "" => {
            return Err(ParseError {
                line_number,
                reason: "層級不可為空".to_owned(),
            });
        }
        _ => {
            return Err(ParseError {
                line_number,
                reason: format!("不支援的層級：{level}"),
            });
        }
    };

    Ok(Some(LogRecord {
        severity,
        message: message.to_owned(),
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_info_record() {
        let record = parse_record("INFO|service started", 1).unwrap().unwrap();
        assert_eq!(record.message, "service started");
        assert_eq!(record.severity, Severity::Info);
    }

    #[test]
    fn parses_trimmed_message_and_preserves_later_separators() {
        let record = parse_record(" WARN | retry | later ", 3).unwrap().unwrap();
        assert_eq!(record.severity, Severity::Warn);
        assert_eq!(record.message, "retry | later");
    }

    #[test]
    fn ignores_blank_line() {
        assert_eq!(parse_record("  ", 1).unwrap(), None);
    }

    #[test]
    fn rejects_missing_separator_with_original_line_number() {
        let error = parse_record("INFO service started", 4).unwrap_err();
        assert_eq!(error.line_number, 4);
        assert_eq!(error.reason, "缺少 | 分隔符");
    }

    #[test]
    fn rejects_empty_level_with_original_line_number() {
        let error = parse_record(" | message", 5).unwrap_err();
        assert_eq!(error.line_number, 5);
        assert_eq!(error.reason, "層級不可為空");
    }

    #[test]
    fn parse_error_has_original_line_number() {
        let error = parse_record("NOTICE|hello", 7).unwrap_err();
        assert_eq!(error.line_number, 7);
    }
}

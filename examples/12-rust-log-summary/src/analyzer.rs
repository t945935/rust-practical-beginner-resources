use crate::record::{LogRecord, Severity};

#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) struct Summary {
    pub(crate) total: usize,
    pub(crate) info: usize,
    pub(crate) warn: usize,
    pub(crate) error: usize,
}

pub(crate) fn summarize(records: &[LogRecord]) -> Summary {
    let mut summary = Summary {
        total: records.len(),
        ..Summary::default()
    };
    for record in records {
        match record.severity {
            Severity::Info => summary.info += 1,
            Severity::Warn => summary.warn += 1,
            Severity::Error => summary.error += 1,
        }
    }
    summary
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse_record;

    #[test]
    fn summarizes_empty_records_as_zero() {
        assert_eq!(summarize(&[]), Summary::default());
    }

    #[test]
    fn counts_each_severity() {
        let records = [
            parse_record("INFO|a", 1).unwrap().unwrap(),
            parse_record("WARN|b", 2).unwrap().unwrap(),
            parse_record("ERROR|c", 3).unwrap().unwrap(),
            parse_record("INFO|d", 4).unwrap().unwrap(),
        ];
        assert_eq!(
            summarize(&records),
            Summary {
                total: 4,
                info: 2,
                warn: 1,
                error: 1,
            }
        );
    }
}

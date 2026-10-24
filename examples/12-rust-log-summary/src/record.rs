#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Severity {
    Info,
    Warn,
    Error,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct LogRecord {
    pub(crate) severity: Severity,
    pub(crate) message: String,
}

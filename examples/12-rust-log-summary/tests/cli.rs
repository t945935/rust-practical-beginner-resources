use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_FILE_ID: AtomicU64 = AtomicU64::new(0);

struct TempFile(PathBuf);

impl TempFile {
    fn from_bytes(bytes: &[u8]) -> Self {
        let id = NEXT_FILE_ID.fetch_add(1, Ordering::Relaxed);
        let path = env::temp_dir().join(format!(
            "rust-log-summary-{}-{id}.log",
            std::process::id()
        ));
        fs::write(&path, bytes).expect("write temporary test input");
        Self(path)
    }
}

impl Drop for TempFile {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.0);
    }
}

fn run_cli<I, S>(args: I) -> Output
where
    I: IntoIterator<Item = S>,
    S: AsRef<std::ffi::OsStr>,
{
    Command::new(env!("CARGO_BIN_EXE_rust-log-summary"))
        .args(args)
        .output()
        .expect("start rust-log-summary")
}

fn assert_failure(output: &Output, expected_stderr: &[&str]) {
    assert!(!output.status.success(), "expected nonzero exit status");
    assert!(
        output.stdout.is_empty(),
        "error case must not print a summary"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    for expected in expected_stderr {
        assert!(
            stderr.contains(expected),
            "stderr {stderr:?} lacks {expected:?}"
        );
    }
}

#[test]
fn success_prints_exact_counts_only_to_stdout() {
    let sample = Path::new(env!("CARGO_MANIFEST_DIR")).join("sample.log");
    let output = run_cli([sample.as_os_str()]);
    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "總筆數：4\nINFO：2\nWARN：1\nERROR：1\n"
    );
    assert!(output.stderr.is_empty());
}

#[test]
fn trims_fields_ignores_blank_lines_and_preserves_later_pipes() {
    let input = TempFile::from_bytes(
        "\n INFO | started \r\n   \r\n WARN | retry | later \r\nERROR|\r\n".as_bytes(),
    );
    let output = run_cli([input.0.as_os_str()]);
    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "總筆數：3\nINFO：1\nWARN：1\nERROR：1\n"
    );
    assert!(output.stderr.is_empty());
}

#[test]
fn unknown_level_reports_original_line_and_no_partial_summary() {
    let input = TempFile::from_bytes(b"INFO|start\n  \nNOTICE|bad\n");
    let output = run_cli([input.0.as_os_str()]);
    assert_failure(&output, &["第 3 行", "不支援的層級：NOTICE"]);
}

#[test]
fn missing_separator_reports_original_line() {
    let input = TempFile::from_bytes(b"INFO|start\nbroken\n");
    let output = run_cli([input.0.as_os_str()]);
    assert_failure(&output, &["第 2 行", "缺少 | 分隔符"]);
}

#[test]
fn empty_level_is_rejected() {
    let input = TempFile::from_bytes(b"|message\n");
    let output = run_cli([input.0.as_os_str()]);
    assert_failure(&output, &["第 1 行", "層級不可為空"]);
}

#[test]
fn missing_argument_prints_usage_to_stderr() {
    let output = run_cli(std::iter::empty::<&std::ffi::OsStr>());
    assert_failure(&output, &["用法：", "<PATH>"]);
}

#[test]
fn extra_argument_is_rejected_before_reading() {
    let output = run_cli(["sample.log", "extra"]);
    assert_failure(&output, &["只能提供一個檔案路徑"]);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.starts_with("錯誤：只能提供一個檔案路徑"));
    assert!(!stderr.starts_with("錯誤：錯誤："));
}

#[test]
fn missing_file_reports_read_error_to_stderr() {
    let id = NEXT_FILE_ID.fetch_add(1, Ordering::Relaxed);
    let missing = env::temp_dir().join(format!(
        "rust-log-summary-missing-{}-{id}.log",
        std::process::id()
    ));
    let output = run_cli([missing.as_os_str()]);
    assert_failure(&output, &["無法讀取檔案"]);
}

#[test]
fn invalid_utf8_is_a_read_error_not_a_panic() {
    let input = TempFile::from_bytes(&[0xff, b'\n']);
    let output = run_cli([input.0.as_os_str()]);
    assert_failure(&output, &["無法讀取檔案"]);
}

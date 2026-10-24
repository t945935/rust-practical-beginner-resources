mod analyzer;
mod parser;
mod record;

use std::{env, fs, process};

use crate::analyzer::summarize;
use crate::parser::parse_record;
use crate::record::LogRecord;

fn run() -> Result<(), String> {
    let mut args = env::args_os();
    let program = args.next().unwrap_or_default();
    let Some(path) = args.next() else {
        return Err(format!("用法：{} <PATH>", program.to_string_lossy()));
    };
    if args.next().is_some() {
        return Err("只能提供一個檔案路徑".to_owned());
    }

    let contents = fs::read_to_string(&path)
        .map_err(|error| format!("無法讀取檔案 {}：{error}", path.to_string_lossy()))?;
    let mut records: Vec<LogRecord> = Vec::new();
    for (index, line) in contents.lines().enumerate() {
        if let Some(record) = parse_record(line, index + 1).map_err(|error| error.to_string())? {
            records.push(record);
        }
    }

    let summary = summarize(&records);
    println!("總筆數：{}", summary.total);
    println!("INFO：{}", summary.info);
    println!("WARN：{}", summary.warn);
    println!("ERROR：{}", summary.error);
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("錯誤：{error}");
        process::exit(1);
    }
}

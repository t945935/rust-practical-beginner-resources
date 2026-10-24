# 範例 12：Rust 紀錄摘要 CLI

對應：第九至第十二章的最終專案狀態（模組、檔案解析、測試與 CLI）。這是可直接建置的最終檢查點，不是第九章或第十章中途版本。

## 前提與規格

- Rust stable，Rust 2024 Edition；本書核對基準為 Rust 1.99.1／Cargo 1.99.1、WSL Linux x86_64。
- 使用標準函式庫，不需外部 crate。
- 每個非空資料行為 `LEVEL|MESSAGE`；只接受 `INFO`、`WARN`、`ERROR`，空白行略過。
- 以第一個 `|` 分欄，修剪欄位兩端空白；訊息其餘 `|` 保留。錯誤需指出原始行號。

## 執行

在本資料夾（包含 `Cargo.toml`）執行：

```console
cargo run -- sample.log
```

預期程式輸出：

```text
總筆數：4
INFO：2
WARN：1
ERROR：1
```

成功摘要只寫到 stdout；成功時 stderr 為空、退出狀態為 0。錯誤會寫到 stderr 並以非零狀態結束。例如：

```console
cargo run -- missing.log
```

預期錯誤以 `錯誤：無法讀取檔案 missing.log：` 開頭；作業系統提供的錯誤細節可能不同。

## 測試

```console
cargo test
```

測試涵蓋欄位修剪、訊息內後續分隔符、空白行、原始行號、錯誤層級、stdout/stderr、參數數量、檔案讀取及無效 UTF-8。

## 恢復與授權

若自己的專案檔案與書中版本混在一起，可將本資料夾作為最終檢查點重新開始；不要把這份最終版覆蓋在另一個仍有舊型別的專案上。各章操作仍以書稿指定的檔案和版本為準。

`LICENSE-CODE` 的 MIT 授權僅適用於 `examples/**/*.rs`（含本專案 source 與測試），不涵蓋本 README、sample data 或其他文字資料。

# Rust 實戰入門｜讀者配套資源

本 Repo 提供《Rust 實戰入門：從所有權到命令列工具》的可下載範例、練習與資料檔。它是書籍的補充資源，不包含書稿。

## 下載

- ZIP 下載：[下載 main 分支壓縮檔](https://github.com/t945935/rust-practical-beginner-resources/archive/refs/heads/main.zip)
- Git 複製：

```bash
git clone https://github.com/t945935/rust-practical-beginner-resources.git
```

解壓或複製後，從 Repo 根目錄依各資料夾 README 指示操作。

## 目前狀態

目前提供兩個可獨立執行的 Cargo 範例：第一章 Hello Rust 起始範例，以及第九至第十二章的最終檢查點 `examples/12-rust-log-summary/`。另有 `data/sample.log` 與第十二章練習提示。Hello Rust 範例尚未在本環境執行；最終專案是依書內既有 runtime harness 建立，但本次工作環境找不到 `cargo`／`rustc`，因此尚未在此 Repo 副本重跑測試。請先依 [Rust 官方安裝說明](https://rust-lang.org/zh-TW/tools/install/)安裝工具鏈，再按各範例 README 執行。

書稿技術基準為 Rust 2024 Edition、Rust 1.99.1／Cargo 1.99.1、WSL Linux x86_64。這不代表 Repo 內新增副本已在本次工作階段通過建置，也不宣稱其他版本或平台相容。

## 資源索引

- `examples/00-hello-rust/`：第一章 Cargo Hello World 起始專案。
- `examples/12-rust-log-summary/`：第九至第十二章最終模組化 CLI，含單元與整合測試；不是逐章中途快照。
- `exercises/12-final-project.md`：最終專案練習、預期行為與提示；不是全書逐題答案集。
- `data/sample.log`：合成範例資料。

## 回報問題

回報時請附：書籍／資源版本、作業系統、`rustc --version`、`cargo --version`、執行命令及完整錯誤訊息。不要附上密碼、token、私鑰或個人資料。

## 授權

本 Repo 的 `LICENSE-CODE` 依使用者選定採 MIT，僅適用於 `examples/**/*.rs`（含測試）。不涵蓋 Markdown 說明、練習、sample data、書稿、封面或其他 Repo 內容。版權署名採書籍 metadata 的 `Happy eBook Authors`。

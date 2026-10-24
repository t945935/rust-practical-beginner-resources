# 第十二章：紀錄摘要 CLI 練習與提示

對應：第十二章最終專案。先在 `examples/12-rust-log-summary/` 完成，再用下列行為檢核；該目錄中的實作是參考答案，不必重複貼到本檔。

## 任務

1. 執行 `cargo run -- sample.log`，確認摘要總數為 4，INFO／WARN／ERROR 分別為 2／1／1。
2. 在自己的副本加入一筆 `INFO|cache warmed`，確認總數變成 5，INFO 變成 3。
3. 加入空白行、欄位兩端空白，以及 `WARN|retry | later`；確認空白行不計數，欄位修剪，訊息保留第二個 `|` 之後的文字。
4. 將一行改成 `NOTICE|bad`；確認程式指出該行原始行號、stderr 有錯誤、stdout 沒有部分摘要，且退出狀態非零。
5. 分別測試沒有路徑、多給一個路徑、不存在的檔案，以及無效 UTF-8 檔案。

## 提示與檢核答案

- CLI 的讀檔與參數邊界在 `src/main.rs`；單行規則與行號錯誤在 `src/parser.rs`；分類計數在 `src/analyzer.rs`。
- 第 1 題預期輸出：

  ```text
  總筆數：4
  INFO：2
  WARN：1
  ERROR：1
  ```

- 第 2 題預期摘要：總筆數 5、INFO 3、WARN 1、ERROR 1。
- 第 3 題仍按非空有效紀錄計數；只以第一個 `|` 分欄。
- 第 4、5 題都應是失敗退出；錯誤寫到 stderr，stdout 不含摘要。作業系統提供的檔案錯誤細節可能不同。
- `cargo test` 的 parser/analyzer 單元測試與 `tests/cli.rs` 整合測試覆蓋上述邊界；失敗時以第一個失敗案例定位責任模組。

本練習與提示是 Markdown 文件，不在 `LICENSE-CODE` 對 Rust 程式碼的 MIT 授權範圍內；本地授權尚未推送，亦待權利人確認。

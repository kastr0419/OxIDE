# エージェント作業方針

## 方針

- 設計後、core と UI を並列実装し、レビュー、テストの順で進める。
- core と UI は設計時の型定義を共通契約とする。
- Windows / Linux の両方に対応する。
- エラーは `anyhow::Result`、非同期通信は `crossbeam-channel` に統一する。
- `cargo build` が警告なしで成功し、主要機能を確認して完了とする。
- コミットは Conventional Commits を使い、次のトレーラーを付ける。

`Co-authored-by: Copilot <223556219+Copilot@users.noreply.github.com>`

## 対応ファイル

| 担当 | ファイル |
|---|---|
| core | `Cargo.toml`、`src/core/*`、`src/templates/*` |
| UI | `src/main.rs`、`src/app.rs`、`src/ui/*` |
| ライセンス | `OSS_LICENSE_AUDIT.md`、`LICENSE-*`、`NOTICE` |
| OSS 文書 | `README*`、`CONTRIBUTING.md`、`src/**/*.rs` の SPDX ヘッダー |

```markdown
# スクラムエージェント構成ドキュメント

> rust-embedded-ide プロジェクトで使用した AI スクラムチームの設定記録。
> 同様のプロジェクトを再実行・再現する際の参照用として保存。

---

## チーム構成

| ロール | エージェントID | モデル | 担当フェーズ |
|--------|--------------|--------|-------------|
| 🗂 プロジェクトマネージャー | pm-agent | gpt-5-mini | Sprint計画・タスク分割 |
| 🏗 アーキテクチャ設計者 | arch-agent | gpt-5-mini | モジュール設計・型定義 |
| 💻 プログラマー #1 | dev-agent | gpt-5-mini | core層実装（board/compiler/flasher/serial/config） |
| 💻 プログラマー #2 | dev2-agent | gpt-5-mini | UI層実装（app/editor/board_picker/build_panel/serial_monitor） |
| 🔍 レビュアー | reviewer-agent | gpt-5-mini | コードレビュー・型不一致修正・ビルド検証 |
| 🧪 テスター | tester-agent | gpt-5-mini | 動作確認・回帰テスト |
| ⚖️ ライセンス審査 | license-agent | gpt-5-mini | OSSライセンス審査・OSS_LICENSE_AUDIT.md作成 |
| 📄 OSSセットアップ | oss-setup-agent | gpt-5-mini | LICENSE/README/CONTRIBUTING/SPDXヘッダー整備 |

---

## 実行順序（スプリントフロー）

```
[Phase 0: 準備]
  ├─ license-agent     → OSS_LICENSE_AUDIT.md 作成
  └─ oss-setup-agent   → LICENSE-MIT/APACHE, README, CONTRIBUTING, NOTICE, SPDXヘッダー

[Phase 1: 設計] ← 並列実行
  ├─ pm-agent          → スプリント計画・ディレクトリ構造・依存関係定義
  └─ arch-agent        → モジュール設計・構造体定義・データフロー設計

[Phase 2: 実装] ← 並列実行
  ├─ dev-agent         → core層 (Cargo.toml + src/core/*)
  └─ dev2-agent        → UI層 (src/ui/* + src/app.rs + src/main.rs)

[Phase 3: 統合・修正]
  └─ reviewer-agent    → 型不一致解消・cargo build 成功まで修正

[Phase 4: テスト]
  └─ tester-agent      → 起動確認・機能テスト・回帰テスト
```

---

## エージェント個別設定

### 🗂 プロジェクトマネージャー (pm-agent)

**モデル**: `gpt-5-mini`  
**役割**: 要件整理、スプリント計画、タスク分割、各メンバーへの指示  
**主要な指示内容**:
- プロジェクト概要・技術スタックの把握
- フェーズ分けしたスプリント計画の作成
- ディレクトリ構造の提案
- 依存クレート（Cargo.toml）の選定
- 不明点をユーザーに質問事項としてまとめる

**出力物**: スプリント計画、ディレクトリ構造案、Cargo.toml依存関係一覧、各メンバーへの指示

---

### 🏗 アーキテクチャ設計者 (arch-agent)

**モデル**: `gpt-5-mini`  
**役割**: モジュール設計、構造体・トレイト定義、データフロー、非同期処理設計  
**主要な指示内容**:
- 各モジュールの責務定義（app/editor/board/compiler/flasher/serial/config）
- Rustコードのスケルトン定義（実装なし、型定義のみ）
- コンパイル→書き込み→シリアルのデータフロー図
- バックグラウンドスレッド + crossbeam-channel による非同期設計
- ボード別設定（target triple、フラッシュツール、引数）の定数定義

**出力物**: アーキテクチャ設計ドキュメント（Markdown）、全モジュールのRustスケルトンコード

---

### 💻 プログラマー #1 (dev-agent)

**モデル**: `gpt-5-mini`  
**役割**: コア層の完全実装  
**担当ファイル**:
```
Cargo.toml
src/main.rs
src/core/mod.rs
src/core/board.rs      ← BoardKind, BoardPreset, BOARD_PRESETS定数
src/core/compiler.rs   ← cargo build実行、バックグラウンドスレッド
src/core/flasher.rs    ← avrdude/esptool/probe-rs 呼び出し
src/core/serial.rs     ← serialport クレート、接続/送受信
src/core/config.rs     ← TOML設定保存/読み込み
src/templates/mod.rs   ← 新規プロジェクトテンプレート生成
```
**主要な指示内容**:
- 対応ボード: Arduino Uno/Nano, ESP32, STM32F4
- OS: Windows + Linux 両対応（`std::path::Path` 使用）
- エラーは `anyhow::Result` で統一
- `cargo build` が成功するまで修正
- フェーズごとにgit commit

---

### 💻 プログラマー #2 (dev2-agent)

**モデル**: `gpt-5-mini`  
**役割**: UI層の完全実装  
**担当ファイル**:
```
src/main.rs            ← eframe::run_native エントリーポイント
src/app.rs             ← IdeApp (eframe::App実装), AppMessage enum, メインレイアウト
src/ui/mod.rs
src/ui/editor.rs       ← egui::TextEdit, ファイル開く/保存 (rfd)
src/ui/board_picker.rs ← ComboBox でボード・ポート選択
src/ui/build_panel.rs  ← Build/Flash/Build&Flash ボタン、ビルドログ
src/ui/serial_monitor.rs ← 接続/切断、受信ログ、送信入力
src/ui/settings.rs     ← ワークスペース設定、テーマ切替
```
**レイアウト**:
```
┌─────────────────────────────────────┐
│ MenuBar [File | Build | Help]        │
├──────────┬──────────────┬───────────┤
│ Board    │              │  Serial   │
│ Picker   │   Editor     │  Monitor  │
│ (250px)  │  (中央拡張)   │  (300px)  │
│ Build    │              │           │
│ Panel    │              │           │
├──────────┴──────────────┴───────────┤
│ StatusBar                           │
└─────────────────────────────────────┘
```
**主要な指示内容**:
- `crossbeam_channel` で core の非同期結果を受信
- `AppMessage` enum で全バックグラウンド結果を統一
- core層は #1 が実装中の前提で、型定義を契約として参照

---

### 🔍 レビュアー (reviewer-agent)

**モデル**: `gpt-5-mini`  
**役割**: 型不一致の発見・修正、ビルド成功まで繰り返し修正  
**主要な指示内容**:
- #1 と #2 の実装で生じた型不一致を全て洗い出す
- UIが期待するインターフェースにcoreを合わせる方針で修正
- `cargo build 2>&1` を実行してエラーを確認・修正（最大5回ループ）
- 修正後に git commit

**典型的な修正内容**:
- `BuildResult`/`FlashResult`/`SerialLine`/`SerialCommand` などの型をcoreに追加
- `build_async`/`flash_async`/`connect_async` などの非同期ラッパー関数を追加
- egui APIバージョン差異の修正（`ui.input(|i| ...)` 形式など）
- `AppConfig` フィールドの不一致修正

---

### 🧪 テスター (tester-agent)

**モデル**: `gpt-5-mini`  
**役割**: ビルド成功後の動作確認・テストケース作成  
**チェック項目**:
- `cargo build` が警告なしで通ること
- アプリケーションが起動すること
- 各UIパネルが表示されること
- ボード選択・ポート選択が機能すること
- シリアル接続/切断のエラーハンドリング
- 設定保存/読み込みが機能すること

---

### ⚖️ ライセンス審査 (license-agent)

**モデル**: `gpt-5-mini`  
**役割**: 依存クレートのOSSライセンス調査・判断資料作成  
**調査項目**: ライセンス種別、商用利用可否、コピーレフト性、表示義務
**判断基準**: MIT OR Apache-2.0 デュアルライセンスで公開、商用利用の可能性あり
**出力物**: `OSS_LICENSE_AUDIT.md`

---

### 📄 OSSセットアップ (oss-setup-agent)

**モデル**: `gpt-5-mini`  
**役割**: プロジェクトのOSSとしての公開準備  
**作成ファイル**:
- `LICENSE-MIT` — MIT License全文
- `LICENSE-APACHE` — Apache License 2.0全文
- `NOTICE` — 著作権表示・第三者ライセンス注記
- `CONTRIBUTING.md` — コントリビューションガイドライン
- `README.md` — プロジェクト概要・使い方・ライセンスバッジ
- `Cargo.toml` — `license`, `description`, `repository`, `keywords`, `categories` 追記
- `src/**/*.rs` — SPDXライセンスヘッダー追加

---

## Git コミット規則

```
feat:     新機能追加
fix:      バグ修正
docs:     ドキュメント変更
refactor: リファクタリング
chore:    ビルド設定・雑務
test:     テスト追加・修正
```

全コミットに以下のトレーラーを付与:
```
Co-authored-by: Copilot <223556219+Copilot@users.noreply.github.com>
```

---

## エージェント再実行時の注意事項

1. **並列実行できるもの**: pm-agent と arch-agent、dev-agent と dev2-agent
2. **順序依存**: reviewer は dev/dev2 の完了後、tester は reviewer の完了後
3. **型契約の共有**: dev-agent と dev2-agent はアーキテクチャ設計者の出力（構造体定義）を契約として共有すること
4. **ビルド環境**: Windows では AppLocker/WDAC によりビルドスクリプトがブロックされる場合がある。その場合は WSL または管理者権限での実行を検討
5. **モデル指定**: `gpt-5-mini` を使用（`gpt-5.4-mini` とは異なる）

---

## プロジェクト固有設定

```toml
# 使用した主要クレート
eframe = "0.31"       # GUI フレームワーク
egui = "0.31"         # UI ウィジェット
serialport = "4"      # シリアル通信
crossbeam-channel = "0.5"  # スレッド間通信
serde = "1"           # シリアライズ
toml = "0.8"          # 設定ファイル
rfd = "0.15"          # ファイルダイアログ
anyhow = "1"          # エラー処理
dirs = "6"            # 設定ディレクトリ
which = "7"           # 外部ツール検索
parking_lot = "0.12"  # 軽量ロック
```

```
# 対応ボード
Arduino Uno  : target = avr-atmega328p      , flasher = avrdude
Arduino Nano : target = avr-atmega328p      , flasher = avrdude
ESP32        : target = xtensa-esp32-none-elf, flasher = esptool.py
STM32F4      : target = thumbv7em-none-eabihf, flasher = probe-rs
```
```

---

## 作成手順

```powershell
cd D:\rust_embedded
# ファイルに書き込む（上記内容をそのまま Set-Content で保存）
# 保存後:
git add AGENTS.md
git commit -m "docs: add scrum agent configuration document (AGENTS.md)

Co-authored-by: Copilot <223556219+Copilot@users.noreply.github.com>"
```

上記内容を `D:\rust_embedded\AGENTS.md` に保存し、git commit してください。

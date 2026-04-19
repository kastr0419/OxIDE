# Coding Guidelines — rust-embedded-ide

このドキュメントは `rust-embedded-ide` の開発で使用したコーディングルール・設計規約をまとめたものです。

---

## 1. ファイル先頭のライセンスヘッダー

すべての `.rs` ファイルの先頭に SPDX ヘッダーを付与します。

```rust
// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright 2026 rust-embedded-ide contributors
```

---

## 2. ディレクトリ構成

```
src/
├── main.rs          — eframe::run_native エントリーポイント
├── lib.rs           — ライブラリルート（テスト用）
├── app.rs           — IdeApp 構造体・AppMessage enum・update() レイアウト
├── core/            — バックエンドロジック（UI非依存）
│   ├── board.rs         BoardKind / BoardPreset / BOARD_PRESETS 定数
│   ├── compiler.rs      cargo build 実行
│   ├── flasher.rs       avrdude / esptool / probe-rs 書き込み
│   ├── serial.rs        serialport 接続・送受信
│   ├── config.rs        TOML 設定保存・読み込み
│   ├── build_analyzer.rs ELF サイズ解析
│   ├── elf_analyzer.rs  ELF セクション・シンボル解析
│   ├── stack_analyzer.rs nm ベーススタック推定
│   ├── debugger.rs      probe-rs デバッグスレッド
│   ├── svd_parser.rs    SVD XML パーサ
│   ├── pinout.rs        ボードピン定義
│   ├── lsp.rs           rust-analyzer LSP クライアント
│   ├── detector.rs      ボード自動検出
│   ├── toolchain.rs     ツールチェーン管理
│   ├── project.rs       プロジェクト作成・保存・読み込み
│   └── snippets.rs      コードスニペット
└── ui/              — egui パネル実装（core を呼び出す）
    ├── editor.rs        コードエディタ（補完・インデント・ブラケット）
    ├── build_panel.rs   ビルド・フラッシュ・サイズメーター
    ├── board_picker.rs  ボード・ポート選択
    ├── serial_monitor.rs シリアルモニタ
    ├── serial_plotter.rs シリアルプロッタ
    ├── debug_panel.rs   デバッグパネル（レジスタ・メモリ・RTT）
    ├── rtt_panel.rs     RTT ログビューア
    ├── svd_panel.rs     SVD 周辺レジスタビューア
    ├── elf_panel.rs     ELF ビューア
    ├── stack_panel.rs   スタック解析パネル
    ├── pinout_panel.rs  ピンアウト表示
    ├── help_panel.rs    ドキュメントビューア
    ├── settings.rs      設定画面
    ├── file_explorer.rs ファイルツリー
    └── fonts.rs         フォント設定
docs/                — ユーザードキュメント（Markdown / 日本語）
```

---

## 3. レイヤー分離ルール

| レイヤー | 場所 | ルール |
|---------|------|--------|
| Core | `src/core/` | `egui` / `eframe` に **依存しない**。I/O・スレッド・ロジックのみ |
| UI | `src/ui/` | `egui::Ui` を受け取り描画する。重い処理は core に委譲 |
| App | `src/app.rs` | Core と UI の接続点。`IdeApp` 状態・メッセージルーティング |

---

## 4. バックグラウンドスレッド + メッセージパッシング

### パターン

重い処理（ビルド・フラッシュ・LSP・デバッグ）はすべて別スレッドで実行し、
結果を `crossbeam_channel` 経由で UI スレッドに送ります。

```rust
// core 側：非同期ラッパー
pub fn build_async(req: BuildRequest, tx: Sender<AppMessage>) {
    std::thread::spawn(move || {
        let result = /* 重い処理 */;
        let _ = tx.send(AppMessage::Build(BuildMsg::Finished(result)));
    });
}
```

```rust
// app.rs 側：毎フレーム受信
fn handle_messages(&mut self) {
    while let Ok(msg) = self.msg_rx.try_recv() {
        match msg {
            AppMessage::Build(BuildMsg::Finished(br)) => { /* 状態更新 */ }
            AppMessage::Flash(FlashMsg::Finished(fr)) => { /* 状態更新 */ }
            // ... 全バリアントを網羅
        }
    }
}
```

### AppMessage enum

新機能追加時は `AppMessage` に新しいバリアントを追加し、
`handle_messages()` の `match` を必ず網羅してください（コンパイラが漏れを検出）。

---

## 5. エラーハンドリング

- Core 層の関数戻り値は `anyhow::Result<T>` を使用
- UI 層ではエラーを `app.build_log` や `app.status_message` に文字列で表示
- パニック（`unwrap()` の乱用）は禁止。`?` 演算子か `unwrap_or_default()` を使用

```rust
// Good
pub fn analyze_elf(path: &Path) -> anyhow::Result<ElfInfo> {
    let data = std::fs::read(path)?;
    let obj = object::File::parse(&*data)?;
    Ok(ElfInfo { /* ... */ })
}

// Bad
let data = std::fs::read(path).unwrap();
```

---

## 6. IdeApp の状態管理

### フィールド命名規則

| プレフィックス | 意味 | 例 |
|--------------|------|-----|
| `is_` | 現在の動作状態（bool） | `is_building`, `is_flashing` |
| `show_` | パネル・ウィンドウ表示フラグ（bool） | `show_elf_panel`, `show_help_window` |
| `selected_` | 選択インデックス（usize） | `selected_board`, `selected_port` |
| `last_` | 最後の結果キャッシュ | `last_dist_path` |
| `_log` | ログ文字列 | `build_log`, `rtt_log` |
| `_filter` | 検索フィルタ文字列 | `stack_filter` |

### 自動フラグパターン

「アクション A 成功後に B を自動実行」する場合は `auto_B_after_A` フラグを使用：

```rust
// Build & Flash の例
pub auto_flash_after_build: bool,

// ビルド成功時
if self.auto_flash_after_build {
    self.auto_flash_after_build = false;
    // flash 起動
}
// ビルド失敗時
self.auto_flash_after_build = false; // 必ずリセット
```

---

## 7. BoardPreset パターン

ボード固有の設定はすべて `BOARD_PRESETS` 定数スライスに集約します。
新しいボードを追加する場合は以下のフィールドをすべて埋めてください。

```rust
BoardPreset {
    kind:          BoardKind::ArduinoUno,
    display_name:  "Arduino Uno",
    target_triple: "avr-atmega328p",
    flash_tool:    FlashTool::Avrdude,
    rustflags:     &["-C", "target-cpu=atmega328p"],
    flash_bytes:   32 * 1024,   // 32 KB
    ram_bytes:     2 * 1024,    //  2 KB
    cpu_arch:      CpuArch::AvrMega,
}
```

---

## 8. UI パネル実装規則

各パネルは `pub fn ui_xxx_panel(app: &mut IdeApp, ui: &mut egui::Ui)` の形で実装します。

```rust
// Good — 関数シグネチャ統一
pub fn ui_debug_panel(app: &mut IdeApp, ui: &mut egui::Ui) { /* ... */ }
pub fn ui_build_panel(app: &mut IdeApp, ui: &mut egui::Ui) { /* ... */ }

// モーダルウィンドウは app.rs の update() 内で開く
if self.show_elf_panel {
    egui::Window::new("📂 ELF Viewer")
        .open(&mut self.show_elf_panel)
        .default_size([700.0, 500.0])
        .show(ctx, |ui| {
            crate::ui::elf_panel::ui_elf_panel(self, ui);
        });
}
```

### egui API 注意点（0.31）

| 非推奨 | 推奨 |
|--------|------|
| `ComboBox::from_id_source(...)` | `ComboBox::from_id_salt(...)` |
| `ui.input().key_pressed(...)` | `ui.input(\|i\| i.key_pressed(...))` |
| `ui.input_mut(...).events` | `ui.input_mut(\|i\| i.events.retain(...))` |
| `egui::Window::new(...).show(...)` (位置未指定) | `.fixed_pos(pos)` または `.default_pos(pos)` を指定 |

---

## 9. LSP 接続の初期化順序

rust-analyzer との接続は必ず以下の順序で行います。

```
initialize → (rust-analyzerからレスポンス受信) → initialized通知 → did_open
```

`did_open` は `LspMessage::Initialized` を受信してから送信すること。
`pending_did_opens` バッファで初期化完了前のリクエストを保持します。

---

## 10. 依存クレート選定基準

| 用途 | クレート | 理由 |
|------|---------|------|
| GUI | `eframe 0.31` + `egui 0.31` | 組み込み向けシンプルな即時描画 |
| スレッド間通信 | `crossbeam-channel 0.5` | 標準より高速・複数送受信者対応 |
| エラー処理 | `anyhow 1.0` | Context付きエラー伝播が容易 |
| シリアル通信 | `serialport 4.0` | クロスプラットフォーム対応 |
| 設定保存 | `toml 0.5` + `serde 1.0` | 人間が読みやすい設定ファイル |
| ファイルダイアログ | `rfd 0.10` | egui と相性が良いネイティブダイアログ |
| ELF 解析 | `object 0.36` | probe-rs と依存バージョン互換 |
| SVD 解析 | `roxmltree 0.20` | 軽量・proc-macro 不要 |
| デバッグ | `probe-rs 0.26` | ARM Cortex-M 対応の標準ツール |
| グラフ | `egui_plot 0.31` | egui 同バージョンで互換性保証 |
| 外部ツール検索 | `which 4` | PATH から実行ファイルを安全に検索 |

---

## 11. Git コミット規則

### コミットメッセージ形式

```
<type>: <概要（日本語 or 英語）>

[本文（任意）]

Co-authored-by: Copilot <223556219+Copilot@users.noreply.github.com>
```

### type 一覧

| type | 用途 |
|------|------|
| `feat` | 新機能追加 |
| `fix` | バグ修正 |
| `docs` | ドキュメント変更 |
| `refactor` | リファクタリング（動作変更なし） |
| `chore` | ビルド設定・依存更新 |
| `test` | テスト追加・修正 |

---

## 12. スクラムチーム構成（AI エージェント）

大きな機能追加は以下のスクラムチームで実施します。詳細は `AGENTS.md` 参照。

| ロール | モデル | 主な責務 |
|--------|--------|---------|
| PM | gpt-5-mini | 要件整理・スプリント計画・タスク分割 |
| Architecture | gpt-5-mini | モジュール設計・型定義スケルトン |
| Programmer #1 | gpt-5-mini | core 層実装 |
| Programmer #2 | gpt-5-mini | UI 層実装 |
| Reviewer | gpt-5-mini | `cargo build` 成功まで修正 |
| Tester | gpt-5-mini | 静的コードレビュー・ロジック検証 |

### 並列実行ルール

- PM と Arch は並列実行可
- Programmer #1 と #2 は並列実行可（型契約を事前に共有）
- Reviewer は全 Programmer 完了後
- Tester は Reviewer 完了後

---

## 13. 命名規則

Rust 標準の命名規約（RFC 430）に従いつつ、このプロジェクト固有のパターンを追加します。

### 基本ケース

| 対象 | ケース | 例 |
|------|--------|-----|
| 関数・メソッド | `snake_case` | `build_async`, `find_elf` |
| 変数・引数 | `snake_case` | `elf_path`, `board_kind` |
| 構造体・enum・型 | `PascalCase` | `IdeApp`, `BoardKind`, `ElfInfo` |
| 定数 (`const`) | `SCREAMING_SNAKE_CASE` | `BOARD_PRESETS`, `UNO_USB_IDS` |
| 静的変数 (`static`) | `SCREAMING_SNAKE_CASE` | `ARDUINO_UNO_PINOUT` |
| モジュール | `snake_case` | `build_analyzer`, `elf_panel` |
| ライフタイム | 短い小文字 | `'a`, `'static` |

---

### 関数命名パターン

#### Core 層

| パターン | 用途 | 例 |
|---------|------|-----|
| `xxx_async` | バックグラウンドスレッド起動 | `build_async`, `flash_async`, `connect_async` |
| `analyze_xxx` | データ解析・パース処理 | `analyze_elf`, `analyze_stack` |
| `parse_xxx` | テキスト・バイナリ解析 | `parse_svd`, `parse_plot_line` |
| `find_xxx` | ファイル・リソース検索 | `find_elf` |
| `get_xxx` | データ取得（Option/Vec 返却） | `get_pinout`, `get_snippets` |
| `detect_by_xxx` | 自動検出 | `detect_by_usb_id`, `detect_by_probe_rs` |
| `check_xxx` | 存在・状態確認（bool 返却） | `check_rust_analyzer` |
| `validate_xxx` | バリデーション（bool 返却） | `validate_custom_path` |
| `spawn_xxx` | スレッド生成 | `spawn_debugger` |
| `list_xxx` | 一覧取得 | `list_ports` |
| `open_xxx` | ファイル・リソースを開く | `open_project` |

#### UI 層

| パターン | 用途 | 例 |
|---------|------|-----|
| `ui_xxx_panel` | パネル描画関数（メイン） | `ui_debug_panel`, `ui_build_panel` |
| `ui_xxx` | サブUI（パネル以外） | `ui_settings`, `ui_editor` |

#### テンプレート層

ボード別テンプレート生成関数は **ボード名をそのまま** `snake_case` で使用：

```rust
pub fn arduino_uno() -> BlinkTemplate { ... }
pub fn stm32f4() -> BlinkTemplate { ... }
pub fn microbit_v2() -> BlinkTemplate { ... }
```

---

### 構造体命名パターン

| パターン | 用途 | 例 |
|---------|------|-----|
| `XxxRequest` | バックグラウンド処理への入力 | `BuildRequest`, `FlashRequest` |
| `XxxResult` | バックグラウンド処理の出力 | `BuildResult`, `FlashResult` |
| `XxxInfo` | 解析・読み取り結果データ | `ElfInfo`, `PinInfo` |
| `XxxReport` | 集計・分析レポート | `StackReport` |
| `XxxHandle` | リソースハンドル（RAII） | `SerialHandle` |
| `XxxPreset` | 静的設定データ | `BoardPreset` |
| `XxxLayout` | メモリ・画面レイアウト | `MemoryLayout` |
| `XxxFrame` | スタック・コールフレーム | `StackFrame` |
| `XxxTab` | タブ状態 | `FileTab` |
| `XxxChannel` | データチャンネル | `PlotChannel` |

---

### enum 命名パターン

| パターン | 用途 | 例 |
|---------|------|-----|
| `XxxKind` | 種類・分類（判別のみ） | `BoardKind`, `FlashToolKind`, `CpuArch` |
| `XxxMsg` | バックグラウンド→UI への粒度細かいメッセージ | `BuildMsg`, `FlashMsg`, `SerialMsg` |
| `AppMessage` | アプリ全体のトップレベルメッセージ（単数形） | `AppMessage` |
| `XxxCommand` | UI→バックグラウンドへの命令 | `DebugCommand` |
| `XxxEvent` | バックグラウンド→UI への通知 | `DebugEvent`, `SerialEvent` |
| `XxxFunction` | 機能・役割の分類 | `PinFunction` |
| `XxxStatus` | 状態 | `RustAnalyzerStatus` |
| `XxxTab` | タブ選択状態 | `RightTab` |

---

### 引数・変数命名規則

#### 予約済み引数名（このプロジェクト全体で統一）

| 引数名 | 型 | 用途 |
|--------|-----|------|
| `tx` | `Sender<AppMessage>` | バックグラウンド→UI メッセージ送信チャンネル |
| `rx` | `Receiver<AppMessage>` | UI 側受信チャンネル |
| `req` | `XxxRequest` | バックグラウンド処理リクエスト |
| `ui` | `&mut egui::Ui` | egui UI コンテキスト |
| `ctx` | `&egui::Context` | egui アプリコンテキスト |
| `app` | `&mut IdeApp` | アプリ全体の状態（UI 関数に渡す） |
| `cc` | `&eframe::CreationContext` | eframe 初期化コンテキスト |
| `elf_path` | `&Path` | ELF ファイルパス |
| `target_dir` | `&Path` | cargo ビルド出力ディレクトリ |
| `workspace` | `PathBuf` | プロジェクトルートディレクトリ |
| `board` | `&BoardPreset` | ボードプリセット参照 |
| `board_kind` | `BoardKind` | ボード種別（値渡し） |
| `port` | `&str` | シリアルポート名（例: "COM3", "/dev/ttyUSB0"） |

#### ローカル変数

- ループカウンタ: `i`, `j`（短い範囲のみ。複雑なループは `idx` / `row_idx`）
- イテレータ要素: 意味のある名前（`entry`, `line`, `part` など）
- 一時バッファ: `buf`, `data`, `output`
- 真偽フラグ: `is_xxx` / `has_xxx` / `show_xxx`（形容詞/動詞 + 対象）

---

### 定数命名規則

```rust
// ボード別 USB ID リスト — "BOARD名_USB_IDS" 形式
const UNO_USB_IDS: &[UsbId] = &[...];
const MICROBIT_V2_USB_IDS: &[UsbId] = &[...];

// ボードプリセット全体 — 単数名詞 + S（スライス）
pub const BOARD_PRESETS: &[BoardPreset] = &[...];

// ピンアウト静的データ — "BOARD名_PINOUT" 形式
pub static ARDUINO_UNO_PINOUT: BoardPinout = BoardPinout { ... };
```

---

### 命名の禁止事項

```rust
// ❌ 略語は使わない（ただし確立された略語は OK: tx/rx/lsp/elf/svd/rtt）
let bd = BOARD_PRESETS[0];          // Bad
let board = BOARD_PRESETS[0];       // Good

// ❌ 型名をそのまま変数名にしない
let string = "hello".to_string();   // Bad
let label = "hello".to_string();    // Good

// ❌ 数字サフィックスで意味を区別しない
let result1 = analyze_elf(...);     // Bad
let elf_sections = analyze_elf(...)// Good

// ❌ ネガティブ名（否定系）は避ける
let not_connected = true;           // Bad
let is_disconnected = true;         // Good（やむを得ない場合）
let is_connected = false;           // Better

// ✅ 確立された略語はそのまま使う
tx, rx          // チャンネル送受信
lsp             // Language Server Protocol
elf             // Executable and Linkable Format
svd             // System View Description
rtt             // Real-Time Transfer
ui              // User Interface
ctx             // Context
```

---

## 14. ドキュメント規則

- `docs/*.md` はユーザー向け、**日本語優先**
- `include_str!("../../docs/xxx.md")` で `help_panel.rs` に組み込み
- 新しい docs ファイルを追加したら `DOCS` 定数配列にも追加すること
- コードコメントは **必要な箇所のみ**（自明な処理にはコメント不要）

---

## 14. Windows 固有の注意点

- パスは `std::path::PathBuf` / `Path` を使用（文字列結合禁止）
- 外部ツール呼び出しは `which` クレートで実行ファイルを検索してから実行
- `arm-none-eabi-size` / `nm` は `PATH` に含まれていない場合があるため `which` で確認
- probe-rs（ST-Link）を使う場合は WinUSB ドライバが必要（Zadig ツール）
- AppLocker / WDAC 環境では `cargo build` のビルドスクリプトがブロックされる場合あり

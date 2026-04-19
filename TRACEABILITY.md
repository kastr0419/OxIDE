# トレーサビリティマトリクス — rust-embedded-ide

> バージョン: 0.1  
> 作成日: 2026-04-19  
> 要求仕様書: REQUIREMENTS.md  
> アーキテクチャ設計書: ARCHITECTURE.md

トップダウン方向（要求 → アーキテクチャ → ソースコード → シンボル）で追跡します。

---

## 読み方

| 列 | 説明 |
|----|------|
| **要件 ID** | REQUIREMENTS.md の FR-X-Y / NFR-X-Y |
| **アーキテクチャ層** | Core / App / UI |
| **モジュール** | `src/` 以下のモジュールパス |
| **ソースファイル** | 実装ファイル（複数の場合は改行） |
| **主要シンボル** | 型・関数・定数名 |

---

## FR-1: コードエディタ

| 要件 ID | アーキテクチャ層 | モジュール | ソースファイル | 主要シンボル |
|---------|----------------|-----------|--------------|------------|
| FR-1-1 ファイル開く・保存 | App, UI | app, ui::editor | src/app.rs, src/ui/editor.rs | `IdeApp::file_path`, `write_or_log()`, `rfd::FileDialog` |
| FR-1-2 複数タブ | App, UI | app, ui::editor | src/app.rs, src/ui/editor.rs | `IdeApp::open_tabs: Vec<FileTab>`, `IdeApp::active_tab` |
| FR-1-3 未保存マーク | App, UI | app | src/app.rs, src/ui/editor.rs | `FileTab::is_dirty`, `IdeApp::is_dirty` |
| FR-1-4 ファイルツリー | UI | ui::file_explorer | src/ui/file_explorer.rs | `ui_file_explorer()`, `IdeApp::workspace_files` |
| FR-1-5 シンタックスHL | UI | ui::editor | src/ui/editor.rs | `egui_extras` (syntect feature) |
| FR-1-6 カーソル位置 | App, UI | app | src/app.rs | `IdeApp::cursor_line`, `cursor_col`, `cursor_char_idx` |
| FR-1-7 Tab インデント | UI | ui::editor | src/ui/editor.rs | `ui.input_mut(\|i\| ...)` イベント処理 |
| FR-1-8 ブラケット自動補完 | UI | ui::editor | src/ui/editor.rs | `ui.input_mut(\|i\| i.events)` ブラケット挿入ロジック |
| FR-1-9 スニペット | Core, App | core::snippets | src/core/snippets.rs | `get_snippets()`, `IdeApp::snippet_query` |
| FR-1-10 ブレークポイント | App, UI | app | src/app.rs, src/ui/editor.rs | `IdeApp::breakpoints: HashSet<usize>` |

---

## FR-2: プロジェクト管理

| 要件 ID | アーキテクチャ層 | モジュール | ソースファイル | 主要シンボル |
|---------|----------------|-----------|--------------|------------|
| FR-2-1 新規プロジェクト | Core, App, UI | core::templates, core::project | src/templates/mod.rs, src/core/project.rs, src/app.rs | `IdeApp::show_new_project_dialog`, `new_project_name`, テンプレート生成関数 |
| FR-2-2 プロジェクトを開く | Core, App | core::project | src/core/project.rs, src/app.rs | `open_project()`, `rfd::FileDialog::pick_folder()` |
| FR-2-3 設定の保存・読み込み | Core | core::config | src/core/config.rs | `AppConfig`, `AppConfig::load()`, `AppConfig::save()` |
| FR-2-4 新規ファイル作成 | App, UI | app | src/app.rs | `IdeApp::show_new_file_dialog`, `new_file_name`, `std::fs::write()` |

---

## FR-3: ボード・ポート選択

| 要件 ID | アーキテクチャ層 | モジュール | ソースファイル | 主要シンボル |
|---------|----------------|-----------|--------------|------------|
| FR-3-1 ボード選択 | Core, App, UI | core::board, ui::board_picker | src/core/board.rs, src/ui/board_picker.rs | `BOARD_PRESETS`, `IdeApp::selected_board`, `egui::ComboBox` |
| FR-3-2 ポート選択 | Core, App, UI | core::serial, ui::board_picker | src/core/serial.rs, src/ui/board_picker.rs | `list_ports()`, `IdeApp::available_ports`, `selected_port` |
| FR-3-3 USB 自動検出 | Core, App | core::detector | src/core/detector.rs | `detect_by_usb_id()`, `DetectedBoard`, `DetectionConfidence` |
| FR-3-4 probe-rs 自動検出 | Core, App | core::detector | src/core/detector.rs | `detect_by_probe_rs()`, `probe_rs::probe::list::Lister` |
| FR-3-5 検出通知 | App | app | src/app.rs | `IdeApp::detection_result`, `AppMessage::BoardDetected` |

---

## FR-4: コンパイル

| 要件 ID | アーキテクチャ層 | モジュール | ソースファイル | 主要シンボル |
|---------|----------------|-----------|--------------|------------|
| FR-4-1 cargo build 実行 | Core | core::compiler | src/core/compiler.rs | `build_async()`, `BuildRequest`, `std::process::Command` |
| FR-4-2 ビルドログ表示 | App, UI | app, ui::build_panel | src/app.rs, src/ui/build_panel.rs | `IdeApp::build_log`, `AppMessage::Build(BuildMsg::Progress)` |
| FR-4-3 成功・失敗表示 | App, UI | app, ui::build_panel | src/app.rs, src/ui/build_panel.rs | `BuildResult::success`, ステータスバー文字列 |
| FR-4-4 ビルド中無効化 | App, UI | app, ui::build_panel | src/app.rs, src/ui/build_panel.rs | `IdeApp::is_building` |
| FR-4-5 Flash 使用量バー | Core, App, UI | core::build_analyzer, ui::build_panel | src/core/build_analyzer.rs, src/ui/build_panel.rs | `BuildStats::flash_used`, `flash_total`, プログレスバー |
| FR-4-6 RAM 使用量バー | Core, App, UI | core::build_analyzer, ui::build_panel | src/core/build_analyzer.rs, src/ui/build_panel.rs | `BuildStats::ram_used`, `ram_total` |

---

## FR-5: マイコンへの書き込み

| 要件 ID | アーキテクチャ層 | モジュール | ソースファイル | 主要シンボル |
|---------|----------------|-----------|--------------|------------|
| FR-5-1 書き込み実行 | Core | core::flasher | src/core/flasher.rs | `flash()`, `BoardPreset::flash_tool` |
| FR-5-2 書き込みツール分岐 | Core | core::flasher | src/core/flasher.rs | `FlashToolKind` enum, match 分岐（Avrdude/Esptool/ProbeRs/Picotool/Bossac/…） |
| FR-5-3 書き込みログ | App, UI | app, ui::build_panel | src/app.rs, src/ui/build_panel.rs | `AppMessage::Flash(FlashMsg::Progress)`, `FlashMessage::Finished` |
| FR-5-4 Build & Flash | App | app | src/app.rs | `IdeApp::auto_flash_after_build` |
| FR-5-5 ELF パス自動検索 | Core | core::compiler | src/core/compiler.rs | `BuildResult::dist_path`, `find_elf()` |
| FR-5-6 非 UTF-8 パス対応 | Core | core::flasher | src/core/flasher.rs | `std::path::PathBuf` 使用（文字列変換なし） |

---

## FR-6: シリアルモニタ

| 要件 ID | アーキテクチャ層 | モジュール | ソースファイル | 主要シンボル |
|---------|----------------|-----------|--------------|------------|
| FR-6-1 接続・切断 | Core, App | core::serial, app | src/core/serial.rs, src/app.rs | `connect()`, `SerialHandle::stop_tx`, `IdeApp::is_serial_connected` |
| FR-6-2 受信ログ 500 行 | App, UI | app, ui::serial_monitor | src/app.rs, src/ui/serial_monitor.rs | `IdeApp::serial_log: VecDeque<String>`（最大 500 要素） |
| FR-6-3 送信 | App, UI | app, ui::serial_monitor | src/app.rs, src/ui/serial_monitor.rs | `IdeApp::serial_input`, `SerialHandle::write_tx.send()` |
| FR-6-4 ログクリア | UI | ui::serial_monitor | src/ui/serial_monitor.rs | `serial_log.clear()` |
| FR-6-5 接続状態インジケーター | App, UI | app, ui::serial_monitor | src/app.rs, src/ui/serial_monitor.rs | `IdeApp::is_serial_connected` |
| FR-6-6 エラー自動検知 | Core, App | core::serial, app | src/core/serial.rs, src/app.rs | `SerialEvent::Error`, `AppMessage::Serial(SerialMsg::Error)` |

---

## FR-7: シリアルプロッタ

| 要件 ID | アーキテクチャ層 | モジュール | ソースファイル | 主要シンボル |
|---------|----------------|-----------|--------------|------------|
| FR-7-1 リアルタイムグラフ | App, UI | app, ui::serial_plotter | src/app.rs, src/ui/serial_plotter.rs | `IdeApp::plot_channels: HashMap<String, PlotChannel>`, `egui_plot` |
| FR-7-2 データ解析 | App | app | src/app.rs | `parse_plot_line()` |
| FR-7-3 色分け表示 | App, UI | app, ui::serial_plotter | src/app.rs, src/ui/serial_plotter.rs | `PlotChannel::color: Color32` |
| FR-7-4 最大保持数設定 | App, UI | app | src/app.rs | `IdeApp::plot_max_points`, `IdeApp::plot_paused` |

---

## FR-8: ピンアウト表示

| 要件 ID | アーキテクチャ層 | モジュール | ソースファイル | 主要シンボル |
|---------|----------------|-----------|--------------|------------|
| FR-8-1 ダイアグラム表示 | Core, UI | core::pinout, ui::pinout_panel | src/core/pinout.rs, src/ui/pinout_panel.rs | `ui_diagram()`, `BoardPinout::pins`, `PinInfo::{x,y}` |
| FR-8-2 テーブル表示 | UI | ui::pinout_panel | src/ui/pinout_panel.rs | `ui_table()`, `IdeApp::pinout_view_table` |
| FR-8-3 機能フィルタ | Core, UI | core::pinout, ui::pinout_panel | src/core/pinout.rs, src/ui/pinout_panel.rs | `pin_visible()`, `IdeApp::pinout_filter`, `ui_legend_and_filter()` |
| FR-8-4 ツールチップ | UI | ui::pinout_panel | src/ui/pinout_panel.rs | `response.on_hover_ui_at_pointer()` |
| FR-8-5 詳細カード | UI | ui::pinout_panel | src/ui/pinout_panel.rs | `ui_pin_detail()`, `IdeApp::pinout_hovered_pin` |
| FR-8-6 対応ボード | Core | core::pinout | src/core/pinout.rs | `ARDUINO_UNO_PINOUT`, `ESP32_PINOUT`, `MICROBIT_PINOUT`, `STM32F4_PINOUT`, `get_pinout()` |

---

## FR-9: ELF ビューア

| 要件 ID | アーキテクチャ層 | モジュール | ソースファイル | 主要シンボル |
|---------|----------------|-----------|--------------|------------|
| FR-9-1 ELF 解析 | Core | core::elf_analyzer | src/core/elf_analyzer.rs | `analyze_elf()`, `object::File::parse()` |
| FR-9-2 セクション一覧 | Core, UI | core::elf_analyzer, ui::elf_panel | src/core/elf_analyzer.rs, src/ui/elf_panel.rs | `ElfInfo::sections: Vec<ElfSection>` |
| FR-9-3 シンボル一覧 | Core, UI | core::elf_analyzer, ui::elf_panel | src/core/elf_analyzer.rs, src/ui/elf_panel.rs | `ElfInfo::symbols: Vec<ElfSymbol>` |
| FR-9-4 Flash/RAM 内訳 | Core, App, UI | core::build_analyzer, ui::build_panel | src/core/build_analyzer.rs, src/ui/build_panel.rs | `BuildStats` |

---

## FR-10: スタック解析

| 要件 ID | アーキテクチャ層 | モジュール | ソースファイル | 主要シンボル |
|---------|----------------|-----------|--------------|------------|
| FR-10-1 スタック推定 | Core | core::stack_analyzer | src/core/stack_analyzer.rs | `analyze_stack()`, `nm` コマンド実行 |
| FR-10-2 フィルタ検索 | App, UI | app, ui::stack_panel | src/app.rs, src/ui/stack_panel.rs | `IdeApp::stack_filter`, `StackReport::frames` |
| FR-10-3 合計表示 | UI | ui::stack_panel | src/ui/stack_panel.rs | `StackReport::total_estimate` |

---

## FR-11: デバッガ連携

| 要件 ID | アーキテクチャ層 | モジュール | ソースファイル | 主要シンボル |
|---------|----------------|-----------|--------------|------------|
| FR-11-1 接続・切断 | Core, App | core::debugger, app | src/core/debugger.rs, src/app.rs | `spawn_debugger()`, `DebugCommand::Connect/Disconnect`, `IdeApp::debug_connected` |
| FR-11-2 実行制御 | Core, UI | core::debugger, ui::debug_panel | src/core/debugger.rs, src/ui/debug_panel.rs | `DebugCommand::{Halt,Continue,Step}`, probe-rs Session |
| FR-11-3 レジスタ表示 | Core, App, UI | core::debugger, ui::debug_panel | src/core/debugger.rs, src/ui/debug_panel.rs | `DebugEvent::Registers`, `IdeApp::debug_registers: Vec<RegisterValue>` |
| FR-11-4 メモリダンプ | Core, App, UI | core::debugger, ui::debug_panel | src/core/debugger.rs, src/ui/debug_panel.rs | `DebugCommand::ReadMemory`, `DebugEvent::MemoryRead`, `IdeApp::debug_memory` |
| FR-11-5 RTT ログ | Core, App, UI | core::debugger, ui::rtt_panel | src/core/debugger.rs, src/ui/rtt_panel.rs | `DebugCommand::StartRtt`, `DebugEvent::RttData`, `IdeApp::rtt_log`, `AppMessage::RttData` |
| FR-11-6 ブレークポイント | App, UI | app, ui::editor | src/app.rs, src/ui/editor.rs | `IdeApp::breakpoints: HashSet<usize>` |

---

## FR-12: SVD レジスタビューア

| 要件 ID | アーキテクチャ層 | モジュール | ソースファイル | 主要シンボル |
|---------|----------------|-----------|--------------|------------|
| FR-12-1 SVD 読み込み | Core | core::svd_parser | src/core/svd_parser.rs | `parse_svd()`, `roxmltree` |
| FR-12-2 階層ツリー表示 | Core, UI | core::svd_parser, ui::svd_panel | src/core/svd_parser.rs, src/ui/svd_panel.rs | `SvdDevice`, `SvdPeripheral`, `SvdRegister`, `ui_svd_panel()` |
| FR-12-3 レジスタ値表示 | App, UI | app, ui::svd_panel | src/app.rs, src/ui/svd_panel.rs | `IdeApp::svd_device`, `IdeApp::svd_search`, `svd_expanded_peripherals` |

---

## FR-13: LSP 連携

| 要件 ID | アーキテクチャ層 | モジュール | ソースファイル | 主要シンボル |
|---------|----------------|-----------|--------------|------------|
| FR-13-1 ra 自動チェック | Core, App | core::toolchain, app | src/core/toolchain.rs, src/app.rs | `check_rust_analyzer()`, `RustAnalyzerStatus`, `IdeApp::ra_status` |
| FR-13-2 LSP 自動起動 | Core, App | core::lsp, app | src/core/lsp.rs, src/app.rs | `LspClient::start()`, `IdeApp::lsp_client`, `lsp_initialized` |
| FR-13-3 補完候補表示 | Core, App, UI | core::lsp, app, ui::editor | src/core/lsp.rs, src/app.rs, src/ui/editor.rs | `CompletionItem`, `IdeApp::lsp_completions`, `show_completion` |
| FR-13-4 診断情報表示 | Core, App, UI | core::lsp, app, ui::editor | src/core/lsp.rs, src/app.rs, src/ui/editor.rs | `Diagnostic`, `IdeApp::lsp_diagnostics` |
| FR-13-5 初期化シーケンス | Core | core::lsp | src/core/lsp.rs | `LspMessage::Initialized`, `pending_did_opens`, initialize → initialized → didOpen 順序 |

---

## FR-14: ツールチェーン管理

| 要件 ID | アーキテクチャ層 | モジュール | ソースファイル | 主要シンボル |
|---------|----------------|-----------|--------------|------------|
| FR-14-1 ra インストール確認 | Core | core::toolchain | src/core/toolchain.rs | `check_rust_analyzer()`, `which` クレート |
| FR-14-2 インストールトリガー | App, UI | app, ui::settings | src/app.rs, src/ui/settings.rs | `IdeApp::ra_installing`, `AppMessage::Toolchain(ToolchainMsg::InstallFinished)` |
| FR-14-3 外部ツール検索 | Core | core::toolchain, core::flasher | src/core/toolchain.rs, src/core/flasher.rs | `which::which("avrdude")` 等 |

---

## FR-15: 設定

| 要件 ID | アーキテクチャ層 | モジュール | ソースファイル | 主要シンボル |
|---------|----------------|-----------|--------------|------------|
| FR-15-1 ワークスペース設定 | App, UI | app, ui::settings | src/app.rs, src/ui/settings.rs | `IdeApp::config.workspace_dir`, `rfd::FileDialog::pick_folder()` |
| FR-15-2 テーマ切替 | App, UI | app, ui::settings | src/app.rs, src/ui/settings.rs | `IdeApp::config.theme`, `egui::Context::set_visuals()` |
| FR-15-3 ボーレート設定 | Core, App | core::config, app | src/core/config.rs, src/app.rs | `AppConfig::last_port`, `DEFAULT_BAUD_RATE` |
| FR-15-4 TOML 永続保存 | Core | core::config | src/core/config.rs | `AppConfig::save()`, `toml::to_string_pretty()` |
| FR-15-5 保存先ディレクトリ | Core | core::config | src/core/config.rs | `dirs::config_dir()`, `AppConfig::path()` |

---

## FR-16: ヘルプ・ドキュメント

| 要件 ID | アーキテクチャ層 | モジュール | ソースファイル | 主要シンボル |
|---------|----------------|-----------|--------------|------------|
| FR-16-1 ドキュメント組み込み | UI | ui::help_panel | src/ui/help_panel.rs | `include_str!()`, `DOCS` 定数配列, `RightTab` |
| FR-16-2 フォントサイズ変更 | App, UI | app | src/app.rs | `IdeApp::doc_font_size` |
| FR-16-3 Markdown HL | UI | ui::help_panel | src/ui/help_panel.rs | `egui_commonmark::CommonMarkViewer`, `IdeApp::doc_cache` |

---

## NFR トレーサビリティ

| 要件 ID | 内容 | 実現場所 | ソースファイル | 主要シンボル |
|---------|------|---------|--------------|------------|
| NFR-1-1 60 FPS 維持 | バックグラウンド処理化 | Core 全体 | src/core/*.rs | `std::thread::spawn()` + `crossbeam_channel` |
| NFR-1-2 ログ 500 行上限 | VecDeque + pop_front | App | src/app.rs | `serial_log: VecDeque<String>`（最大 500 要素） |
| NFR-1-3 UI ブロック禁止 | 非同期スレッド | Core 全体 | src/core/*.rs | `build_async()`, `flash()`, `connect()`, `spawn_debugger()` |
| NFR-2-1 パニック禁止 | anyhow + ? | Core | src/core/*.rs | `anyhow::Result`, `?` 演算子 |
| NFR-2-2 境界チェック | `.get()` 使用 | App, UI | src/app.rs, src/ui/*.rs | `BOARD_PRESETS.get(app.selected_board)` |
| NFR-2-3 スレッドリーク防止 | crossbeam select! | Core | src/core/serial.rs | `crossbeam_channel::select!`（stop_rx 受信で break） |
| NFR-2-4 非 UTF-8 パス | PathBuf 使用 | Core | src/core/flasher.rs | `std::path::PathBuf`（文字列変換なし） |
| NFR-3-1 Win/Linux 対応 | 条件コンパイル | Core | src/core/serial.rs | `#[cfg(windows)]` ドライブレター列挙 |
| NFR-3-2 パス操作 | PathBuf 統一 | Core 全体 | src/core/*.rs | `PathBuf::join()`, `Path::exists()` |
| NFR-4-1 Core の egui 非依存 | レイヤー分離 | Core | src/core/*.rs | `use egui` なし |
| NFR-4-2 ボード追加容易性 | BOARD_PRESETS スライス | Core | src/core/board.rs | `BOARD_PRESETS: &[BoardPreset]` への1エントリ追加 |
| NFR-4-3 メッセージ網羅性 | enum + match | App | src/app.rs | `AppMessage` enum、`handle_messages()` の exhaustive match |
| NFR-4-4 フォーマット/Lint | cargo fmt/clippy | — | — | CI / pre-commit |
| NFR-4-5 SPDX ヘッダー | ファイル先頭 | 全 .rs | src/**/*.rs | `// SPDX-License-Identifier: MIT OR Apache-2.0` |
| NFR-5-1 デュアルライセンス | Cargo.toml | — | Cargo.toml | `license = "MIT OR Apache-2.0"` |
| NFR-5-2 依存ライセンス確認 | 審査 | — | OSS_LICENSE_AUDIT.md | 全クレートの MIT/Apache-2.0/BSD 確認 |
| NFR-5-3 ライセンス記録 | 審査ドキュメント | — | OSS_LICENSE_AUDIT.md | ライセンス審査結果表 |

---

## ソースファイル → 要件 逆引き

| ソースファイル | 実装している主な要件 |
|--------------|-------------------|
| `src/main.rs` | — (エントリーポイントのみ) |
| `src/app.rs` | FR-1〜16 全体（状態管理・メッセージルーティング）, NFR-1-2, NFR-2-2 |
| `src/core/board.rs` | FR-3-1, FR-5-2, NFR-4-2 |
| `src/core/compiler.rs` | FR-4-1〜4-5, FR-5-5, NFR-1-3 |
| `src/core/flasher.rs` | FR-5-1〜5-6, NFR-2-4 |
| `src/core/serial.rs` | FR-6-1〜6-6, FR-3-2, NFR-2-3, NFR-3-1 |
| `src/core/lsp.rs` | FR-13-2〜13-5 |
| `src/core/debugger.rs` | FR-11-1〜11-5 |
| `src/core/detector.rs` | FR-3-3, FR-3-4 |
| `src/core/config.rs` | FR-2-3, FR-15-3〜15-5 |
| `src/core/build_analyzer.rs` | FR-4-5, FR-4-6, FR-9-4 |
| `src/core/elf_analyzer.rs` | FR-9-1〜9-4 |
| `src/core/stack_analyzer.rs` | FR-10-1〜10-3 |
| `src/core/svd_parser.rs` | FR-12-1〜12-2 |
| `src/core/pinout.rs` | FR-8-1, FR-8-3, FR-8-6 |
| `src/core/toolchain.rs` | FR-14-1〜14-3 |
| `src/core/project.rs` | FR-2-2 |
| `src/core/snippets.rs` | FR-1-9 |
| `src/templates/mod.rs` | FR-2-1 |
| `src/ui/editor.rs` | FR-1-1〜1-10, FR-13-3, FR-13-4 |
| `src/ui/build_panel.rs` | FR-4-2〜4-6, FR-5-3 |
| `src/ui/board_picker.rs` | FR-3-1〜3-5 |
| `src/ui/serial_monitor.rs` | FR-6-1〜6-6 |
| `src/ui/serial_plotter.rs` | FR-7-1〜7-4 |
| `src/ui/debug_panel.rs` | FR-11-1〜11-5 |
| `src/ui/rtt_panel.rs` | FR-11-5 |
| `src/ui/svd_panel.rs` | FR-12-2〜12-3 |
| `src/ui/elf_panel.rs` | FR-9-2〜9-3 |
| `src/ui/stack_panel.rs` | FR-10-2〜10-3 |
| `src/ui/pinout_panel.rs` | FR-8-1〜8-5 |
| `src/ui/help_panel.rs` | FR-16-1〜16-3 |
| `src/ui/settings.rs` | FR-15-1〜15-2, FR-14-2 |
| `src/ui/file_explorer.rs` | FR-1-4 |
| `src/ui/fonts.rs` | — (日本語フォント設定) |

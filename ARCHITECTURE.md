# アーキテクチャ設計書 — ALLoIDE

> バージョン: 0.1  
> 作成日: 2026-04-19  
> ライセンス: MIT OR Apache-2.0

---

## 1. システム概要

ALLoIDE は **3 層アーキテクチャ**で構成されます。

```
┌───────────────────────────────────────────┐
│                  UI 層                     │  src/ui/**
│  egui パネル描画。重い処理は Core に委譲    │
├───────────────────────────────────────────┤
│                 App 層                     │  src/app/**
│  IdeApp 状態管理・メッセージルーティング    │
├───────────────────────────────────────────┤
│                Core 層                     │  src/core/**
│  I/O・ビルド・フラッシュ・デバッグロジック  │  （egui に非依存）
└───────────────────────────────────────────┘
```

| 層 | 責務 | egui 依存 |
|----|------|----------|
| Core | バックエンド処理・外部ツール呼び出し・スレッド管理 | **なし** |
| App | 状態保持・チャンネル受信・パネルへのルーティング | あり（Color32 等） |
| UI | 即時描画モード GUI（入力検知・描画） | あり |

---

## 2. コンポーネント図

```
src/main.rs
  └── eframe::run_native(IdeApp)
        │
        ▼
src/app/mod.rs ── IdeApp
  ├── Sender<CoreEvent>   ─── バックグラウンドスレッドが送信
  ├── Receiver<CoreEvent> ─── 毎フレーム try_recv() でポーリング
  │
  ├── [Core 呼び出し]
  │    ├── core::build::compiler   build_async()
  │    ├── core::build::flasher    flash()
  │    ├── core::serial     connect() / list_ports()
  │    ├── core::editor::lsp        LspClient::start()
  │    ├── core::inspect::debugger  spawn_debugger()
  │    ├── core::board::detector    detect_by_usb_id() / detect_by_probe_rs()
  │    ├── app::config              AppConfig::load() / save()
  │    ├── core::inspect::elf       analyze_elf()
  │    ├── core::inspect::stack     analyze_stack()
  │    ├── core::inspect::svd       parse_svd()
  │    ├── core::build::toolchain   check_rust_analyzer()
  │    ├── core::editor::project    open_project()
  │    └── core::editor::snippets   get_snippets()
  │
  └── [UI 呼び出し]
       ├── ui::editor          ui_editor()
       ├── ui::build_panel     ui_build_panel()
       ├── ui::board_picker    ui_board_picker()
       ├── ui::serial_monitor  ui_serial_monitor()
       ├── ui::serial_plotter  ui_serial_plotter()
       ├── ui::debug_panel     ui_debug_panel()
       ├── ui::rtt_panel       ui_rtt_panel()
       ├── ui::svd_panel       ui_svd_panel()
       ├── ui::elf_panel       ui_elf_panel()
       ├── ui::stack_panel     ui_stack_panel()
       ├── ui::pinout_panel    ui_pinout_panel()
       ├── ui::help_panel      ui_help_panel()
       ├── ui::settings        ui_settings()
       └── ui::file_explorer   ui_file_explorer()
```

---

## 3. 非同期メッセージパッシングパターン

すべての重い処理（ビルド・書き込み・シリアル通信・LSP・デバッグ）は
バックグラウンドスレッドで実行し、`crossbeam_channel` を通じて
`CoreEvent` として UI スレッドに結果を送信します。

```
[UI スレッド]                    [バックグラウンドスレッド]
    │                                    │
    │  Sender<CoreEvent> をクローンして渡す
    │────────────────────────────────────►│
    │                                    │  重い処理（I/O・外部コマンド）
    │                                    │
    │◄───────────────────────────────────│
    │  tx.send(CoreEvent::XXX(...))
    │
    │  毎フレーム: while let Ok(msg) = rx.try_recv() { ... }
    │  IdeApp フィールドを更新 → 次フレームの描画に反映
```

### CoreEvent バリアント一覧

```rust
pub enum CoreEvent {
    Build(BuildMsg),                    // ビルド進捗・結果
    Flash(FlashMsg),                    // 書き込み進捗・結果
    Serial(SerialMsg),                  // シリアル受信・接続状態
    Toolchain(ToolchainMsg),            // ツールチェーン操作結果
    BoardDetected(Option<DetectedBoard>), // 自動検出結果
    LspCompletion(Vec<CompletionItem>), // 補完候補
    LspDiagnostic(Vec<Diagnostic>),     // 診断情報
    LspInitialized,                     // LSP 初期化完了
    BuildAnalysis(BuildStats),          // ELF サイズ解析結果
    RttData { channel: u32, data: String }, // RTT データ
    ElfAnalysis(ElfInfo),               // ELF 解析結果
    StackAnalysis(StackReport),         // スタック解析結果
    Error(String),                      // 汎用エラー
}
```

---

## 4. 主要データフロー

### 4.1 ビルドフロー

```
[ui::build_panel]
  ↓ Build ボタン押下
[app/mod.rs]
  ↓ core::build::compiler::build_async(BuildRequest, msg_tx.clone())
[core::build::compiler — バックグラウンドスレッド]
  ↓ cargo build --release --target <triple>
  ↓ 成功時: ELF を dist/ にコピー
  ↓ tx.send(CoreEvent::Build(BuildMsg::Finished(BuildResult)))
[app/events.rs handle_messages()]
  ↓ is_building = false / build_log 更新 / last_dist_path 設定
  ↓ (auto_flash_after_build が true なら flash_async を起動)
[ui::build_panel]
  → ビルドログ・サイズメーター表示
```

### 4.2 書き込みフロー

```
[ui::build_panel]
  ↓ Flash ボタン押下
[app/mod.rs]
  ↓ core::build::flasher::flash(preset, port, elf_path, flash_tx)
[core::build::flasher — バックグラウンドスレッド]
  ↓ FlashToolKind に応じて外部コマンド実行:
    Avrdude   → avrdude -c arduino -p <mcu> -P <port> -U flash:w:<hex>:i
    Esptool   → esptool.py --port <port> write_flash <offset> <bin>
    ProbeRs   → probe-rs download <elf> --chip <chip>
    Picotool  → ELF→UF2 変換 → UF2 ファイル書き込み
    Bossac    → bossac -p <port> -e -w -v <bin>
    DaplinkHex→ ELF→HEX 変換 → ドライブにコピー
    SdCard    → ELF→kernel.img 変換 → ドライブにコピー
    (他: OpenOcd / StFlash / NrfJprog / TeensyLoader)
  ↓ tx.send(CoreEvent::Flash(FlashMsg::Finished(FlashResult)))
[app/mod.rs]
  → is_flashing = false / build_log 更新
```

### 4.3 シリアルフロー

```
[ui::serial_monitor]
  ↓ 接続ボタン押下
[app/mod.rs]
  ↓ core::serial::connect(port, baud, serial_event_tx) → SerialHandle
  ↓ SerialHandle { write_tx, stop_tx } を serial_tx に保存

[core::serial — 読み込みスレッド]
  BufReader::read_line() ループ
  → SerialEvent::Data(line) → CoreEvent::Serial(SerialMsg::Line)
  → SerialEvent::Error     → CoreEvent::Serial(SerialMsg::Error)
  → SerialEvent::Closed    → CoreEvent::Serial(SerialMsg::Disconnected)

[core::serial — 書き込みスレッド]
  crossbeam select! { write_rx → write, stop_rx → break }

[ui::serial_monitor]
  ↓ 送信ボタン / Enter キー
[app/mod.rs]
  ↓ serial_tx.as_ref().map(|h| h.write_tx.send(text))
```

### 4.4 LSP フロー

```
[app/mod.rs（プロジェクト読み込み時）]
  ↓ core::editor::lsp::LspClient::start(workspace, ra_path, ui_tx)
[core::editor::lsp — LSP 送受信スレッド]
  1. rust-analyzer プロセス起動
  2. initialize リクエスト送信
  3. レスポンス受信 → LspMessage::Initialized → ui_tx.send()
  4. (app/events.rs で initialized を受信) → initialized 通知 + didOpen 送信
  5. textDocument/completion リクエスト → LspMessage::CompletionItems
  6. textDocument/publishDiagnostics 通知 → LspMessage::Diagnostics

[app/events.rs handle_messages()]
  LspInitialized → lsp_initialized = true、pending_did_opens を送信
  LspCompletion  → lsp_completions 更新
  LspDiagnostic  → lsp_diagnostics 更新
```

### 4.5 デバッグフロー

```
[app/mod.rs（起動時）]
  ↓ core::inspect::debugger::spawn_debugger() → (debug_cmd_tx, debug_evt_rx)

[ui::debug_panel]
  ↓ 接続ボタン押下
  ↓ debug_cmd_tx.send(DebugCommand::Connect { chip })

[core::inspect::debugger — デバッグスレッド]
  probe_rs::Permissions::new() → Session 確立
  DebugCommand::Halt       → core.halt()
  DebugCommand::Continue   → core.run()
  DebugCommand::Step       → core.step()
  DebugCommand::ReadRegisters → core.read_core_reg()
  DebugCommand::ReadMemory → session.core().read_8()
  DebugCommand::StartRtt   → RTT ワーカースレッド起動
  → evt_tx.send(DebugEvent::XXX)

[app/events.rs（毎フレーム）]
  debug_evt_rx.try_recv() → debug_registers / debug_memory 更新
```

---

## 5. ウィンドウレイアウト

```
┌──────────────────────────────────────────────────────────┐
│ MenuBar: File | Build | Tools | Debug | Help              │
├──────────────┬──────────────────────┬────────────────────┤
│ 左パネル      │                      │ 右パネル            │
│ (250px)      │     中央エリア         │ (300px)            │
│              │                      │                    │
│ BoardPicker  │  FileExplorer (上部)  │ RightTab:          │
│ BuildPanel   │  ─────────────────── │  ・Pinout           │
│              │  Editor (複数タブ)    │  ・SerialMonitor    │
│              │                      │  ・SerialPlotter    │
│              │                      │  ・Help/Docs        │
├──────────────┴──────────────────────┴────────────────────┤
│ StatusBar: カーソル位置 | ボード名 | LSP 状態 | 接続状態    │
└──────────────────────────────────────────────────────────┘

モーダルウィンドウ（show_xxx フラグで制御）:
  ・ELF Viewer        (show_elf_panel)
  ・Stack Analyzer    (show_stack_panel)
  ・Debug Panel       (show_debug_panel)
  ・SVD Viewer        (show_svd_panel)
  ・Settings          (show_settings)
  ・Help              (show_help_window)
  ・New Project       (show_new_project_dialog)
  ・New File          (show_new_file_dialog)
```

---

## 6. ボードプリセットシステム

```
BoardPreset {
    kind:            BoardKind (enum)       // 識別子
    display_name:    &'static str           // コンボボックス表示名
    cpu_arch:        CpuArch (enum)         // CPU アーキテクチャ
    target_triple:   &'static str           // cargo --target に渡す
    avrdude_mcu:     Option<&'static str>   // avrdude -p 引数
    flash_tool:      FlashToolKind (enum)   // 書き込みツール選択
    default_baud:    u32                    // デフォルトボーレート
    usb_ids:         &'static [UsbId]       // 自動検出用 VID/PID テーブル
    probe_rs_chip:   &'static str           // probe-rs --chip 引数
    rustflags:       &'static [&'static str]// 追加 RUSTFLAGS
    flash_offset:    u32                    // ESP32 書き込みオフセット
    memory_layout:   Option<MemoryLayout>   // memory.x 自動生成用
    toolchain_note:  Option<&'static str>   // ツールチェーン注記
}
```

新ボード追加時は `BOARD_PRESETS` スライスに1エントリ追加するだけで、
コンパイラ・書き込み・検出・ピンアウト以外の全機能が自動対応します。

---

## 7. 設定管理

```
AppConfig (Serialize/Deserialize)
  ├── last_board: Option<String>       // 前回選択ボード名
  ├── last_port:  Option<String>       // 前回選択ポート
  ├── workspace_dir: PathBuf           // ワークスペースディレクトリ
  ├── theme: Option<String>            // "dark" / "light"
  └── rust_analyzer_path: Option<PathBuf> // ra カスタムパス

保存先 (dirs クレート):
  Windows: %APPDATA%\alloide\config.toml
  Linux:   ~/.config/alloide/config.toml
```

---

## 8. モジュール依存関係

```
app/mod.rs
  ├─► core::board        (BoardPreset, BOARD_PRESETS, BoardKind)
  ├─► core::build::compiler (build_async, BuildRequest, BuildResult)
  ├─► core::build::flasher  (flash, FlashMessage)
  ├─► core::serial       (connect, list_ports, SerialHandle, SerialEvent)
  ├─► core::editor::lsp  (LspClient, LspMessage, CompletionItem, Diagnostic)
  ├─► core::inspect::debugger (spawn_debugger, DebugCommand, DebugEvent, RegisterValue)
  ├─► core::board::detector   (detect_by_usb_id, detect_by_probe_rs, DetectedBoard)
  ├─► app::config        (AppConfig)
  ├─► core::build::analyzer (BuildStats)
  ├─► core::inspect::elf   (ElfInfo)
  ├─► core::inspect::stack (StackReport)
  ├─► core::inspect::svd   (SvdDevice)
  ├─► core::build::toolchain (check_rust_analyzer, RustAnalyzerStatus)
  ├─► core::editor::project  (open_project)
  ├─► core::editor::snippets (get_snippets)
  └─► ui::*              (各 ui_xxx_panel 関数)

ui::pinout_panel
  └─► core::board::pinout (get_pinout, BoardPinout, PinInfo, PinFunction)

ui::build_panel
  └─► core::build::analyzer (BuildStats)

ui::debug_panel
  └─► core::inspect::debugger (DebugCommand, DebugEvent, RegisterValue)

ui::svd_panel
  └─► core::inspect::svd (SvdDevice, SvdPeripheral, SvdRegister)

ui::elf_panel
  └─► core::inspect::elf (ElfInfo, ElfSection, ElfSymbol)

ui::stack_panel
  └─► core::inspect::stack (StackReport, StackFrame)
```

---

## 9. 主要な型定義

### IdeApp（src/app/mod.rs）

`IdeApp` は `eframe::App` を実装するアプリ全体の状態コンテナです。
フィールド命名規則：

| プレフィックス | 意味 |
|--------------|------|
| `is_` | 現在の動作状態 (bool) |
| `show_` | パネル表示フラグ (bool) |
| `selected_` | 選択インデックス (usize) |
| `last_` | 最後の結果キャッシュ |
| `_log` | ログ文字列 |
| `_filter` | 検索フィルタ |

### 主要リクエスト/レスポンス型

| リクエスト | レスポンス | 用途 |
|-----------|-----------|------|
| `BuildRequest` | `BuildResult` | cargo build |
| `FlashRequest`（preset + port + elf_path）| `FlashMessage` | 書き込み |
| `DebugCommand` | `DebugEvent` | デバッガ操作 |
| LSP JSON-RPC | `LspMessage` | rust-analyzer 通信 |

---

## 10. 外部プロセス起動パターン

```rust
// 共通パターン：std::process::Command + バックグラウンドスレッド
std::thread::spawn(move || {
    let output = Command::new("cargo")
        .current_dir(&project_dir)
        .args(&["build", "--release", "--target", &triple])
        .output()
        .expect("cargo not found");
    let _ = tx.send(CoreEvent::Build(BuildMsg::Finished(BuildResult {
        success: output.status.success(),
        stdout: String::from_utf8_lossy(&output.stdout).into(),
        stderr: String::from_utf8_lossy(&output.stderr).into(),
        ..
    })));
});
```

ストリーミングが必要なケース（avrdude など）は
`Command::stdout(Stdio::piped())` + `BufReader::lines()` でリアルタイム出力を処理します。

---

## 11. 自動検出シーケンス

```
detect_by_usb_id()                   (Stage 1)
  └── serialport::available_ports()
      → UsbPort { vid, pid }
      → BOARD_PRESETS[].usb_ids 照合
      → DetectionConfidence::High / Medium

detect_by_probe_rs()                  (Stage 2, ARM/RISC-V のみ)
  └── probe_rs::probe::list::Lister::list_all()
      → チップ名文字列
      → BOARD_PRESETS[].probe_rs_chip 照合
      → DetectionConfidence::Exact
```

---

## 12. テンプレート生成フロー

```
[ui::board_picker — テンプレート読み込みボタン]
  ↓ template_confirm_board = Some(index)
[ui/workbench.rs — 確認ダイアログ OK]
  ↓ core::templates::generate(board_kind)
  → .cargo/config.toml（target triple 設定）
  → Cargo.toml（依存クレート）
  → src/main.rs（ボード別ブリンクサンプル）
  → memory.x（Cortex-M / RISC-V ボードのみ）
  → build.rs（memory.x リンカ設定）
```

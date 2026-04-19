// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright 2026 rust-embedded-ide contributors

use crossbeam_channel::{unbounded, Receiver, Sender};

use std::path::PathBuf;
use std::collections::VecDeque;

pub fn write_or_log(path: &std::path::Path, content: &str, log: &mut String) {
    if let Err(e) = std::fs::write(path, content) {
        *log = format!("[ERROR] ファイル保存失敗 {}: {}", path.display(), e);
    }
}


/// 開いているファイルタブ1つ分の状態
#[derive(Clone)]
pub struct FileTab {
    pub path: PathBuf,
    pub content: String,
    pub is_dirty: bool,
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct PlotChannel {
    pub name: String,
    pub color: egui::Color32,
    pub values: std::collections::VecDeque<f64>,
}

pub fn parse_plot_line(line: &str) -> Vec<(String, f64)> {
    let line = line.trim();
    let mut result = Vec::new();
    for (i, part) in line.split(',').enumerate() {
        let part = part.trim();
        if let Some(colon) = part.find(':') {
            let name = part[..colon].trim().to_string();
            let val_str = part[colon+1..].trim();
            if let Ok(v) = val_str.parse::<f64>() {
                result.push((name, v));
            }
        } else if let Ok(v) = part.parse::<f64>() {
            result.push((format!("ch{}", i), v));
        }
    }
    result
}

// App messages from background tasks
#[allow(dead_code)]
pub enum BuildMsg { Started, Progress(String), Finished(crate::core::compiler::BuildResult) }
#[allow(dead_code)]
pub enum FlashMsg { Started, Progress(String), Finished(crate::core::flasher::FlashResult) }
#[allow(dead_code)]
pub enum SerialMsg { Line(String), Error(String), Connected, Disconnected }
#[allow(dead_code)]
pub enum ToolchainMsg {
    InstallStarted,
    InstallFinished(Result<crate::core::toolchain::RustAnalyzerStatus, String>),
}

#[allow(dead_code)]
pub enum AppMessage {
    Build(BuildMsg),
    Flash(FlashMsg),
    Serial(SerialMsg),
    Toolchain(ToolchainMsg),
    BoardDetected(Option<crate::core::detector::DetectedBoard>),
    LspCompletion(Vec<crate::core::lsp::CompletionItem>),
    LspDiagnostic(Vec<crate::core::lsp::Diagnostic>),
    LspInitialized,
    BuildAnalysis(crate::core::build_analyzer::BuildStats),
    RttData { channel: u32, data: String },
    ElfAnalysis(crate::core::elf_analyzer::ElfInfo),
    StackAnalysis(crate::core::stack_analyzer::StackReport),
    Error(String),
}

pub struct IdeApp {
    // エディタ
    pub editor_text: String,
    pub file_path: Option<PathBuf>,
    pub is_dirty: bool,

    // ボード・ポート選択
    pub selected_board: usize,      // BOARD_PRESETS のインデックス
    pub available_ports: Vec<String>,
    pub selected_port: usize,

    // ビルド状態
    pub build_log: String,
    pub is_building: bool,
    pub is_flashing: bool,
    pub auto_flash_after_build: bool,
    /// 最後にビルド成功した成果物の dist フォルダ
    pub last_dist_path: Option<PathBuf>,

    // Build analysis / editor helpers
    pub build_stats: Option<crate::core::build_analyzer::BuildStats>,
    pub breakpoints: std::collections::HashSet<usize>,

    // シリアル
    pub serial_log: VecDeque<String>,
    pub serial_input: String,
    pub is_serial_connected: bool,
    pub serial_tx: Option<crossbeam_channel::Sender<crate::core::serial::SerialCommand>>,

    // RTT
    pub rtt_log: Vec<(u32, String)>,         // (channel, message)
    pub rtt_running: bool,
    pub rtt_channel: u32,

    // ELF / Stack analysis
    pub elf_info: Option<crate::core::elf_analyzer::ElfInfo>,
    pub stack_report: Option<crate::core::stack_analyzer::StackReport>,
    pub stack_filter: String,

    // Pinout UI state
    pub pinout_hovered_pin: Option<u8>,
    #[allow(dead_code)]
    pub show_pinout: bool,
    /// 0=All 1=GPIO 2=UART 3=SPI 4=I2C 5=PWM 6=ADC 7=Power 8=GND
    pub pinout_filter: u8,
    /// false=Diagram  true=Table
    pub pinout_view_table: bool,
    pub show_elf_panel: bool,
    pub show_stack_panel: bool,

    // カーソル位置（1始まり）
    pub cursor_line: usize,
    pub cursor_col: usize,
    pub cursor_char_idx: usize,
    pub cursor_screen_pos: Option<egui::Pos2>,

    // チャンネル
    pub msg_tx: Sender<AppMessage>,
    pub msg_rx: Receiver<AppMessage>,

    // 設定
    pub config: crate::core::config::AppConfig,
    pub show_settings: bool,
    pub show_help_window: bool,
    pub snippet_query: String,

    // 検出結果表示
    pub detection_result: Option<String>,

    // テンプレート読み込み確認ダイアログ
    // Some(board_index) = 確認待ち中, None = 非表示
    pub template_confirm_board: Option<usize>,

    // Project / new project dialog
    pub show_new_project_dialog: bool,
    pub new_project_name: String,
    pub new_project_base_dir: std::path::PathBuf,
    pub project_name: Option<String>,
    // マルチファイル
    pub open_tabs: Vec<FileTab>,
    pub active_tab: usize,
    pub workspace_files: Vec<PathBuf>,
    pub show_new_file_dialog: bool,
    pub new_file_name: String,

    // LSP
    pub lsp_client: Option<crate::core::lsp::LspClient>,
    pub lsp_completions: Vec<crate::core::lsp::CompletionItem>,
    pub lsp_diagnostics: Vec<crate::core::lsp::Diagnostic>,
    pub show_completion: bool,
    pub lsp_initialized: bool,
    pub pending_did_opens: Vec<(String, String)>, // (uri, content) buffered until initialized
    pub completion_selected: usize,
    pub doc_version: i32,

    // rust-analyzer / LSP toolchain status
    pub ra_status: crate::core::toolchain::RustAnalyzerStatus,
    pub ra_installing: bool,
    pub ra_install_log: String,

    // 右パネル タブ・ドキュメント選択
    pub right_tab: crate::ui::help_panel::RightTab,
    pub selected_doc: usize,
    pub doc_font_size: f32,
    pub doc_cache: egui_commonmark::CommonMarkCache,

    // Debug panel state
    pub debug_cmd_tx: Option<crossbeam_channel::Sender<crate::core::debugger::DebugCommand>>,
    pub debug_evt_rx: Option<crossbeam_channel::Receiver<crate::core::debugger::DebugEvent>>,
    pub debug_connected: bool,
    pub debug_halted: bool,
    pub debug_registers: Vec<crate::core::debugger::RegisterValue>,
    pub debug_chip_name: String,
    pub debug_error: String,
    pub debug_watch_addr: String,
    pub debug_memory: Vec<u8>,
    pub debug_memory_addr: u64,
    pub show_debug_panel: bool,

    // Serial plotter state
    pub plot_channels: std::collections::HashMap<String, PlotChannel>,
    pub plot_max_points: usize,
    pub plot_paused: bool,
    pub show_plotter_tab: bool,

    // SVD viewer state
    pub svd_device: Option<crate::core::svd_parser::SvdDevice>,
    pub svd_search: String,
    pub svd_expanded_peripherals: std::collections::HashSet<String>,
    pub show_svd_panel: bool,
}

impl IdeApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Load config (may be implemented in core)
        let config = crate::core::config::AppConfig::load().unwrap_or_default();

        let (tx, rx) = unbounded();

        // initial ports
        let ports = crate::core::serial::list_ports().unwrap_or_default();

        // attempt to find preset index from config.last_board
        let mut selected_board = 0usize;
        for (i, p) in crate::core::board::BOARD_PRESETS.iter().enumerate() {
            if config.last_board.as_deref() == Some(p.display_name) {
                selected_board = i;
                break;
            }
        }

        // 日本語フォントを含むシステムフォントをインストール
        crate::ui::fonts::install_japanese_fonts(&cc.egui_ctx);

        let new_project_base_dir = config.workspace_dir.parent()
            .unwrap_or(&config.workspace_dir)
            .to_path_buf();

        let (debug_cmd_tx, debug_evt_rx) = crate::core::debugger::spawn_debugger();

        let mut app = Self {
            editor_text: String::new(),
            file_path: None,
            is_dirty: false,
            selected_board,
            available_ports: ports,
            selected_port: 0,
            build_log: String::new(),
            is_building: false,
            is_flashing: false,
            auto_flash_after_build: false,
            last_dist_path: None,
            build_stats: None,
            breakpoints: std::collections::HashSet::new(),
            serial_log: VecDeque::new(),
            serial_input: String::new(),
            is_serial_connected: false,
            serial_tx: None,
            rtt_log: Vec::new(),
            rtt_running: false,
            rtt_channel: 0,
            elf_info: None,
            stack_report: None,
            stack_filter: String::new(),
            pinout_hovered_pin: None,
            show_pinout: false,
            pinout_filter: 0,
            pinout_view_table: false,
            show_elf_panel: false,
            show_stack_panel: false,
            cursor_line: 1,
            cursor_col: 1,
            cursor_char_idx: 0,
            cursor_screen_pos: None,
            msg_tx: tx,
            msg_rx: rx,
            config: config.clone(),
            show_new_project_dialog: false,
            new_project_name: String::new(),
            new_project_base_dir,
            project_name: None,
            open_tabs: Vec::new(),
            active_tab: 0,
            workspace_files: Vec::new(),
            show_new_file_dialog: false,
            new_file_name: String::new(),
            show_settings: false,
            show_help_window: false,
            snippet_query: String::new(),
            detection_result: None,
            template_confirm_board: None,
            lsp_client: None,
            lsp_completions: Vec::new(),
            lsp_diagnostics: Vec::new(),
            show_completion: false,
            lsp_initialized: false,
            pending_did_opens: Vec::new(),
            completion_selected: 0,
            doc_version: 1,
            ra_status: crate::core::toolchain::check_rust_analyzer(),
            ra_installing: false,
            ra_install_log: String::new(),
            right_tab: crate::ui::help_panel::RightTab::default(),
            selected_doc: 0,
            doc_font_size: 14.0,
            doc_cache: egui_commonmark::CommonMarkCache::default(),

            // Debug panel state
            debug_cmd_tx: Some(debug_cmd_tx),
            debug_evt_rx: Some(debug_evt_rx),
            debug_connected: false,
            debug_halted: false,
            debug_registers: Vec::new(),
            debug_chip_name: String::from("nRF52833_xxAA"),
            debug_error: String::new(),
            debug_watch_addr: String::from("0x20000000"),
            debug_memory: Vec::new(),
            debug_memory_addr: 0,
            show_debug_panel: true,

            // Serial plotter defaults
            plot_channels: std::collections::HashMap::new(),
            plot_max_points: 200,
            plot_paused: false,
            show_plotter_tab: false,

            // SVD viewer defaults
            svd_device: None,
            svd_search: String::new(),
            svd_expanded_peripherals: std::collections::HashSet::new(),
            show_svd_panel: false,
        };

        app.refresh_workspace_files();

        app.sanitize_selected_board();

        // LSP 起動（rust-analyzerがあれば）。workspace_dir を使用
        let ws_dir = app.config.workspace_dir.clone();
        let ws = if ws_dir.exists() { Some(ws_dir) } else { std::env::current_dir().ok() };
        if let Some(ws) = ws {
            let (lsp_tx, lsp_rx) = unbounded::<crate::core::lsp::LspMessage>();
            let ra_path = app.config.rust_analyzer_path.clone();
            if let Some(client) = crate::core::lsp::start_lsp(ws.clone(), lsp_tx, ra_path) {
                // 起動時はファイル未ロードなので did_open は open_file_in_tab で行う
                app.lsp_client = Some(client);

                let app_msg_tx = app.msg_tx.clone();
                std::thread::spawn(move || {
                    while let Ok(msg) = lsp_rx.recv() {
                        match msg {
                            crate::core::lsp::LspMessage::CompletionItems(items) => {
                                let _ = app_msg_tx.send(AppMessage::LspCompletion(items));
                            }
                            crate::core::lsp::LspMessage::Diagnostics(diags) => {
                                let _ = app_msg_tx.send(AppMessage::LspDiagnostic(diags));
                            }
                            crate::core::lsp::LspMessage::Error(e) => {
                                let _ = app_msg_tx.send(AppMessage::Error(e));
                            }
                            crate::core::lsp::LspMessage::Initialized => {
                                let _ = app_msg_tx.send(AppMessage::LspInitialized);
                            }
                        }
                    }
                });
            }
        }

        app
    }

    fn handle_messages(&mut self) {
        while let Ok(msg) = self.msg_rx.try_recv() {
            match msg {
                AppMessage::Build(BuildMsg::Started) => {
                    self.is_building = true;
                    self.build_log = "[BUILD] ビルド開始...\n".to_string();
                }
                AppMessage::Build(BuildMsg::Progress(text)) => {
                    self.build_log.push_str(&text);
                    self.build_log.push('\n');
                }
                AppMessage::Flash(FlashMsg::Started) => {
                    self.is_flashing = true;
                    self.build_log.push_str("\n[FLASH] フラッシュ開始...\n");
                }
                AppMessage::Flash(FlashMsg::Progress(text)) => {
                    self.build_log.push_str(&format!("{}\n", text));
                }
                AppMessage::Build(BuildMsg::Finished(br)) => {
                    self.is_building = false;
                    self.build_log = format!("stdout:\n{}\nstderr:\n{}", br.stdout, br.stderr);
                    if br.success {
                        self.build_log = format!("[SUCCESS]\n{}", self.build_log);
                        if let Some(p) = br.dist_path {
                            self.last_dist_path = Some(p.clone());
                            // Spawn analysis of ELF in background
                            if let Some(board) = crate::core::board::BOARD_PRESETS.get(self.selected_board) {
                                let board_clone = board.clone();
                                let tx = self.msg_tx.clone();
                                let target = p.clone();
                                std::thread::spawn(move || {
                                    if let Some(elf) = crate::core::build_analyzer::find_elf(&target, &board_clone) {
                                        if let Ok(stats) = crate::core::build_analyzer::analyze_elf(&elf, &board_clone) {
                                            let _ = tx.send(crate::app::AppMessage::BuildAnalysis(stats));
                                        }
                                    }
                                });
                            }
                            // Build & Flash: auto-trigger flash after successful build
                            if self.auto_flash_after_build {
                                self.auto_flash_after_build = false;
                                if let Some(preset) = crate::core::board::BOARD_PRESETS.get(self.selected_board) {
                                    // Find ELF artifact for flashing
                                    let elf_path = std::fs::read_dir(&p).ok().and_then(|mut rd| {
                                        rd.find_map(|e| {
                                            let path = e.ok()?.path();
                                            if path.extension().map(|x| x == "elf").unwrap_or(false) {
                                                Some(path)
                                            } else {
                                                None
                                            }
                                        })
                                    });
                                    if let Some(artifact) = elf_path {
                                        let port = self.available_ports.get(self.selected_port).cloned().unwrap_or_default();
                                        let flash_req = crate::core::flasher::FlashRequest {
                                            board: preset.kind.clone(),
                                            artifact,
                                            port,
                                        };
                                        self.is_flashing = true;
                                        self.build_log = format!("[SUCCESS + FLASH開始]\n{}", self.build_log);
                                        crate::core::flasher::flash_async(flash_req, self.msg_tx.clone());
                                    } else {
                                        self.build_log = format!("[SUCCESS]\n[ERROR] フラッシュ用ELFが見つかりません\n{}", self.build_log);
                                    }
                                }
                            }
                        }
                    } else {
                        self.auto_flash_after_build = false;
                        self.build_log = format!("[FAIL]\n{}", self.build_log);
                    }
                }
                AppMessage::Flash(FlashMsg::Finished(fr)) => {
                    self.is_flashing = false;
                    self.build_log = format!("[FLASH]\n{}", fr.output);
                }
                AppMessage::Serial(SerialMsg::Line(line)) => {
                    // Try to parse numeric channels for plotter
                    if !self.plot_paused {
                        for (name, value) in parse_plot_line(&line) {
                            let max_pts = self.plot_max_points;
                            let channel_count = self.plot_channels.len();
                            let ch = self.plot_channels.entry(name.clone()).or_insert_with(|| {
                                let colors = [
                                    egui::Color32::from_rgb(255, 100, 100),
                                    egui::Color32::from_rgb(100, 200, 100),
                                    egui::Color32::from_rgb(100, 150, 255),
                                    egui::Color32::from_rgb(255, 200, 50),
                                    egui::Color32::from_rgb(200, 100, 255),
                                ];
                                PlotChannel {
                                    name: name.clone(),
                                    color: colors[channel_count % colors.len()],
                                    values: std::collections::VecDeque::with_capacity(max_pts),
                                }
                            });
                            ch.values.push_back(value);
                            while ch.values.len() > self.plot_max_points {
                                ch.values.pop_front();
                            }
                        }
                    }

                    self.serial_log.push_back(line);
                    while self.serial_log.len() > 500 {
                        self.serial_log.pop_front();
                    }
                }
                AppMessage::Serial(SerialMsg::Connected) => {
                    self.is_serial_connected = true;
                }
                AppMessage::Serial(SerialMsg::Disconnected) => {
                    self.is_serial_connected = false;
                    self.serial_tx = None;
                }
                AppMessage::BoardDetected(Some(detected)) => {
                    self.selected_board = detected.board_index;
                    // ポートも自動選択
                    if let Some(pos) = self.available_ports.iter().position(|p| *p == detected.port_name) {
                        self.selected_port = pos;
                    }
                    self.detection_result = Some(detected.description.clone());
                }
                AppMessage::BoardDetected(None) => {
                    self.detection_result = Some("No board detected. Please connect a board.".to_string());
                }
                AppMessage::LspCompletion(items) if !items.is_empty() => {
                    self.lsp_completions = items;
                    self.show_completion = true;
                    self.completion_selected = 0;
                }
                AppMessage::LspDiagnostic(diags) => {
                    self.lsp_diagnostics = diags;
                }
                AppMessage::LspInitialized => {
                    self.lsp_initialized = true;
                    // Flush buffered did_open calls that were waiting for initialization
                    if let Some(ref lsp) = self.lsp_client {
                        let pending = std::mem::take(&mut self.pending_did_opens);
                        for (uri, content) in pending {
                            lsp.did_open(&uri, &content);
                        }
                    }
                }
                AppMessage::Toolchain(crate::app::ToolchainMsg::InstallStarted) => {
                    self.ra_installing = true;
                    self.ra_install_log = "Installing rust-analyzer via rustup...".to_string();
                }
                AppMessage::Toolchain(crate::app::ToolchainMsg::InstallFinished(Ok(status))) => {
                    self.ra_installing = false;
                    let path_str = status.path.as_ref()
                        .map(|p| p.to_string_lossy().to_string())
                        .unwrap_or_default();
                    self.ra_install_log = format!("✅ Installed: {}", path_str);
                    self.ra_status = status;
                    // LSP を再起動するため lsp_client をリセット（次の update() で再起動される）
                    self.lsp_client = None;
                }
                AppMessage::Toolchain(crate::app::ToolchainMsg::InstallFinished(Err(e))) => {
                    self.ra_installing = false;
                    self.ra_install_log = format!("❌ Install failed: {}", e);
                }
                AppMessage::BuildAnalysis(stats) => {
                    self.build_stats = Some(stats);
                }
                AppMessage::RttData { channel, data } => {
                    self.rtt_log.push((channel, data));
                    if self.rtt_log.len() > 10000 {
                        self.rtt_log.drain(0..1000);
                    }
                }
                AppMessage::ElfAnalysis(info) => {
                    self.elf_info = Some(info);
                }
                AppMessage::StackAnalysis(report) => {
                    self.stack_report = Some(report);
                    self.show_stack_panel = true;
                }
                AppMessage::Error(e) => {
                    self.build_log = format!("[ERROR] {}", e);
                }
                _ => {}
            }
        }

        // Poll debug events
        if let Some(ref rx) = self.debug_evt_rx {
            while let Ok(evt) = rx.try_recv() {
                match evt {
                    crate::core::debugger::DebugEvent::Connected { .. } => {
                        self.debug_connected = true;
                        self.debug_error.clear();
                    }
                    crate::core::debugger::DebugEvent::Disconnected => {
                        self.debug_connected = false;
                        self.debug_halted = false;
                        self.debug_registers.clear();
                    }
                    crate::core::debugger::DebugEvent::Halted => {
                        self.debug_halted = true;
                    }
                    crate::core::debugger::DebugEvent::Continued => {
                        self.debug_halted = false;
                    }
                    crate::core::debugger::DebugEvent::Registers(regs) => {
                        self.debug_registers = regs;
                    }
                    crate::core::debugger::DebugEvent::MemoryRead { addr, data } => {
                        self.debug_memory_addr = addr;
                        self.debug_memory = data;
                    }
                    crate::core::debugger::DebugEvent::RttData { channel, data } => {
                        self.rtt_log.push((channel, data));
                        if self.rtt_log.len() > 1000 {
                            self.rtt_log.drain(0..500);
                        }
                    }
                    crate::core::debugger::DebugEvent::ProbeList(_) => {}
                    crate::core::debugger::DebugEvent::Error(e) => {
                        self.debug_error = e;
                    }
                }
            }
        }
    }
}

impl IdeApp {
    /// LSP サーバーを (再)起動する。プロジェクト切り替え時に呼ぶ。
    pub fn start_or_restart_lsp(&mut self) {
        // 旧クライアントをドロップ（プロセスも終了する）
        self.lsp_client = None;
        self.lsp_initialized = false;
        self.pending_did_opens.clear();

        let ws_dir = self.config.workspace_dir.clone();
        if !ws_dir.exists() { return; }

        let (lsp_tx, lsp_rx) = crossbeam_channel::unbounded::<crate::core::lsp::LspMessage>();
        let ra_path = self.config.rust_analyzer_path.clone();

        let Some(client) = crate::core::lsp::start_lsp(ws_dir, lsp_tx, ra_path) else { return };

        // 現在開いているファイルに did_open を送る
        if let Some(ref path) = self.file_path.clone() {
            let uri = format!("file:///{}", path.to_string_lossy().replace('\\', "/"));
            client.did_open(&uri, &self.editor_text);
        }

        self.lsp_client = Some(client);

        let app_msg_tx = self.msg_tx.clone();
        std::thread::spawn(move || {
            while let Ok(msg) = lsp_rx.recv() {
                match msg {
                    crate::core::lsp::LspMessage::CompletionItems(items) => {
                        let _ = app_msg_tx.send(AppMessage::LspCompletion(items));
                    }
                    crate::core::lsp::LspMessage::Diagnostics(diags) => {
                        let _ = app_msg_tx.send(AppMessage::LspDiagnostic(diags));
                    }
                    crate::core::lsp::LspMessage::Error(e) => {
                        let _ = app_msg_tx.send(AppMessage::Error(e));
                    }
                    crate::core::lsp::LspMessage::Initialized => {
                        let _ = app_msg_tx.send(AppMessage::LspInitialized);
                    }
                }
            }
        });
    }

    /// 現在のタブ内容を open_tabs に保存する
    pub fn sync_active_tab(&mut self) {
        if let Some(tab) = self.open_tabs.get_mut(self.active_tab) {
            tab.content = self.editor_text.clone();
            tab.is_dirty = self.is_dirty;
        }
    }

    /// ファイルを新しいタブで開く（既に開いていればそのタブに切り替え）
    pub fn open_file_in_tab(&mut self, path: PathBuf) {
        // 既に開いていたらそのタブに切り替え
        if let Some(idx) = self.open_tabs.iter().position(|t| t.path == path) {
            self.switch_to_tab(idx);
            return;
        }
        let content = std::fs::read_to_string(&path).unwrap_or_default();
        self.sync_active_tab();
        self.open_tabs.push(FileTab {
            path: path.clone(),
            content: content.clone(),
            is_dirty: false,
        });
        self.active_tab = self.open_tabs.len() - 1;
        // LSP に did_open を送る（初期化完了後でないと rust-analyzer が無視する）
        {
            let uri = format!("file:///{}", path.to_string_lossy().replace('\\', "/"));
            if self.lsp_initialized {
                if let Some(ref lsp) = self.lsp_client {
                    lsp.did_open(&uri, &content);
                }
            } else if self.lsp_client.is_some() {
                // 初期化待ちのためバッファに追加
                self.pending_did_opens.push((uri, content.clone()));
            }
        }
        self.editor_text = content;
        self.file_path = Some(path);
        self.is_dirty = false;
    }

    /// タブを切り替える
    pub fn switch_to_tab(&mut self, idx: usize) {
        if idx == self.active_tab && !self.open_tabs.is_empty() { return; }
        self.sync_active_tab();
        self.active_tab = idx;
        if let Some(tab) = self.open_tabs.get(idx) {
            self.editor_text = tab.content.clone();
            self.file_path = Some(tab.path.clone());
            self.is_dirty = tab.is_dirty;
        }
    }

    /// タブを閉じる（dirty なら先に保存）
    pub fn close_tab(&mut self, idx: usize) {
        if idx >= self.open_tabs.len() { return; }
        // 閉じる前に保存
        let tab = &self.open_tabs[idx];
        if tab.is_dirty {
            write_or_log(&tab.path, &tab.content, &mut self.build_log);
        }
        self.open_tabs.remove(idx);
        if self.open_tabs.is_empty() {
            self.editor_text = String::new();
            self.file_path = None;
            self.is_dirty = false;
            self.active_tab = 0;
        } else {
            self.active_tab = self.active_tab.min(self.open_tabs.len().saturating_sub(1));
            if let Some(tab) = self.open_tabs.get(self.active_tab).cloned() {
                self.editor_text = tab.content;
                self.file_path = Some(tab.path);
                self.is_dirty = tab.is_dirty;
            } else {
                self.editor_text = String::new();
                self.file_path = None;
                self.is_dirty = false;
            }
        }
    }

    /// ワークスペースのファイル一覧を更新する
    pub fn refresh_workspace_files(&mut self) {
        self.workspace_files = scan_workspace_files(&self.config.workspace_dir);
    }

    /// Returns a stable reference to the selected BoardPreset (bounds-safe).
    pub fn selected_board_preset(&self) -> &'static crate::core::board::BoardPreset {
        let presets = crate::core::board::BOARD_PRESETS;
        presets.get(self.selected_board)
            .or_else(|| presets.first())
            .unwrap_or_else(|| panic!("BOARD_PRESETS must not be empty"))
    }

    /// Clamps selected_board to valid range.
    pub fn sanitize_selected_board(&mut self) {
        let len = crate::core::board::BOARD_PRESETS.len();
        if self.selected_board >= len.max(1) {
            self.selected_board = 0;
        }
    }
}

/// ワークスペース内の編集対象ファイルを収集する
fn scan_workspace_files(workspace: &std::path::Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    if !workspace.exists() {
        return files;
    }
    // src/ 以下を再帰収集
    let src_dir = workspace.join("src");
    if let Ok(entries) = std::fs::read_dir(&src_dir) {
        for entry in entries.flatten() {
            let p = entry.path();
            if p.is_file() {
                files.push(p);
            }
        }
    }
    // .cargo/ ディレクトリ
    let cargo_dir = workspace.join(".cargo");
    if let Ok(entries) = std::fs::read_dir(&cargo_dir) {
        for entry in entries.flatten() {
            let p = entry.path();
            if p.is_file() {
                files.push(p);
            }
        }
    }
    // ルートファイル
    for name in &["Cargo.toml", "memory.x", "build.rs", "rust-toolchain.toml"] {
        let p = workspace.join(name);
        if p.exists() {
            files.push(p);
        }
    }
    files.sort();
    files
}

impl eframe::App for IdeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // process incoming messages from core/background threads
        self.handle_messages();

        // Ctrl+S save shortcut
        if ctx.input(|i| i.modifiers.ctrl && i.key_pressed(egui::Key::S)) {
            if let Some(ref path) = self.file_path.clone() {
                if std::fs::write(path, &self.editor_text).is_ok() {
                    self.is_dirty = false;
                    let _ = self.config.save();
                }
            }
        }

        let msg_tx = self.msg_tx.clone();

        // Menu bar
        let mut settings_clicked = false;
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("📁 New Project...").clicked() {
                        self.new_project_name = String::new();
                        self.new_project_base_dir = self.config.workspace_dir.parent()
                            .unwrap_or(&self.config.workspace_dir).to_path_buf();
                        self.show_new_project_dialog = true;
                        ui.close_menu();
                    }
                    if ui.button("📂 Open Project...").clicked() {
                        if let Some(dir) = rfd::FileDialog::new().pick_folder() {
                            match crate::core::project::open_project(&dir) {
                                Ok(info) => {
                                    self.config.workspace_dir = dir.clone();
                                    self.project_name = info.project_name;
                                    if let Some(board) = info.board {
                                        if let Some(idx) = crate::core::board::BOARD_PRESETS
                                            .iter().position(|p| p.kind == board) {
                                            self.selected_board = idx;
                                        }
                                    }
                                    self.open_tabs.clear();
                                    self.active_tab = 0;
                                    self.start_or_restart_lsp();
                                    let main_rs = dir.join("src").join("main.rs");
                                    self.open_file_in_tab(main_rs);
                                    self.refresh_workspace_files();
                                    let _ = self.config.save();
                                    self.build_log = format!(
                                        "[OK] プロジェクトを開きました: {}",
                                        dir.display()
                                    );
                                }
                                Err(e) => {
                                    self.build_log = format!("[ERROR] プロジェクトを開けませんでした: {}", e);
                                }
                            }
                        }
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("💾 Save  Ctrl+S").clicked() {
                        if let Some(ref path) = self.file_path.clone() {
                            if let Err(e) = std::fs::write(path, &self.editor_text) {
                                self.build_log = format!("[ERROR] 保存失敗: {}", e);
                            } else {
                                self.is_dirty = false;
                                let _ = self.config.save();
                                self.build_log = "[OK] 保存しました".to_string();
                            }
                        } else {
                            if let Some(path) = rfd::FileDialog::new()
                                .add_filter("Rust", &["rs"]).save_file() {
                                if let Err(e) = std::fs::write(&path, &self.editor_text) {
                                    self.build_log = format!("[ERROR] 保存失敗: {}", e);
                                } else {
                                    self.file_path = Some(path);
                                    self.is_dirty = false;
                                    let _ = self.config.save();
                                    self.build_log = "[OK] 保存しました".to_string();
                                }
                            }
                        }
                        ui.close_menu();
                    }
                    if ui.button("💾 Save As...").clicked() {
                        if let Some(path) = rfd::FileDialog::new()
                            .add_filter("Rust", &["rs"]).save_file() {
                            if let Err(e) = std::fs::write(&path, &self.editor_text) {
                                self.build_log = format!("[ERROR] 保存失敗: {}", e);
                            } else {
                                self.file_path = Some(path);
                                self.is_dirty = false;
                                self.build_log = "[OK] 名前を付けて保存しました".to_string();
                            }
                        }
                        ui.close_menu();
                    }
                });
                ui.menu_button("Build", |ui| {
                    if ui.button("Build").clicked() {}
                });
                ui.menu_button("Help", |ui| {
                    let _ = ui.button("About");
                    if ui.button("使い方ガイド").clicked() {
                        self.show_help_window = true;
                    }
                });
                if ui.button("Settings").clicked() {
                    settings_clicked = true;
                }
                if ui.button("📋 SVD").clicked() {
                    self.show_svd_panel = !self.show_svd_panel;
                }
                ui.checkbox(&mut self.show_debug_panel, "Debug Panel");
            });
        });
        if settings_clicked { self.show_settings = !self.show_settings; }

        // Left: Board picker + Build panel
        egui::SidePanel::left("left_panel")
    .resizable(true)
    .default_width(crate::core::config::LEFT_PANEL_WIDTH)
    .width_range(100.0..=600.0)
    .show(ctx, |ui| {
            ui.heading("Board");
            crate::ui::board_picker::ui_board_picker(self, ui, &msg_tx);
            ui.separator();
            crate::ui::build_panel::ui_build_panel(self, ui, &msg_tx);
            ui.separator();
            crate::ui::file_explorer::ui_file_explorer(self, ui);
        });

        // Right: Serial monitor / Docs
        egui::SidePanel::right("right_panel")
    .resizable(true)
    .default_width(crate::core::config::RIGHT_PANEL_WIDTH)
    .width_range(100.0..=600.0)
    .show(ctx, |ui| {
            // Tab buttons
            ui.horizontal(|ui| {
                ui.selectable_value(
                    &mut self.right_tab,
                    crate::ui::help_panel::RightTab::SerialMonitor,
                    "🔌 Serial",
                );
                ui.selectable_value(
                    &mut self.right_tab,
                    crate::ui::help_panel::RightTab::Docs,
                    "📖 Docs",
                );
                ui.selectable_value(
                    &mut self.right_tab,
                    crate::ui::help_panel::RightTab::Pinout,
                    "📌 ピンアウト",
                );
            });
            ui.separator();
            match self.right_tab {
                crate::ui::help_panel::RightTab::SerialMonitor => {
                    crate::ui::serial_monitor::ui_serial_monitor(self, ui, &msg_tx);
                }
                crate::ui::help_panel::RightTab::Docs => {
                    crate::ui::help_panel::ui_help_panel(self, ui);
                }
                crate::ui::help_panel::RightTab::Pinout => {
                    crate::ui::pinout_panel::ui_pinout_panel(self, ui);
                }
            }
        });

        if self.show_debug_panel {
            egui::SidePanel::right("debug_panel")
                .min_width(280.0)
                .default_width(300.0)
                .show(ctx, |ui| {
                    ui.heading("🐛 Debug");
                    ui.separator();
                    crate::ui::debug_panel::ui_debug_panel(self, ui);
                });
        }

        if self.show_svd_panel {
            egui::SidePanel::right("svd_panel")
                .resizable(true)
                .min_width(280.0)
                .default_width(320.0)
                .show(ctx, |ui| {
                    ui.heading("📋 SVD Viewer");
                    ui.separator();
                    crate::ui::svd_panel::ui_svd_panel(self, ui);
                });
        }

        // Center: Editor
        egui::CentralPanel::default().show(ctx, |ui| {
            crate::ui::editor::ui_editor(self, ui, &msg_tx);
        });

        // Bottom: Status bar
        let mut quit_clicked = false;
        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if let Some(ref name) = self.project_name {
                    ui.label(format!("📁 {}", name));
                    ui.separator();
                }
                ui.label(format!("Board: {}", crate::core::board::BOARD_PRESETS.get(self.selected_board).map(|p| p.display_name).unwrap_or("<unknown>")));
                ui.separator();
                ui.label(format!("Ln {}, Col {}", self.cursor_line, self.cursor_col));
                if self.is_dirty { ui.label("●"); }
                ui.separator();
                // LSP ステータス
                let lsp_text = if self.lsp_client.is_some() {
                    if self.lsp_initialized { "LSP ●" } else { "LSP…" }
                } else {
                    "LSP ✕"
                };
                let lsp_color = if self.lsp_client.is_some() {
                    if self.lsp_initialized { egui::Color32::from_rgb(80, 200, 80) } else { egui::Color32::YELLOW }
                } else {
                    egui::Color32::from_rgb(200, 80, 80)
                };
                ui.label(egui::RichText::new(lsp_text).color(lsp_color));
                ui.separator();
                if self.is_building { ui.label("Building..."); }
                if self.is_flashing { ui.label("Flashing..."); }
                if ui.button("Quit").clicked() {
                        quit_clicked = true;
                    }
            });
        });
        if quit_clicked {
            // graceful exit - left as no-op for now
        }

        if self.show_settings {
            let mut show = self.show_settings;
            egui::Window::new("Settings")
                .open(&mut show)
                .min_width(420.0)
                .show(ctx, |ui| {
                    crate::ui::settings::ui_settings(self, ui);
                });
            self.show_settings = show;
        }

        if self.show_help_window {
            let mut show = self.show_help_window;
            egui::Window::new("📖 使い方ガイド")
                .open(&mut show)
                .resizable(true)
                .default_size([800.0, 600.0])
                .show(ctx, |ui| {
                    crate::ui::help_panel::ui_help_panel(self, ui);
                });
            self.show_help_window = show;
        }

        // ELF Viewer window
        if self.show_elf_panel {
            let mut show = self.show_elf_panel;
            egui::Window::new("📂 ELF Viewer")
                .open(&mut show)
                .default_size([700.0, 500.0])
                .show(ctx, |ui| {
                    crate::ui::elf_panel::ui_elf_panel(self, ui);
                });
            self.show_elf_panel = show;
        }

        // Stack Analysis window
        if self.show_stack_panel {
            let mut show = self.show_stack_panel;
            egui::Window::new("📊 Stack Analysis")
                .open(&mut show)
                .default_size([700.0, 500.0])
                .show(ctx, |ui| {
                    crate::ui::stack_panel::ui_stack_panel(self, ui);
                });
            self.show_stack_panel = show;
        }

        // テンプレート読み込み確認ダイアログ
        if let Some(board_idx) = self.template_confirm_board {
            let board_name = crate::core::board::BOARD_PRESETS
                .get(board_idx)
                .map(|p| p.display_name)
                .unwrap_or("Unknown");

            let mut close = false;
            egui::Window::new("テンプレートを読み込みますか？")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.add_space(8.0);
                        ui.label(format!(
                            "「{}」のLチカテンプレートをエディタに読み込みますか？",
                            board_name
                        ));
                        ui.add_space(12.0);
                        ui.horizontal(|ui| {
                            if ui.button("✅ 読み込む").clicked() {
                                // テンプレートを取得してエディタに設定（ファイルをワークスペースに書き出す）
                                if let Some(preset) = crate::core::board::BOARD_PRESETS.get(board_idx) {
                                    let ws = self.config.workspace_dir.clone();
                                    // IDE 本体のソースディレクトリへの上書きを防ぐ（簡易チェック）
                                    if ws.join("src").join("lib.rs").exists() {
                                        self.build_log = "[ERROR] ワークスペースが IDE 本体のディレクトリです。Settings からワークスペースを変更してください。".to_string();
                                    } else {
                                        match crate::templates::create_blink_project(&ws, &preset.kind) {
                                            Ok(_) => {
                                                let main_rs_path = ws.join("src").join("main.rs");
                                                self.open_tabs.clear();
                                                self.active_tab = 0;
                                                self.open_file_in_tab(main_rs_path);
                                                self.refresh_workspace_files();
                                                self.is_dirty = false;
                                            }
                                            Err(e) => {
                                                self.build_log = format!("[ERROR] テンプレートの書き出しに失敗: {}", e);
                                            }
                                        }
                                    }
                                }
                                close = true;
                            }
                            ui.add_space(8.0);
                            if ui.button("❌ 読み込まない").clicked() {
                                close = true;
                            }
                        });
                        ui.add_space(8.0);
                    });
                });

            if close {
                self.template_confirm_board = None;
            }
        }

        // 新規プロジェクトダイアログ
        if self.show_new_project_dialog {
            let mut close = false;
            let mut do_create = false;
            egui::Window::new("📁 新規プロジェクト作成")
                .collapsible(false)
                .resizable(false)
                .min_width(420.0)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .show(ctx, |ui| {
                    egui::Grid::new("new_proj_grid")
                        .num_columns(2)
                        .spacing([8.0, 6.0])
                        .show(ui, |ui| {
                            ui.label("プロジェクト名:");
                            ui.text_edit_singleline(&mut self.new_project_name);
                            ui.end_row();
                            ui.label("保存先:");
                            ui.horizontal(|ui| {
                                let dir_str = self.new_project_base_dir.to_string_lossy();
                                ui.label(egui::RichText::new(dir_str.as_ref()).monospace().small());
                                if ui.small_button("変更...").clicked() {
                                    if let Some(dir) = rfd::FileDialog::new().pick_folder() {
                                        self.new_project_base_dir = dir;
                                    }
                                }
                            });
                            ui.end_row();
                            ui.label("ボード:");
                            ui.label(
                                crate::core::board::BOARD_PRESETS
                                    .get(self.selected_board)
                                    .map(|p| p.display_name)
                                    .unwrap_or("<unknown>"),
                            );
                            ui.end_row();
                            ui.label("作成先:");
                            let project_dir = self.new_project_base_dir.join(self.new_project_name.trim());
                            ui.label(
                                egui::RichText::new(project_dir.to_string_lossy().as_ref())
                                    .monospace()
                                    .small()
                                    .weak(),
                            );
                            ui.end_row();
                        });
                    ui.add_space(8.0);
                    ui.horizontal(|ui| {
                        let can_create = !self.new_project_name.trim().is_empty();
                        if ui.add_enabled(can_create, egui::Button::new("✅ 作成")).clicked() {
                            do_create = true;
                            close = true;
                        }
                        ui.add_space(8.0);
                        if ui.button("❌ キャンセル").clicked() {
                            close = true;
                        }
                    });
                });

            if do_create && !self.new_project_name.trim().is_empty() {
                let project_dir = self.new_project_base_dir.join(self.new_project_name.trim());
                if project_dir.join("src").join("lib.rs").exists() {
                    self.build_log =
                        "[ERROR] そのパスはIDE本体のディレクトリです。別の場所を選択してください。"
                            .to_string();
                } else if let Some(preset) = crate::core::board::BOARD_PRESETS.get(self.selected_board) {
                    match crate::templates::create_blink_project(&project_dir, &preset.kind) {
                        Ok(_) => {
                            let main_rs_path = project_dir.join("src").join("main.rs");
                            self.config.workspace_dir = project_dir.clone();
                            self.project_name = Some(self.new_project_name.trim().to_string());
                            self.open_tabs.clear();
                            self.active_tab = 0;
                            self.open_file_in_tab(main_rs_path);
                            self.refresh_workspace_files();
                            self.is_dirty = false;
                            self.build_log = format!(
                                "[OK] プロジェクト「{}」を作成しました: {}",
                                self.new_project_name.trim(),
                                project_dir.display()
                            );
                            let _ = self.config.save();
                        }
                        Err(e) => {
                            self.build_log = format!("[ERROR] プロジェクト作成失敗: {}", e);
                        }
                    }
                }
            }
            if close {
                self.show_new_project_dialog = false;
            }
        }

        // 新規ファイル作成ダイアログ
        if self.show_new_file_dialog {
            let mut close_file_dialog = false;
            let mut do_create_file = false;
            egui::Window::new("➕ 新規ファイル作成")
                .collapsible(false)
                .resizable(false)
                .min_width(340.0)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.label("ファイル名 (src/):");
                        ui.text_edit_singleline(&mut self.new_file_name);
                    });
                    ui.label(
                        egui::RichText::new("例: led.rs  sensors.rs  config.rs")
                            .small()
                            .weak(),
                    );
                    ui.add_space(6.0);
                    ui.horizontal(|ui| {
                        let name = self.new_file_name.trim();
                        let can_create = !name.is_empty() && name.ends_with(".rs");
                        if ui.add_enabled(can_create, egui::Button::new("✅ 作成")).clicked() {
                            do_create_file = true;
                            close_file_dialog = true;
                        }
                        ui.add_space(8.0);
                        if ui.button("❌ キャンセル").clicked() {
                            close_file_dialog = true;
                        }
                    });
                });
            if do_create_file {
                let name = self.new_file_name.trim().to_string();
                let src_dir = self.config.workspace_dir.join("src");
                let path = src_dir.join(&name);
                if std::fs::create_dir_all(&src_dir).is_ok() {
                    if !path.exists() {
                        write_or_log(&path, &format!("// {}\n", name), &mut self.build_log);
                    }
                    self.open_file_in_tab(path);
                    self.refresh_workspace_files();
                }
            }
            if close_file_dialog {
                self.show_new_file_dialog = false;
            }
        }

        // request repaint for async updates
        ctx.request_repaint();
    }
}



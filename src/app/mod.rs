// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright 2026 ALLoIDE contributors

use crossbeam_channel::{unbounded, Receiver, Sender};

use std::collections::VecDeque;
use std::path::PathBuf;

pub mod config;
mod events;
mod workspace;

pub(crate) use workspace::write_or_log;
pub use workspace::FileTab;

use crate::core::event::CoreEvent;

#[allow(dead_code)]
#[derive(Clone)]
pub struct PlotChannel {
    pub name: String,
    pub color: egui::Color32,
    pub values: std::collections::VecDeque<f64>,
}

pub struct IdeApp {
    // エディタ
    pub editor_text: String,
    pub file_path: Option<PathBuf>,
    pub is_dirty: bool,

    // ボード・ポート選択
    pub selected_board: usize, // BOARD_PRESETS のインデックス
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
    pub rtt_log: Vec<(u32, String)>, // (channel, message)
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
    pub msg_tx: Sender<CoreEvent>,
    pub msg_rx: Receiver<CoreEvent>,

    // 設定
    pub config: config::AppConfig,
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

    // Agent
    pub agent_prompt: String,
    pub agent_log: String,
    pub agent_running: bool,
    pub agent_allow_edits: bool,

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
        let config = config::AppConfig::load().unwrap_or_default();

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

        let new_project_base_dir = config
            .workspace_dir
            .parent()
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
            agent_prompt: String::new(),
            agent_log: String::new(),
            agent_running: false,
            agent_allow_edits: false,
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

        // 起動時に workspace_dir/dist/*.elf が存在すれば last_dist_path を自動設定
        let dist_dir = app.config.workspace_dir.join("dist");
        if dist_dir.exists() {
            let has_elf = std::fs::read_dir(&dist_dir)
                .ok()
                .and_then(|mut rd| {
                    rd.find_map(|e| {
                        let p = e.ok()?.path();
                        if p.extension().map(|x| x == "elf").unwrap_or(false) {
                            Some(())
                        } else {
                            None
                        }
                    })
                })
                .is_some();
            if has_elf {
                app.last_dist_path = Some(dist_dir);
            }
        }

        // LSP 起動（rust-analyzerがあれば）。workspace_dir を使用
        let ws_dir = app.config.workspace_dir.clone();
        let ws = if ws_dir.exists() {
            Some(ws_dir)
        } else {
            std::env::current_dir().ok()
        };
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
                                let _ = app_msg_tx.send(CoreEvent::LspCompletion(items));
                            }
                            crate::core::lsp::LspMessage::Diagnostics(diags) => {
                                let _ = app_msg_tx.send(CoreEvent::LspDiagnostic(diags));
                            }
                            crate::core::lsp::LspMessage::Error(e) => {
                                let _ = app_msg_tx.send(CoreEvent::Error(e));
                            }
                            crate::core::lsp::LspMessage::Initialized => {
                                let _ = app_msg_tx.send(CoreEvent::LspInitialized);
                            }
                        }
                    }
                });
            }
        }

        app
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
        if !ws_dir.exists() {
            return;
        }

        let (lsp_tx, lsp_rx) = crossbeam_channel::unbounded::<crate::core::lsp::LspMessage>();
        let ra_path = self.config.rust_analyzer_path.clone();

        let Some(client) = crate::core::lsp::start_lsp(ws_dir, lsp_tx, ra_path) else {
            return;
        };

        // 現在開いているファイルに did_open を送る
        if let Some(ref path) = self.file_path.clone() {
            let uri = crate::core::lsp::file_uri(path);
            client.did_open(&uri, &self.editor_text);
        }

        self.lsp_client = Some(client);

        let app_msg_tx = self.msg_tx.clone();
        std::thread::spawn(move || {
            while let Ok(msg) = lsp_rx.recv() {
                match msg {
                    crate::core::lsp::LspMessage::CompletionItems(items) => {
                        let _ = app_msg_tx.send(CoreEvent::LspCompletion(items));
                    }
                    crate::core::lsp::LspMessage::Diagnostics(diags) => {
                        let _ = app_msg_tx.send(CoreEvent::LspDiagnostic(diags));
                    }
                    crate::core::lsp::LspMessage::Error(e) => {
                        let _ = app_msg_tx.send(CoreEvent::Error(e));
                    }
                    crate::core::lsp::LspMessage::Initialized => {
                        let _ = app_msg_tx.send(CoreEvent::LspInitialized);
                    }
                }
            }
        });
    }

    /// Returns a stable reference to the selected BoardPreset (bounds-safe).
    pub fn selected_board_preset(&self) -> &'static crate::core::board::BoardPreset {
        let presets = crate::core::board::BOARD_PRESETS;
        presets
            .get(self.selected_board)
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

impl eframe::App for IdeApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        crate::ui::workbench::update(self, ctx, frame);
    }
}

#[cfg(test)]
mod tests {
    use super::{events::append_log_once, workspace::reload_clean_tabs, FileTab};

    #[test]
    fn finished_output_does_not_replace_or_duplicate_progress() {
        let mut log = "[BUILD] ビルド開始...\nCompiling app\n".to_string();
        append_log_once(&mut log, "Compiling app\nFinished dev");

        assert_eq!(log, "[BUILD] ビルド開始...\nCompiling app\nFinished dev\n");
    }

    #[test]
    fn agent_reload_preserves_dirty_tabs() {
        let dir = std::env::temp_dir().join(format!("alloide-agent-reload-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let clean_path = dir.join("clean.rs");
        let dirty_path = dir.join("dirty.rs");
        let deleted_path = dir.join("deleted.rs");
        std::fs::write(&clean_path, "from disk").unwrap();
        std::fs::write(&dirty_path, "from disk").unwrap();
        let mut tabs = vec![
            FileTab {
                path: clean_path,
                content: "old".into(),
                is_dirty: false,
            },
            FileTab {
                path: dirty_path,
                content: "unsaved".into(),
                is_dirty: true,
            },
            FileTab {
                path: deleted_path,
                content: "stale".into(),
                is_dirty: false,
            },
        ];

        reload_clean_tabs(&mut tabs);

        assert_eq!(tabs[0].content, "from disk");
        assert_eq!(tabs[1].content, "unsaved");
        assert_eq!(tabs.len(), 2);
        std::fs::remove_dir_all(dir).unwrap();
    }
}

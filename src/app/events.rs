// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright 2026 ALLoIDE contributors

use super::{workspace::reload_clean_tabs, IdeApp, PlotChannel};
use crate::core::event::{BuildMsg, CoreEvent, FlashMsg, SerialMsg, ToolchainMsg};

pub(super) fn append_log_once(log: &mut String, text: &str) {
    for line in text.lines() {
        if !log.lines().any(|existing| existing == line) {
            if !log.ends_with('\n') {
                log.push('\n');
            }
            log.push_str(line);
            log.push('\n');
        }
    }
}

fn parse_plot_line(line: &str) -> Vec<(String, f64)> {
    let line = line.trim();
    let mut result = Vec::new();
    for (i, part) in line.split(',').enumerate() {
        let part = part.trim();
        if let Some(colon) = part.find(':') {
            let name = part[..colon].trim().to_string();
            let val_str = part[colon + 1..].trim();
            if let Ok(v) = val_str.parse::<f64>() {
                result.push((name, v));
            }
        } else if let Ok(v) = part.parse::<f64>() {
            result.push((format!("ch{}", i), v));
        }
    }
    result
}

impl IdeApp {
    pub(crate) fn handle_messages(&mut self) {
        while let Ok(msg) = self.msg_rx.try_recv() {
            match msg {
                CoreEvent::Agent(crate::core::agent::AgentEvent::Started) => {
                    self.agent_running = true;
                }
                CoreEvent::Agent(crate::core::agent::AgentEvent::Output(output)) => {
                    self.agent_log.push_str(&output);
                    self.agent_log.push('\n');
                }
                CoreEvent::Agent(crate::core::agent::AgentEvent::Finished(result)) => {
                    self.agent_running = false;
                    if let Err(error) = result {
                        if !self.agent_log.ends_with('\n') {
                            self.agent_log.push('\n');
                        }
                        self.agent_log.push_str(&format!("[ERROR] {error}\n"));
                    }

                    self.sync_active_tab();
                    reload_clean_tabs(&mut self.open_tabs);
                    self.active_tab = self.active_tab.min(self.open_tabs.len().saturating_sub(1));
                    if let Some(tab) = self.open_tabs.get(self.active_tab) {
                        self.editor_text = tab.content.clone();
                        self.file_path = Some(tab.path.clone());
                        self.is_dirty = tab.is_dirty;
                    } else {
                        self.editor_text.clear();
                        self.file_path = None;
                        self.is_dirty = false;
                    }
                    self.refresh_workspace_files();
                }
                CoreEvent::Build(BuildMsg::Started) => {
                    self.is_building = true;
                    self.build_log = "[BUILD] ビルド開始...\n".to_string();
                }
                CoreEvent::Build(BuildMsg::Progress(text)) => {
                    self.build_log.push_str(&text);
                    self.build_log.push('\n');
                }
                CoreEvent::Flash(FlashMsg::Started) => {
                    self.is_flashing = true;
                    self.build_log.push_str("\n[FLASH] フラッシュ開始...\n");
                }
                CoreEvent::Flash(FlashMsg::Progress(text)) => {
                    self.build_log.push_str(&format!("{}\n", text));
                }
                CoreEvent::Build(BuildMsg::Finished(br)) => {
                    self.is_building = false;
                    append_log_once(&mut self.build_log, &br.stdout);
                    append_log_once(&mut self.build_log, &br.stderr);
                    if br.success {
                        self.build_log.push_str("[SUCCESS]\n");
                        if let Some(p) = br.dist_path {
                            self.last_dist_path = Some(p.clone());
                            // Spawn analysis of ELF in background
                            if let Some(board) =
                                crate::core::board::BOARD_PRESETS.get(self.selected_board)
                            {
                                let board_clone = board.clone();
                                let tx = self.msg_tx.clone();
                                let target = p.clone();
                                std::thread::spawn(move || {
                                    if let Some(elf) =
                                        crate::core::build_analyzer::find_elf(&target, &board_clone)
                                    {
                                        if let Ok(stats) = crate::core::build_analyzer::analyze_elf(
                                            &elf,
                                            &board_clone,
                                        ) {
                                            let _ = tx.send(CoreEvent::BuildAnalysis(stats));
                                        }
                                    }
                                });
                            }
                            // Build & Flash: auto-trigger flash after successful build
                            if self.auto_flash_after_build {
                                self.auto_flash_after_build = false;
                                if let Some(preset) =
                                    crate::core::board::BOARD_PRESETS.get(self.selected_board)
                                {
                                    // Find ELF artifact for flashing
                                    let elf_path = std::fs::read_dir(&p).ok().and_then(|mut rd| {
                                        rd.find_map(|e| {
                                            let path = e.ok()?.path();
                                            if path.extension().map(|x| x == "elf").unwrap_or(false)
                                            {
                                                Some(path)
                                            } else {
                                                None
                                            }
                                        })
                                    });
                                    if let Some(artifact) = elf_path {
                                        let port = self
                                            .available_ports
                                            .get(self.selected_port)
                                            .cloned()
                                            .unwrap_or_default();
                                        let flash_req = crate::core::flasher::FlashRequest {
                                            board: preset.kind.clone(),
                                            artifact,
                                            port,
                                        };
                                        self.is_flashing = true;
                                        self.build_log =
                                            format!("[SUCCESS + FLASH開始]\n{}", self.build_log);
                                        crate::core::flasher::flash_async(
                                            flash_req,
                                            self.msg_tx.clone(),
                                        );
                                    } else {
                                        self.build_log = format!("[SUCCESS]\n[ERROR] フラッシュ用ELFが見つかりません\n{}", self.build_log);
                                    }
                                }
                            }
                        }
                    } else {
                        self.auto_flash_after_build = false;
                        self.build_log.push_str("[FAIL]\n");
                    }
                }
                CoreEvent::Flash(FlashMsg::Finished(fr)) => {
                    self.is_flashing = false;
                    append_log_once(&mut self.build_log, &fr.output);
                    self.build_log.push_str(if fr.success {
                        "[FLASH SUCCESS]\n"
                    } else {
                        "[FLASH FAIL]\n"
                    });
                }
                CoreEvent::Serial(SerialMsg::Line(line)) => {
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
                CoreEvent::Serial(SerialMsg::Error(error)) => {
                    self.serial_log.push_back(format!("[ERROR] {error}"));
                    while self.serial_log.len() > 500 {
                        self.serial_log.pop_front();
                    }
                }
                CoreEvent::Serial(SerialMsg::Connected) => {
                    self.is_serial_connected = true;
                }
                CoreEvent::Serial(SerialMsg::Disconnected) => {
                    self.is_serial_connected = false;
                    self.serial_tx = None;
                }
                CoreEvent::BoardDetected(Some(detected)) => {
                    self.selected_board = detected.board_index;
                    // ポートも自動選択
                    if let Some(pos) = self
                        .available_ports
                        .iter()
                        .position(|p| *p == detected.port_name)
                    {
                        self.selected_port = pos;
                    }
                    self.detection_result = Some(detected.description.clone());
                }
                CoreEvent::BoardDetected(None) => {
                    self.detection_result =
                        Some("No board detected. Please connect a board.".to_string());
                }
                CoreEvent::LspCompletion(items) if !items.is_empty() => {
                    self.lsp_completions = items;
                    self.show_completion = true;
                    self.completion_selected = 0;
                }
                CoreEvent::LspDiagnostic(diags) => {
                    self.lsp_diagnostics = diags;
                }
                CoreEvent::LspInitialized => {
                    self.lsp_initialized = true;
                    // Flush buffered did_open calls that were waiting for initialization
                    if let Some(ref lsp) = self.lsp_client {
                        let pending = std::mem::take(&mut self.pending_did_opens);
                        for (uri, content) in pending {
                            lsp.did_open(&uri, &content);
                        }
                    }
                }
                CoreEvent::Toolchain(ToolchainMsg::InstallStarted) => {
                    self.ra_installing = true;
                    self.ra_install_log = "Installing rust-analyzer via rustup...".to_string();
                }
                CoreEvent::Toolchain(ToolchainMsg::InstallFinished(Ok(status))) => {
                    self.ra_installing = false;
                    let path_str = status
                        .path
                        .as_ref()
                        .map(|p| p.to_string_lossy().to_string())
                        .unwrap_or_default();
                    self.ra_install_log = format!("✅ Installed: {}", path_str);
                    self.ra_status = status;
                    // LSP を再起動するため lsp_client をリセット（次の update() で再起動される）
                    self.lsp_client = None;
                }
                CoreEvent::Toolchain(ToolchainMsg::InstallFinished(Err(e))) => {
                    self.ra_installing = false;
                    self.ra_install_log = format!("❌ Install failed: {}", e);
                }
                CoreEvent::BuildAnalysis(stats) => {
                    self.build_stats = Some(stats);
                }
                CoreEvent::RttData { channel, data } => {
                    self.rtt_log.push((channel, data));
                    if self.rtt_log.len() > 10000 {
                        self.rtt_log.drain(0..1000);
                    }
                }
                CoreEvent::ElfAnalysis(info) => {
                    self.elf_info = Some(info);
                }
                CoreEvent::StackAnalysis(report) => {
                    self.stack_report = Some(report);
                    self.show_stack_panel = true;
                }
                CoreEvent::Error(e) => {
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

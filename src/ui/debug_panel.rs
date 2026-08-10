// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright 2026 rust-embedded-ide contributors

use crate::app::IdeApp;
use crate::core::debugger::DebugCommand;

pub fn ui_debug_panel(app: &mut IdeApp, ui: &mut egui::Ui) {
    // Check if target supports hardware debug
    let board_kind = crate::core::board::BOARD_PRESETS
        .get(app.selected_board)
        .map(|p| p.kind.clone())
        .unwrap_or(crate::core::board::BoardKind::Stm32F4);

    use crate::core::board::BoardKind;
    let is_arm = matches!(board_kind, BoardKind::MicroBitV2 | BoardKind::Stm32F4);

    if !is_arm {
        ui.colored_label(
            egui::Color32::YELLOW,
            "⚠ Hardware debug not supported for this target.",
        );
        ui.label("Use avr-gdb or esptool with external GDB for AVR/ESP32.");
        return;
    }

    // ─ 接続コントロール ─
    ui.horizontal(|ui| {
        ui.label("Chip:");
        ui.text_edit_singleline(&mut app.debug_chip_name);
    });

    ui.horizontal(|ui| {
        let connected = app.debug_connected;
        if !connected {
            if ui.button("🔌 Connect").clicked() {
                if let Some(ref tx) = app.debug_cmd_tx {
                    let _ = tx.send(DebugCommand::Connect {
                        chip: app.debug_chip_name.clone(),
                    });
                }
            }
        } else {
            if ui.button("⏹ Disconnect").clicked() {
                if let Some(ref tx) = app.debug_cmd_tx {
                    let _ = tx.send(DebugCommand::Disconnect);
                }
            }
        }

        if connected {
            ui.separator();
            if ui.button("⏸ Halt").clicked() {
                if let Some(ref tx) = app.debug_cmd_tx {
                    let _ = tx.send(DebugCommand::Halt);
                }
            }
            if ui.button("▶ Continue").clicked() {
                if let Some(ref tx) = app.debug_cmd_tx {
                    let _ = tx.send(DebugCommand::Continue);
                }
            }
            if ui.button("⮕ Step").clicked() {
                if let Some(ref tx) = app.debug_cmd_tx {
                    let _ = tx.send(DebugCommand::Step);
                }
            }
            if ui.button("🔄 Refresh").clicked() {
                if let Some(ref tx) = app.debug_cmd_tx {
                    let _ = tx.send(DebugCommand::ReadRegisters);
                }
            }
        }
    });

    // ステータス
    let status_color = if app.debug_connected {
        if app.debug_halted {
            egui::Color32::YELLOW
        } else {
            egui::Color32::GREEN
        }
    } else {
        egui::Color32::GRAY
    };
    let status_text = if app.debug_connected {
        if app.debug_halted {
            "● Halted"
        } else {
            "● Running"
        }
    } else {
        "○ Disconnected"
    };
    ui.colored_label(status_color, status_text);

    // エラーメッセージ
    if !app.debug_error.is_empty() {
        ui.colored_label(egui::Color32::RED, format!("⚠ {}", &app.debug_error));
    }

    ui.separator();
    // RTT panel
    crate::ui::rtt_panel::ui_rtt_panel(app, ui);

    // ─ レジスタテーブル ─
    ui.label(egui::RichText::new("CPU Registers").strong());
    if app.debug_registers.is_empty() {
        ui.label(
            egui::RichText::new("(no data — halt target to read registers)")
                .small()
                .color(egui::Color32::GRAY),
        );
    } else {
        egui::ScrollArea::vertical()
            .id_salt("debug_regs_scroll")
            .max_height(280.0)
            .show(ui, |ui| {
                egui::Grid::new("reg_grid")
                    .striped(true)
                    .min_col_width(60.0)
                    .show(ui, |ui| {
                        // Header
                        ui.label(egui::RichText::new("Name").strong().small());
                        ui.label(egui::RichText::new("Hex").strong().small());
                        ui.label(egui::RichText::new("Dec").strong().small());
                        ui.end_row();

                        for reg in &app.debug_registers {
                            ui.label(egui::RichText::new(&reg.name).monospace().small());
                            ui.label(
                                egui::RichText::new(reg.hex())
                                    .monospace()
                                    .small()
                                    .color(egui::Color32::from_rgb(100, 200, 255)),
                            );
                            ui.label(egui::RichText::new(reg.dec()).monospace().small());
                            ui.end_row();
                        }
                    });
            });
    }

    ui.separator();

    // ─ メモリウォッチ ─
    ui.label(egui::RichText::new("Memory Watch").strong());
    ui.horizontal(|ui| {
        ui.label("Addr (hex):");
        ui.text_edit_singleline(&mut app.debug_watch_addr);
        if ui.button("Read").clicked() {
            if let Ok(addr) = u64::from_str_radix(
                app.debug_watch_addr
                    .trim_start_matches("0x")
                    .trim_start_matches("0X"),
                16,
            ) {
                if let Some(ref tx) = app.debug_cmd_tx {
                    let _ = tx.send(DebugCommand::ReadMemory { addr, len: 32 });
                }
            }
        }
    });
    if !app.debug_memory.is_empty() {
        ui.label(
            egui::RichText::new(format!("@ 0x{:08X}:", app.debug_memory_addr))
                .monospace()
                .small(),
        );
        let hex_str: String = app
            .debug_memory
            .chunks(4)
            .map(|chunk| {
                chunk
                    .iter()
                    .map(|b| format!("{:02X}", b))
                    .collect::<Vec<_>>()
                    .join(" ")
            })
            .collect::<Vec<_>>()
            .join("  ");
        ui.label(egui::RichText::new(hex_str).monospace().small());
    }

    ui.separator();
    ui.label(egui::RichText::new("🔴 Breakpoints").strong());
    if app.breakpoints.is_empty() {
        ui.label(
            egui::RichText::new("(none — click gutter to add)")
                .small()
                .color(egui::Color32::GRAY),
        );
    } else {
        let mut to_remove: Option<usize> = None;
        let mut sorted: Vec<usize> = app.breakpoints.iter().copied().collect();
        sorted.sort_unstable();
        for &line in &sorted {
            ui.horizontal(|ui| {
                ui.colored_label(egui::Color32::RED, "●");
                ui.label(format!("Line {}", line));
                if ui.small_button("×").clicked() {
                    to_remove = Some(line);
                }
            });
        }
        if let Some(line) = to_remove {
            app.breakpoints.remove(&line);
        }
    }
}

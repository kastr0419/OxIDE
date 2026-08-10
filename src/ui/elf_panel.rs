// SPDX-License-Identifier: MIT OR Apache-2.0
// ELF viewer UI skeleton

use crate::app::IdeApp;
use egui::Ui;

pub fn ui_elf_panel(app: &mut IdeApp, ui: &mut Ui) {
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new("🔍 ELF Viewer").strong());
        ui.separator();
        if ui.button("Analyze ELF").clicked() {
            let board = crate::core::board::BOARD_PRESETS
                .get(app.selected_board)
                .cloned();
            let project_dir = app.config.workspace_dir.clone();
            let tx = app.msg_tx.clone();
            std::thread::spawn(move || {
                if let Some(board) = board {
                    if let Some(elf_path) =
                        crate::core::build_analyzer::find_elf(&project_dir, &board)
                    {
                        match crate::core::elf_analyzer::analyze_elf(&elf_path) {
                            Ok(info) => {
                                let _ = tx.send(crate::app::AppMessage::ElfAnalysis(info));
                            }
                            Err(e) => {
                                let _ = tx.send(crate::app::AppMessage::Error(e.to_string()));
                            }
                        }
                    } else {
                        let _ = tx.send(crate::app::AppMessage::Error("ELF not found".to_string()));
                    }
                } else {
                    let _ = tx.send(crate::app::AppMessage::Error(
                        "No board selected".to_string(),
                    ));
                }
            });
        }
    });

    ui.add_space(8.0);

    if let Some(ref info) = app.elf_info {
        ui.label(format!("アーキテクチャ: {}", info.arch));
        ui.separator();
        ui.label(egui::RichText::new("Sections").strong());
        egui::ScrollArea::vertical()
            .max_height(200.0)
            .show(ui, |ui| {
                egui::Grid::new("elf_sections_grid")
                    .striped(true)
                    .min_col_width(60.0)
                    .show(ui, |ui| {
                        ui.label("Name");
                        ui.label("VMA");
                        ui.label("Size");
                        ui.label("Type");
                        ui.end_row();
                        let mut sections = info.sections.clone();
                        sections.sort_by_key(|s| s.size);
                        for s in sections.iter().rev() {
                            ui.label(&s.name);
                            ui.label(format!("0x{:08X}", s.vma));
                            ui.label(format!("{}", s.size));
                            ui.label(&s.section_type);
                            ui.end_row();
                        }
                    });
            });

        ui.add_space(8.0);
        ui.label(egui::RichText::new("Symbols").strong());
        ui.add_space(4.0);
        let mut filter: String = String::new();
        ui.horizontal(|ui| {
            ui.label("Filter:");
            ui.text_edit_singleline(&mut filter);
        });
        egui::ScrollArea::vertical().show(ui, |ui| {
            egui::Grid::new("elf_symbols_grid")
                .striped(true)
                .min_col_width(80.0)
                .show(ui, |ui| {
                    ui.label("Name");
                    ui.label("Address");
                    ui.label("Size");
                    ui.label("Type");
                    ui.end_row();
                    for sym in &info.symbols {
                        if !filter.is_empty() && !sym.name.contains(&filter) {
                            continue;
                        }
                        ui.label(&sym.name);
                        ui.label(format!("0x{:08X}", sym.address));
                        ui.label(format!("{}", sym.size));
                        ui.label(&sym.sym_type);
                        ui.end_row();
                    }
                });
        });
    } else {
        ui.colored_label(
            egui::Color32::YELLOW,
            "ビルド後に「Analyze ELF」をクリックしてください",
        );
    }
}

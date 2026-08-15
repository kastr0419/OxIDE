// SPDX-License-Identifier: MIT OR Apache-2.0
// Stack analysis UI

use crate::app::IdeApp;
use egui::Ui;

pub fn ui_stack_panel(app: &mut IdeApp, ui: &mut Ui) {
    ui.vertical(|ui| {
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new("Stack Analysis").strong());
            if ui.button("Analyze Stack").clicked() {
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
                            match crate::core::stack_analyzer::analyze_stack(&elf_path) {
                                Ok(report) => {
                                    let _ = tx
                                        .send(crate::core::event::CoreEvent::StackAnalysis(report));
                                }
                                Err(e) => {
                                    let _ = tx
                                        .send(crate::core::event::CoreEvent::Error(e.to_string()));
                                }
                            }
                        } else {
                            let _ = tx.send(crate::core::event::CoreEvent::Error(
                                "ELF not found".to_string(),
                            ));
                        }
                    } else {
                        let _ = tx.send(crate::core::event::CoreEvent::Error(
                            "No board selected".to_string(),
                        ));
                    }
                });
            }
        });

        ui.add_space(6.0);

        ui.horizontal(|ui| {
            ui.label("Filter:");
            ui.text_edit_singleline(&mut app.stack_filter);
        });

        ui.add_space(6.0);

        if let Some(report) = &app.stack_report {
            ui.label(format!(
                "Estimated total stack: {} bytes",
                report.total_estimate
            ));
            if !report.frames.is_empty() {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    for f in &report.frames {
                        if !app.stack_filter.is_empty() && !f.function.contains(&app.stack_filter) {
                            continue;
                        }
                        ui.horizontal(|ui| {
                            ui.label(f.function.to_string());
                            if let Some(s) = f.stack_usage {
                                ui.label(format!("{} bytes", s));
                            }
                        });
                    }
                });
            } else {
                ui.label("No frame details available.");
            }
            if !report.warnings.is_empty() {
                ui.separator();
                ui.label("Warnings:");
                for w in &report.warnings {
                    ui.colored_label(ui.visuals().warn_fg_color, format!("- {}", w));
                }
            }
        } else {
            ui.label("No stack report available. Run analysis after building.");
        }
    });
}

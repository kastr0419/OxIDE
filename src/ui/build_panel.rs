// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright 2026 ALLoIDE contributors

fn save_editor_to_disk(app: &mut crate::app::IdeApp) {
    if let Some(path) = &app.file_path {
        if let Err(e) = std::fs::write(path, &app.editor_text) {
            app.build_log = format!("[ERROR] ファイル保存失敗: {}", e);
        }
    }
}

fn open_dist_folder(path: &std::path::Path) {
    #[cfg(target_os = "windows")]
    let _ = std::process::Command::new("explorer").arg(path).spawn();
    #[cfg(target_os = "linux")]
    let _ = std::process::Command::new("xdg-open").arg(path).spawn();
    #[cfg(target_os = "macos")]
    let _ = std::process::Command::new("open").arg(path).spawn();
}

fn find_elf(dist: &std::path::Path) -> Option<std::path::PathBuf> {
    std::fs::read_dir(dist).ok()?.find_map(|entry| {
        let path = entry.ok()?.path();
        (path.extension()? == "elf").then_some(path)
    })
}

pub fn ui_build_panel(
    app: &mut crate::app::IdeApp,
    ui: &mut egui::Ui,
    tx: &crossbeam_channel::Sender<crate::core::event::CoreEvent>,
) {
    ui.horizontal(|ui| {
        let build_btn = ui.add_enabled(!app.is_building, egui::Button::new("▶ Build"));
        if build_btn.clicked() {
            save_editor_to_disk(app);
            app.is_building = true;
            let req = crate::core::compiler::BuildRequest {
                project_dir: app.config.workspace_dir.clone(),
                target_triple: crate::core::board::BOARD_PRESETS
                    .get(app.selected_board)
                    .map(|p| p.target_triple.to_string())
                    .unwrap_or_default(),
                release: false,
                board: crate::core::board::BOARD_PRESETS
                    .get(app.selected_board)
                    .map(|p| p.kind.clone()),
            };
            crate::core::compiler::build_async(req, tx.clone());
        }

        let flash_btn = ui.add_enabled(!app.is_flashing, egui::Button::new("⚡ Flash"));
        if flash_btn.clicked() {
            app.is_flashing = true;
            // last_dist_path からELFを検索してflash_asyncを呼ぶ
            let elf_path = app.last_dist_path.as_deref().and_then(find_elf);
            match elf_path {
                Some(artifact) => {
                    if let Some(preset) = crate::core::board::BOARD_PRESETS.get(app.selected_board)
                    {
                        let port = app
                            .available_ports
                            .get(app.selected_port)
                            .cloned()
                            .unwrap_or_default();
                        let flash_req = crate::core::flasher::FlashRequest {
                            board: preset.kind.clone(),
                            artifact,
                            port,
                        };
                        crate::core::flasher::flash_async(flash_req, tx.clone());
                    } else {
                        app.is_flashing = false;
                        app.build_log = "[ERROR] ボードが選択されていません".to_string();
                    }
                }
                None => {
                    app.is_flashing = false;
                    app.build_log =
                        "[ERROR] ビルド成果物が見つかりません。先に Build を実行してください。"
                            .to_string();
                }
            }
        }

        if ui.button("▶ Build & Flash").clicked() {
            save_editor_to_disk(app);
            app.is_building = true;
            app.auto_flash_after_build = true;
            let req = crate::core::compiler::BuildRequest {
                project_dir: app.config.workspace_dir.clone(),
                target_triple: crate::core::board::BOARD_PRESETS
                    .get(app.selected_board)
                    .map(|p| p.target_triple.to_string())
                    .unwrap_or_default(),
                release: false,
                board: crate::core::board::BOARD_PRESETS
                    .get(app.selected_board)
                    .map(|p| p.kind.clone()),
            };
            crate::core::compiler::build_async(req, tx.clone());
        }

        if app.is_building {
            ui.spinner();
        }
    });

    ui.horizontal(|ui| {
        if ui.button("📂 ELF").clicked() {
            app.show_elf_panel = true;
        }
        if ui.button("📊 Stack").clicked() {
            app.show_stack_panel = true;
        }

        let virtual_port = app
            .available_ports
            .get(app.selected_port)
            .map(String::as_str)
            == Some(crate::core::serial::VIRTUAL_PORT_NAME);
        if virtual_port {
            if let Some(preset) = crate::core::board::BOARD_PRESETS.get(app.selected_board) {
                match crate::core::simulator::support(&preset.kind) {
                    crate::core::simulator::SimulationSupport::Supported { .. } => {
                        let artifact = app.last_dist_path.as_deref().and_then(find_elf);
                        if let Some(artifact) = artifact {
                            if ui
                                .button("🧠 CPU/GPIO Sim")
                                .on_hover_text("Renode を使用して CPU と GPIO をシミュレーション")
                                .clicked()
                            {
                                let request = crate::core::simulator::SimulationRequest {
                                    board: preset.kind.clone(),
                                    artifact,
                                };
                                match crate::core::simulator::launch(&request) {
                                    Ok(script) => app.build_log.push_str(&format!(
                                        "\n[SIM] Renode を起動しました: {}\n",
                                        script.display()
                                    )),
                                    Err(error) => app.build_log.push_str(&format!(
                                        "\n[ERROR] CPU/GPIO simulation: {error}\n"
                                    )),
                                }
                            }
                        }
                    }
                    crate::core::simulator::SimulationSupport::Unsupported(reason) => {
                        ui.label("CPU/GPIO Sim: 非対応").on_hover_text(reason);
                    }
                }
            }
        }
    });

    ui.separator();

    // 成果物フォルダ
    if let Some(ref dist) = app.last_dist_path.clone() {
        ui.horizontal(|ui| {
            ui.label("📦 成果物:");
            ui.label(
                egui::RichText::new(dist.to_string_lossy())
                    .monospace()
                    .weak(),
            );
            if ui
                .button("📁 開く")
                .on_hover_text("エクスプローラーでフォルダを開く")
                .clicked()
            {
                open_dist_folder(dist);
            }
        });
        ui.label(
            egui::RichText::new("💡 Flash が上手くいかない場合はここのファイルをマイコンにドラッグ&ドロップしてください")
                .small()
                .color(egui::Color32::from_rgb(160, 160, 100)),
        );
        ui.separator();
    }

    ui.horizontal(|ui| {
        ui.label("Build Log:");
        if ui
            .button("📋 Copy")
            .on_hover_text("ビルドログをクリップボードにコピー")
            .clicked()
        {
            ui.ctx().copy_text(app.build_log.clone());
        }
    });
    egui::ScrollArea::vertical()
        .stick_to_bottom(true)
        .show(ui, |ui| {
            ui.add(
                egui::TextEdit::multiline(&mut app.build_log)
                    .desired_rows(10)
                    .interactive(false),
            );
        });

    // Flash/RAM usage meter
    if let Some(ref stats) = app.build_stats {
        ui.separator();
        ui.label(egui::RichText::new("📊 Build Size").strong());

        ui.horizontal(|ui| {
            ui.label("Flash:");
            let flash_color = if stats.flash_percent() > 0.9 {
                egui::Color32::RED
            } else if stats.flash_percent() > 0.75 {
                ui.visuals().warn_fg_color
            } else {
                egui::Color32::from_rgb(0, 180, 100)
            };
            let bar = egui::widgets::ProgressBar::new(stats.flash_percent())
                .desired_width(160.0)
                .fill(flash_color);
            ui.add(bar);
            ui.label(format!(
                "{} / {} B ({:.1}%)",
                stats.flash_used,
                stats.flash_total,
                stats.flash_percent() * 100.0
            ));
        });

        ui.horizontal(|ui| {
            ui.label("RAM:  ");
            let ram_color = if stats.ram_percent() > 0.9 {
                egui::Color32::RED
            } else if stats.ram_percent() > 0.75 {
                ui.visuals().warn_fg_color
            } else {
                egui::Color32::from_rgb(0, 120, 220)
            };
            let bar = egui::widgets::ProgressBar::new(stats.ram_percent())
                .desired_width(160.0)
                .fill(ram_color);
            ui.add(bar);
            ui.label(format!(
                "{} / {} B ({:.1}%)",
                stats.ram_used,
                stats.ram_total,
                stats.ram_percent() * 100.0
            ));
        });
    }
}

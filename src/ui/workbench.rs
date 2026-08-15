// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright 2026 ALLoIDE contributors

use crate::app::{write_or_log, IdeApp};

pub(crate) fn update(app: &mut IdeApp, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    // process incoming messages from core/background threads
    app.handle_messages();

    // Ctrl+S save shortcut
    if ctx.input(|i| i.modifiers.ctrl && i.key_pressed(egui::Key::S)) {
        if let Some(ref path) = app.file_path.clone() {
            if std::fs::write(path, &app.editor_text).is_ok() {
                app.is_dirty = false;
                let _ = app.config.save();
            }
        }
    }

    let msg_tx = app.msg_tx.clone();

    // Menu bar
    let mut settings_clicked = false;
    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("📁 New Project...").clicked() {
                    app.new_project_name = String::new();
                    app.new_project_base_dir = app
                        .config
                        .workspace_dir
                        .parent()
                        .unwrap_or(&app.config.workspace_dir)
                        .to_path_buf();
                    app.show_new_project_dialog = true;
                    ui.close_menu();
                }
                if ui.button("📂 Open Project...").clicked() {
                    if let Some(dir) = rfd::FileDialog::new().pick_folder() {
                        match crate::core::project::open_project(&dir) {
                            Ok(info) => {
                                app.config.workspace_dir = dir.clone();
                                app.project_name = info.project_name;
                                if let Some(board) = info.board {
                                    if let Some(idx) = crate::core::board::BOARD_PRESETS
                                        .iter()
                                        .position(|p| p.kind == board)
                                    {
                                        app.selected_board = idx;
                                    }
                                }
                                app.open_tabs.clear();
                                app.active_tab = 0;
                                app.start_or_restart_lsp();
                                let main_rs = dir.join("src").join("main.rs");
                                app.open_file_in_tab(main_rs);
                                app.refresh_workspace_files();
                                let _ = app.config.save();
                                app.build_log =
                                    format!("[OK] プロジェクトを開きました: {}", dir.display());
                            }
                            Err(e) => {
                                app.build_log =
                                    format!("[ERROR] プロジェクトを開けませんでした: {}", e);
                            }
                        }
                    }
                    ui.close_menu();
                }
                ui.separator();
                if ui.button("💾 Save  Ctrl+S").clicked() {
                    if let Some(ref path) = app.file_path.clone() {
                        if let Err(e) = std::fs::write(path, &app.editor_text) {
                            app.build_log = format!("[ERROR] 保存失敗: {}", e);
                        } else {
                            app.is_dirty = false;
                            let _ = app.config.save();
                            app.build_log = "[OK] 保存しました".to_string();
                        }
                    } else {
                        if let Some(path) = rfd::FileDialog::new()
                            .add_filter("Rust", &["rs"])
                            .save_file()
                        {
                            if let Err(e) = std::fs::write(&path, &app.editor_text) {
                                app.build_log = format!("[ERROR] 保存失敗: {}", e);
                            } else {
                                app.file_path = Some(path);
                                app.is_dirty = false;
                                let _ = app.config.save();
                                app.build_log = "[OK] 保存しました".to_string();
                            }
                        }
                    }
                    ui.close_menu();
                }
                if ui.button("💾 Save As...").clicked() {
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("Rust", &["rs"])
                        .save_file()
                    {
                        if let Err(e) = std::fs::write(&path, &app.editor_text) {
                            app.build_log = format!("[ERROR] 保存失敗: {}", e);
                        } else {
                            app.file_path = Some(path);
                            app.is_dirty = false;
                            app.build_log = "[OK] 名前を付けて保存しました".to_string();
                        }
                    }
                    ui.close_menu();
                }
            });
            ui.menu_button("Build", |ui| if ui.button("Build").clicked() {});
            ui.menu_button("Help", |ui| {
                let _ = ui.button("About");
                if ui.button("使い方ガイド").clicked() {
                    app.show_help_window = true;
                }
            });
            if ui.button("Settings").clicked() {
                settings_clicked = true;
            }
            if ui.button("📋 SVD").clicked() {
                app.show_svd_panel = !app.show_svd_panel;
            }
            ui.checkbox(&mut app.show_debug_panel, "Debug Panel");
        });
    });
    if settings_clicked {
        app.show_settings = !app.show_settings;
    }

    // Left: Board picker + Build panel
    egui::SidePanel::left("left_panel")
        .resizable(true)
        .default_width(crate::app::config::LEFT_PANEL_WIDTH)
        .width_range(100.0..=600.0)
        .show(ctx, |ui| {
            ui.heading("Board");
            crate::ui::board_picker::ui_board_picker(app, ui, &msg_tx);
            ui.separator();
            crate::ui::build_panel::ui_build_panel(app, ui, &msg_tx);
            ui.separator();
            crate::ui::file_explorer::ui_file_explorer(app, ui);
        });

    // Right: Serial monitor / Docs
    egui::SidePanel::right("right_panel")
        .resizable(true)
        .default_width(crate::app::config::RIGHT_PANEL_WIDTH)
        .width_range(100.0..=600.0)
        .show(ctx, |ui| {
            // Tab buttons
            ui.horizontal_wrapped(|ui| {
                ui.selectable_value(
                    &mut app.right_tab,
                    crate::ui::help_panel::RightTab::SerialMonitor,
                    "🔌 Serial",
                );
                ui.selectable_value(
                    &mut app.right_tab,
                    crate::ui::help_panel::RightTab::Agent,
                    "🤖 Agent",
                );
                ui.selectable_value(
                    &mut app.right_tab,
                    crate::ui::help_panel::RightTab::Docs,
                    "📖 Docs",
                );
                ui.selectable_value(
                    &mut app.right_tab,
                    crate::ui::help_panel::RightTab::Pinout,
                    "📌 ピンアウト",
                );
                ui.selectable_value(
                    &mut app.right_tab,
                    crate::ui::help_panel::RightTab::VirtualBoard,
                    "🧪 Board",
                );
            });
            ui.separator();
            match app.right_tab {
                crate::ui::help_panel::RightTab::SerialMonitor => {
                    crate::ui::serial_monitor::ui_serial_monitor(app, ui, &msg_tx);
                }
                crate::ui::help_panel::RightTab::Agent => {
                    crate::ui::agent_panel::ui_agent_panel(app, ui, &msg_tx);
                }
                crate::ui::help_panel::RightTab::Docs => {
                    crate::ui::help_panel::ui_help_panel(app, ui);
                }
                crate::ui::help_panel::RightTab::Pinout => {
                    crate::ui::pinout_panel::ui_pinout_panel(app, ui);
                }
                crate::ui::help_panel::RightTab::VirtualBoard => {
                    crate::ui::virtual_board_panel::ui_virtual_board_panel(app, ui);
                }
            }
        });

    if app.show_debug_panel {
        egui::SidePanel::right("debug_panel")
            .min_width(280.0)
            .default_width(300.0)
            .show(ctx, |ui| {
                ui.heading("🐛 Debug");
                ui.separator();
                crate::ui::debug_panel::ui_debug_panel(app, ui);
            });
    }

    if app.show_svd_panel {
        egui::SidePanel::right("svd_panel")
            .resizable(true)
            .min_width(280.0)
            .default_width(320.0)
            .show(ctx, |ui| {
                ui.heading("📋 SVD Viewer");
                ui.separator();
                crate::ui::svd_panel::ui_svd_panel(app, ui);
            });
    }

    // Center: Editor
    egui::CentralPanel::default().show(ctx, |ui| {
        crate::ui::editor::ui_editor(app, ui, &msg_tx);
    });

    // Bottom: Status bar
    let mut quit_clicked = false;
    egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
        ui.horizontal(|ui| {
            if let Some(ref name) = app.project_name {
                ui.label(format!("📁 {}", name));
                ui.separator();
            }
            ui.label(format!(
                "Board: {}",
                crate::core::board::BOARD_PRESETS
                    .get(app.selected_board)
                    .map(|p| p.display_name)
                    .unwrap_or("<unknown>")
            ));
            ui.separator();
            ui.label(format!("Ln {}, Col {}", app.cursor_line, app.cursor_col));
            if app.is_dirty {
                ui.label("●");
            }
            ui.separator();
            // LSP ステータス
            let lsp_text = if app.lsp_client.is_some() {
                if app.lsp_initialized {
                    "LSP ●"
                } else {
                    "LSP…"
                }
            } else {
                "LSP ✕"
            };
            let lsp_color = if app.lsp_client.is_some() {
                if app.lsp_initialized {
                    egui::Color32::from_rgb(80, 200, 80)
                } else {
                    ui.visuals().warn_fg_color
                }
            } else {
                egui::Color32::from_rgb(200, 80, 80)
            };
            ui.label(egui::RichText::new(lsp_text).color(lsp_color));
            ui.separator();
            if app.is_building {
                ui.label("Building...");
            }
            if app.is_flashing {
                ui.label("Flashing...");
            }
            if ui.button("Quit").clicked() {
                quit_clicked = true;
            }
        });
    });
    if quit_clicked {
        // graceful exit - left as no-op for now
    }

    if app.show_settings {
        let mut show = app.show_settings;
        egui::Window::new("Settings")
            .open(&mut show)
            .min_width(420.0)
            .show(ctx, |ui| {
                crate::ui::settings::ui_settings(app, ui);
            });
        app.show_settings = show;
    }

    if app.show_help_window {
        let mut show = app.show_help_window;
        egui::Window::new("📖 使い方ガイド")
            .open(&mut show)
            .resizable(true)
            .default_size([800.0, 600.0])
            .show(ctx, |ui| {
                crate::ui::help_panel::ui_help_panel(app, ui);
            });
        app.show_help_window = show;
    }

    // ELF Viewer window
    if app.show_elf_panel {
        let mut show = app.show_elf_panel;
        egui::Window::new("📂 ELF Viewer")
            .open(&mut show)
            .default_size([700.0, 500.0])
            .show(ctx, |ui| {
                crate::ui::elf_panel::ui_elf_panel(app, ui);
            });
        app.show_elf_panel = show;
    }

    // Stack Analysis window
    if app.show_stack_panel {
        let mut show = app.show_stack_panel;
        egui::Window::new("📊 Stack Analysis")
            .open(&mut show)
            .default_size([700.0, 500.0])
            .show(ctx, |ui| {
                crate::ui::stack_panel::ui_stack_panel(app, ui);
            });
        app.show_stack_panel = show;
    }

    // テンプレート読み込み確認ダイアログ
    if let Some(board_idx) = app.template_confirm_board {
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
                                    let ws = app.config.workspace_dir.clone();
                                    // IDE 本体のソースディレクトリへの上書きを防ぐ（簡易チェック）
                                    if ws.join("src").join("lib.rs").exists() {
                                        app.build_log = "[ERROR] ワークスペースが IDE 本体のディレクトリです。Settings からワークスペースを変更してください。".to_string();
                                    } else {
                                        match crate::templates::create_blink_project(&ws, &preset.kind) {
                                            Ok(_) => {
                                                let main_rs_path = ws.join("src").join("main.rs");
                                                app.open_tabs.clear();
                                                app.active_tab = 0;
                                                app.open_file_in_tab(main_rs_path);
                                                app.refresh_workspace_files();
                                                app.is_dirty = false;
                                            }
                                            Err(e) => {
                                                app.build_log = format!("[ERROR] テンプレートの書き出しに失敗: {}", e);
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
            app.template_confirm_board = None;
        }
    }

    // 新規プロジェクトダイアログ
    if app.show_new_project_dialog {
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
                        ui.text_edit_singleline(&mut app.new_project_name);
                        ui.end_row();
                        ui.label("保存先:");
                        ui.horizontal(|ui| {
                            let dir_str = app.new_project_base_dir.to_string_lossy();
                            ui.label(egui::RichText::new(dir_str.as_ref()).monospace().small());
                            if ui.small_button("変更...").clicked() {
                                if let Some(dir) = rfd::FileDialog::new().pick_folder() {
                                    app.new_project_base_dir = dir;
                                }
                            }
                        });
                        ui.end_row();
                        ui.label("ボード:");
                        ui.label(
                            crate::core::board::BOARD_PRESETS
                                .get(app.selected_board)
                                .map(|p| p.display_name)
                                .unwrap_or("<unknown>"),
                        );
                        ui.end_row();
                        ui.label("作成先:");
                        let project_dir =
                            app.new_project_base_dir.join(app.new_project_name.trim());
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
                    let can_create = !app.new_project_name.trim().is_empty();
                    if ui
                        .add_enabled(can_create, egui::Button::new("✅ 作成"))
                        .clicked()
                    {
                        do_create = true;
                        close = true;
                    }
                    ui.add_space(8.0);
                    if ui.button("❌ キャンセル").clicked() {
                        close = true;
                    }
                });
            });

        if do_create && !app.new_project_name.trim().is_empty() {
            let project_dir = app.new_project_base_dir.join(app.new_project_name.trim());
            if project_dir.join("src").join("lib.rs").exists() {
                app.build_log =
                    "[ERROR] そのパスはIDE本体のディレクトリです。別の場所を選択してください。"
                        .to_string();
            } else if let Some(preset) = crate::core::board::BOARD_PRESETS.get(app.selected_board) {
                match crate::templates::create_blink_project(&project_dir, &preset.kind) {
                    Ok(_) => {
                        let main_rs_path = project_dir.join("src").join("main.rs");
                        app.config.workspace_dir = project_dir.clone();
                        app.project_name = Some(app.new_project_name.trim().to_string());
                        app.open_tabs.clear();
                        app.active_tab = 0;
                        app.open_file_in_tab(main_rs_path);
                        app.refresh_workspace_files();
                        app.is_dirty = false;
                        app.build_log = format!(
                            "[OK] プロジェクト「{}」を作成しました: {}",
                            app.new_project_name.trim(),
                            project_dir.display()
                        );
                        let _ = app.config.save();
                    }
                    Err(e) => {
                        app.build_log = format!("[ERROR] プロジェクト作成失敗: {}", e);
                    }
                }
            }
        }
        if close {
            app.show_new_project_dialog = false;
        }
    }

    // 新規ファイル作成ダイアログ
    if app.show_new_file_dialog {
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
                    ui.text_edit_singleline(&mut app.new_file_name);
                });
                ui.label(
                    egui::RichText::new("例: led.rs  sensors.rs  config.rs")
                        .small()
                        .weak(),
                );
                ui.add_space(6.0);
                ui.horizontal(|ui| {
                    let name = app.new_file_name.trim();
                    let can_create = !name.is_empty() && name.ends_with(".rs");
                    if ui
                        .add_enabled(can_create, egui::Button::new("✅ 作成"))
                        .clicked()
                    {
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
            let name = app.new_file_name.trim().to_string();
            let src_dir = app.config.workspace_dir.join("src");
            let path = src_dir.join(&name);
            if std::fs::create_dir_all(&src_dir).is_ok() {
                if !path.exists() {
                    write_or_log(&path, &format!("// {}\n", name), &mut app.build_log);
                }
                app.open_file_in_tab(path);
                app.refresh_workspace_files();
            }
        }
        if close_file_dialog {
            app.show_new_file_dialog = false;
        }
    }

    // request repaint for async updates
    ctx.request_repaint();
}

// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright 2026 rust-embedded-ide contributors

pub fn ui_file_explorer(app: &mut crate::app::IdeApp, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new("📁 Files").strong());
        if ui.small_button("🔄").on_hover_text("ファイル一覧を更新").clicked() {
            app.refresh_workspace_files();
        }
        if ui.small_button("➕").on_hover_text("新規 .rs ファイル作成").clicked() {
            app.new_file_name = String::new();
            app.show_new_file_dialog = true;
        }
    });

    // ワークスペースパス表示（短縮）
    let ws_str = app.config.workspace_dir.to_string_lossy();
    ui.label(
        egui::RichText::new(ws_str.as_ref())
            .small()
            .weak()
            .monospace(),
    );

    ui.separator();

    let workspace = app.config.workspace_dir.clone();
    let files = app.workspace_files.clone();
    let active_path = app.file_path.clone();

    egui::ScrollArea::vertical()
        .id_salt("file_explorer_scroll")
        .max_height(180.0)
        .show(ui, |ui| {
            if files.is_empty() {
                ui.label(
                    egui::RichText::new("（ファイルなし）").small().weak(),
                );
                return;
            }
            let mut delete_target: Option<std::path::PathBuf> = None;
            for path in &files {
                let rel = path.strip_prefix(&workspace).unwrap_or(path);
                let is_active = active_path.as_ref() == Some(path);

                let name = rel.to_string_lossy();
                let icon = if name.ends_with(".rs") { "📄" } else if name.ends_with(".toml") { "⚙" } else { "📃" };
                let display = format!("{} {}", icon, name);

                let resp = ui.selectable_label(
                    is_active,
                    egui::RichText::new(&display).small().monospace(),
                );
                if resp.clicked() {
                    app.open_file_in_tab(path.clone());
                }
                resp.context_menu(|ui| {
                    if ui.button("🗑 削除").clicked() {
                        delete_target = Some(path.clone());
                        ui.close_menu();
                    }
                });
            }
            if let Some(target) = delete_target {
                let _ = std::fs::remove_file(&target);
                app.refresh_workspace_files();
                // 削除したファイルのタブがあれば閉じる
                if let Some(idx) = app.open_tabs.iter().position(|t| t.path == target) {
                    app.open_tabs.remove(idx);
                    if app.active_tab >= app.open_tabs.len() && !app.open_tabs.is_empty() {
                        app.active_tab = app.open_tabs.len() - 1;
                    }
                    if app.open_tabs.is_empty() {
                        app.editor_text = String::new();
                        app.file_path = None;
                        app.is_dirty = false;
                    } else {
                        if let Some(tab) = app.open_tabs.get(app.active_tab).cloned() {
                            app.editor_text = tab.content;
                            app.file_path = Some(tab.path);
                            app.is_dirty = tab.is_dirty;
                        } else {
                            app.editor_text = String::new();
                            app.file_path = None;
                            app.is_dirty = false;
                        }
                    }
                }
            }
        });
}

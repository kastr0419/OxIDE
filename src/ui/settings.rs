// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright 2026 rust-embedded-ide contributors

pub fn ui_settings(app: &mut crate::app::IdeApp, ui: &mut egui::Ui) {
    // ─ ワークスペース ─
    ui.heading("Workspace");
    ui.horizontal(|ui| {
        ui.label("Directory:");
        if ui.button("Choose...").clicked() {
            if let Some(d) = rfd::FileDialog::new().pick_folder() {
                app.config.workspace_dir = d;
            }
        }
        ui.label(app.config.workspace_dir.to_string_lossy().as_ref());
    });

    // ─ テーマ ─
    ui.separator();
    ui.heading("Appearance");
    let mut dark = app.config.theme.as_deref() == Some("dark");
    if ui.checkbox(&mut dark, "Dark Theme").changed() {
        app.config.theme = if dark {
            Some("dark".to_string())
        } else {
            Some("light".to_string())
        };
    }

    // ─ rust-analyzer ─
    ui.separator();
    ui.heading("rust-analyzer (LSP)");

    // インストール状態の表示
    if app.ra_status.is_installed {
        let path_str = app
            .ra_status
            .path
            .as_ref()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_default();
        ui.label(
            egui::RichText::new(format!("✅ Installed: {}", path_str)).color(egui::Color32::GREEN),
        );
        if let Some(ref ver) = app.ra_status.version {
            ui.label(egui::RichText::new(ver).small().color(egui::Color32::GRAY));
        }
    } else {
        ui.label(
            egui::RichText::new("❌ Not installed / not found in PATH").color(egui::Color32::RED),
        );
    }

    ui.horizontal(|ui| {
        // rustup でインストール
        ui.add_enabled_ui(!app.ra_installing, |ui| {
            if ui
                .button("⬇ Install via rustup")
                .on_hover_text("rustup component add rust-analyzer を実行してインストール")
                .clicked()
            {
                crate::core::toolchain::install_rust_analyzer_async(app.msg_tx.clone());
            }
        });

        // 状態を再チェック
        if ui.button("🔄 Re-check").clicked() {
            app.ra_status = crate::core::toolchain::check_rust_analyzer();
            app.ra_install_log.clear();
        }

        // 手動でパスを指定
        if ui
            .button("📂 Locate manually")
            .on_hover_text("rust-analyzer の実行ファイルを手動で指定してパスを保存")
            .clicked()
        {
            if let Some(path) = rfd::FileDialog::new()
                .set_title("rust-analyzer を選択")
                .pick_file()
            {
                if crate::core::toolchain::validate_custom_path(&path) {
                    app.config.rust_analyzer_path = Some(path.clone());
                    app.ra_status = crate::core::toolchain::RustAnalyzerStatus {
                        is_installed: true,
                        path: Some(path),
                        version: None,
                    };
                    let _ = app.config.save();
                } else {
                    app.ra_install_log =
                        "❌ 無効なパスです（実行可能ファイルではありません）".to_string();
                }
            }
        }
    });

    // インストール中スピナー
    if app.ra_installing {
        ui.horizontal(|ui| {
            ui.spinner();
            ui.label("Installing...");
        });
    }

    // インストールログ
    if !app.ra_install_log.is_empty() {
        ui.label(egui::RichText::new(&app.ra_install_log).small());
    }

    // PATH ガイダンス（インストール済みでも PATH が通っていない場合）
    if app.ra_status.is_installed {
        if let Some(ref p) = app.ra_status.path {
            if let Some(dir) = p.parent() {
                let dir_str = dir.to_string_lossy();
                if !std::env::var("PATH")
                    .unwrap_or_default()
                    .contains(dir_str.as_ref())
                {
                    ui.separator();
                    ui.label(
                        egui::RichText::new("⚠ PATHが通っていない可能性があります")
                            .color(egui::Color32::YELLOW)
                            .small(),
                    );
                    ui.label(
                        egui::RichText::new(format!("以下をPATHに追加してください: {}", dir_str))
                            .small()
                            .color(egui::Color32::GRAY),
                    );
                }
            }
        }
    }

    ui.separator();
    if ui.button("💾 Save Settings").clicked() {
        let _ = app.config.save();
    }
}

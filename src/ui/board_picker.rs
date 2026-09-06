// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright 2026 ALLoIDE contributors

use egui::ComboBox;

pub fn ui_board_picker(
    app: &mut crate::app::IdeApp,
    ui: &mut egui::Ui,
    tx: &crossbeam_channel::Sender<crate::core::event::CoreEvent>,
) {
    // Board selection
    let prev_board = app.selected_board;
    ComboBox::from_label("Board")
        .selected_text(app.selected_board_preset().display_name)
        .show_ui(ui, |ui| {
            for (i, p) in crate::core::board::BOARD_PRESETS.iter().enumerate() {
                if ui
                    .selectable_label(app.selected_board == i, p.display_name)
                    .clicked()
                {
                    app.selected_board = i;
                }
            }
        });

    // ボードが変わった && テンプレートが存在する場合、確認ダイアログを表示
    if app.selected_board != prev_board {
        let has_template =
            crate::templates::blink::get_blink_template(&app.selected_board_preset().kind)
                .is_some();
        if has_template {
            app.template_confirm_board = Some(app.selected_board);
        }
    }

    ui.label("Target:");
    ui.label(app.selected_board_preset().target_triple);

    ui.separator();
    // Ports
    ComboBox::from_label("Port")
        .selected_text(
            app.available_ports
                .get(app.selected_port)
                .cloned()
                .unwrap_or_default(),
        )
        .show_ui(ui, |ui| {
            for (i, port) in app.available_ports.iter().enumerate() {
                if ui.selectable_label(app.selected_port == i, port).clicked() {
                    app.selected_port = i;
                }
            }
        });
    if app
        .available_ports
        .get(app.selected_port)
        .map(String::as_str)
        == Some(crate::core::serial::VIRTUAL_PORT_NAME)
    {
        ui.colored_label(
            ui.visuals().warn_fg_color,
            "仮想環境（実機への書き込みなし）",
        );
    }
    if ui.button("🔄 Refresh Ports").clicked() {
        app.available_ports = crate::core::serial::list_ports().unwrap_or_default();
        app.selected_port = 0;
    }

    ui.separator();
    if ui.button("🔍 Auto Detect").clicked() {
        app.detection_result = Some("Detecting...".to_string());
        crate::core::detector::auto_detect(tx.clone());
    }

    // テンプレート読み込みボタン
    let has_template =
        crate::templates::blink::get_blink_template(&app.selected_board_preset().kind).is_some();
    ui.add_enabled_ui(has_template, |ui| {
        if ui
            .button("📄 Load Template")
            .on_hover_text("現在のボード用Lチカテンプレートをエディタに読み込む")
            .clicked()
        {
            app.template_confirm_board = Some(app.selected_board);
        }
    });
    if !has_template {
        ui.label(
            egui::RichText::new("(このボードはテンプレート未対応)")
                .small()
                .color(egui::Color32::DARK_GRAY),
        );
    }

    // 検出結果の表示
    if let Some(ref msg) = app.detection_result {
        ui.colored_label(
            if msg.contains("No board") || msg.contains("Detecting") {
                ui.visuals().warn_fg_color
            } else {
                egui::Color32::GREEN
            },
            msg,
        );
    }
}

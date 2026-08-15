// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright 2026 ALLoIDE contributors

use crate::core::simulator::VirtualFlashState;
use egui::{Align2, Color32, FontId, Sense, Stroke, Vec2};

pub fn ui_virtual_board_panel(app: &crate::app::IdeApp, ui: &mut egui::Ui) {
    let is_virtual = app
        .available_ports
        .get(app.selected_port)
        .map(String::as_str)
        == Some(crate::core::serial::VIRTUAL_PORT_NAME);

    if !is_virtual {
        ui.vertical_centered(|ui| {
            ui.add_space(24.0);
            ui.heading("仮想ボードは未選択です");
            ui.label("Port から「ALLoIDE Virtual Board」を選択してください。");
        });
        return;
    }

    ui.heading("ALLoIDE Virtual Board");
    ui.label(app.selected_board_preset().display_name);
    ui.add_space(6.0);

    let width = ui.available_width().clamp(240.0, 440.0);
    let (rect, _) = ui.allocate_exact_size(Vec2::new(width, 190.0), Sense::hover());
    let painter = ui.painter();
    painter.rect_filled(rect, 12.0, Color32::from_rgb(30, 64, 56));
    painter.rect_stroke(
        rect,
        12.0,
        Stroke::new(2.0_f32, Color32::from_rgb(68, 130, 108)),
        egui::StrokeKind::Inside,
    );

    painter.text(
        rect.center_top() + egui::vec2(0.0, 20.0),
        Align2::CENTER_CENTER,
        "ALLoIDE",
        FontId::proportional(22.0),
        Color32::WHITE,
    );

    let flash_color = match app.virtual_board.flash {
        VirtualFlashState::Empty => Color32::from_gray(70),
        VirtualFlashState::Flashing => Color32::YELLOW,
        VirtualFlashState::Ready => Color32::GREEN,
        VirtualFlashState::Failed => Color32::RED,
    };
    let leds = [
        ("PWR", Color32::GREEN),
        ("FW", flash_color),
        (
            "RX",
            if app.virtual_board.activity_led {
                Color32::LIGHT_BLUE
            } else {
                Color32::from_gray(70)
            },
        ),
    ];
    for (index, (label, color)) in leds.into_iter().enumerate() {
        let center = egui::pos2(
            rect.left() + rect.width() * (index as f32 + 1.0) / 4.0,
            rect.center().y,
        );
        painter.circle_filled(center, 13.0, color);
        painter.circle_stroke(center, 13.0, Stroke::new(1.5_f32, Color32::BLACK));
        painter.text(
            center + egui::vec2(0.0, 27.0),
            Align2::CENTER_CENTER,
            label,
            FontId::monospace(13.0),
            Color32::WHITE,
        );
    }

    painter.text(
        rect.center_bottom() - egui::vec2(0.0, 16.0),
        Align2::CENTER_CENTER,
        "BUILT-IN VIRTUAL I/O",
        FontId::monospace(11.0),
        Color32::from_gray(180),
    );

    ui.add_space(10.0);
    egui::Grid::new("virtual_board_status")
        .num_columns(2)
        .spacing([10.0, 6.0])
        .show(ui, |ui| {
            ui.label("Flash");
            ui.strong(match app.virtual_board.flash {
                VirtualFlashState::Empty => "Empty",
                VirtualFlashState::Flashing => "Flashing…",
                VirtualFlashState::Ready => "Ready",
                VirtualFlashState::Failed => "Failed",
            });
            ui.end_row();

            ui.label("Serial");
            ui.strong(if app.virtual_board.serial_connected {
                "Connected"
            } else {
                "Disconnected"
            });
            ui.end_row();
        });

    ui.add_space(8.0);
    let sensor = app.virtual_board.sensor_value;
    ui.label("Sensor");
    ui.add(
        egui::ProgressBar::new(sensor.unwrap_or(0) as f32 / 99.0)
            .show_percentage()
            .text(sensor.map_or_else(|| "No data".to_string(), |value| value.to_string())),
    );

    ui.add_space(8.0);
    ui.label("Last serial line");
    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.set_min_width(ui.available_width());
        ui.monospace(app.virtual_board.last_serial.as_deref().unwrap_or("—"));
    });

    ui.add_space(10.0);
    if !app.virtual_board.serial_connected {
        ui.label("Serial タブで Connect すると RX と Sensor が動きます。");
    }
    ui.label(
        egui::RichText::new(
            "RX と Sensor は内蔵の仮想シリアル動作を表示します。実際の CPU / GPIO 実行には Renode が必要です。",
        )
        .small()
        .weak(),
    );
}

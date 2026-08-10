// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright 2026 rust-embedded-ide contributors

use crate::core::serial::SerialSettings;

pub fn ui_serial_monitor(
    app: &mut crate::app::IdeApp,
    ui: &mut egui::Ui,
    tx: &crossbeam_channel::Sender<crate::app::AppMessage>,
) {
    ui.horizontal(|ui| {
        if app.is_serial_connected {
            if ui.button("Disconnect").clicked() {
                if let Some(s) = &app.serial_tx {
                    let _ = s.send(crate::core::serial::SerialCommand::Disconnect);
                }
            }
        } else {
            if ui.button("Connect").clicked() {
                let port = app
                    .available_ports
                    .get(app.selected_port)
                    .cloned()
                    .unwrap_or_default();
                let baud = crate::core::board::BOARD_PRESETS
                    .get(app.selected_board)
                    .map(|p| p.default_baud)
                    .unwrap_or(crate::core::config::DEFAULT_BAUD_RATE);
                let settings = SerialSettings {
                    port_name: port.clone(),
                    baud_rate: baud,
                };
                let (cmd_tx, cmd_rx) = crossbeam_channel::unbounded();
                // store tx to allow sending
                app.serial_tx = Some(cmd_tx.clone());
                // spawn connect
                crate::core::serial::connect_async(settings, tx.clone(), cmd_rx);
            }
        }

        ui.separator();
        ui.label("Baud:");
        egui::ComboBox::from_id_salt("baud_cb")
            .selected_text("Baud")
            .show_ui(ui, |ui| {
                for b in [9600u32, 19200, 38400, 57600, 115200, 230400].iter() {
                    if ui.selectable_label(false, b.to_string()).clicked() {
                        // change desired baud for next connect
                    }
                }
            });

        if ui.button("🗑 Clear").clicked() {
            app.serial_log.clear();
        }
    });

    // Tab switcher: Monitor | Plotter
    ui.horizontal(|ui| {
        if ui
            .selectable_label(!app.show_plotter_tab, "📟 Monitor")
            .clicked()
        {
            app.show_plotter_tab = false;
        }
        if ui
            .selectable_label(app.show_plotter_tab, "📈 Plotter")
            .clicked()
        {
            app.show_plotter_tab = true;
        }
    });
    ui.separator();

    if app.show_plotter_tab {
        crate::ui::serial_plotter::ui_serial_plotter(app, ui);
        return;
    }

    egui::ScrollArea::vertical()
        .stick_to_bottom(true)
        .show(ui, |ui| {
            for line in app.serial_log.iter() {
                ui.label(line);
            }
        });

    ui.horizontal(|ui| {
        let resp = ui.add(egui::TextEdit::singleline(&mut app.serial_input));
        if (resp.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)))
            || ui.button("Send").clicked()
        {
            if let Some(tx_cmd) = &app.serial_tx {
                let _ = tx_cmd.send(crate::core::serial::SerialCommand::Send(
                    app.serial_input.clone(),
                ));
            }
            app.serial_input.clear();
        }
    });
}

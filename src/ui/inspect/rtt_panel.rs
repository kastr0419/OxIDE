// SPDX-License-Identifier: MIT OR Apache-2.0
// RTT panel UI skeleton

use crate::app::IdeApp;
use egui::Ui;

pub fn ui_rtt_panel(app: &mut IdeApp, ui: &mut Ui) {
    use crate::core::debugger::DebugCommand;

    ui.horizontal(|ui| {
        ui.label(egui::RichText::new("RTT Log").strong());
        ui.separator();
        ui.label("Channel:");
        egui::ComboBox::from_id_salt("rtt_channel_cb")
            .selected_text(format!("{}", app.rtt_channel))
            .show_ui(ui, |ui| {
                for ch in 0..3u32 {
                    ui.selectable_value(&mut app.rtt_channel, ch, format!("{}", ch));
                }
            });

        ui.separator();
        if app.debug_connected {
            if !app.rtt_running {
                if ui.button("Start RTT").clicked() {
                    if let Some(ref tx) = app.debug_cmd_tx {
                        let _ = tx.send(DebugCommand::StartRtt {
                            channel: app.rtt_channel,
                        });
                        app.rtt_running = true;
                    }
                }
            } else {
                if ui.button("Stop RTT").clicked() {
                    if let Some(ref tx) = app.debug_cmd_tx {
                        let _ = tx.send(DebugCommand::StopRtt);
                        app.rtt_running = false;
                    }
                }
            }
        } else {
            ui.add_enabled(false, egui::Button::new("Start RTT"));
        }

        if ui.button("Clear").clicked() {
            app.rtt_log.clear();
        }
    });

    ui.add_space(6.0);

    if !app.debug_connected {
        ui.colored_label(
            ui.visuals().warn_fg_color,
            "デバッガに接続してからRTTを開始してください",
        );
        return;
    }

    egui::ScrollArea::vertical()
        .stick_to_bottom(true)
        .show(ui, |ui| {
            ui.vertical(|ui| {
                for (ch, line) in &app.rtt_log {
                    ui.label(format!("[ch{}] {}", ch, line));
                }
            });
        });
}

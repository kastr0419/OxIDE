// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright 2026 ALLoIDE contributors

pub fn ui_serial_plotter(app: &mut crate::app::IdeApp, ui: &mut egui::Ui) {
    // Controls
    ui.horizontal(|ui| {
        let pause_label = if app.plot_paused {
            "▶ Resume"
        } else {
            "⏸ Pause"
        };
        if ui.button(pause_label).clicked() {
            app.plot_paused = !app.plot_paused;
        }
        if ui.button("🗑 Clear").clicked() {
            app.plot_channels.clear();
        }
        ui.label("Max points:");
        let mut max_pts = app.plot_max_points as f32;
        if ui
            .add(egui::Slider::new(&mut max_pts, 50.0..=1000.0).integer())
            .changed()
        {
            app.plot_max_points = max_pts as usize;
        }
    });

    if app.plot_channels.is_empty() {
        ui.label(egui::RichText::new(
            "📡 Waiting for data...\nSend comma-separated values:\n  temp:23.5,humidity:65\n  or: 23.5,65"
        ).small().color(egui::Color32::GRAY));
        return;
    }

    // Plot
    use egui_plot::{Line, Plot, PlotPoints};

    Plot::new("serial_plot")
        .height(ui.available_height() - 10.0)
        .legend(egui_plot::Legend::default())
        .show(ui, |plot_ui| {
            for (name, channel) in &app.plot_channels {
                let points: PlotPoints = channel
                    .values
                    .iter()
                    .enumerate()
                    .map(|(i, &v)| [i as f64, v])
                    .collect();
                let line = Line::new(points)
                    .name(name)
                    .color(channel.color)
                    .width(1.5_f32);
                plot_ui.line(line);
            }
        });
}

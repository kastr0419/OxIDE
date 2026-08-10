// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright 2026 rust-embedded-ide contributors

#![windows_subsystem = "windows"]

mod app;
mod core;
mod templates;
mod ui;

fn main() {
    use crate::core::config::{WINDOW_HEIGHT, WINDOW_WIDTH};
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([WINDOW_WIDTH, WINDOW_HEIGHT])
            .with_title("OxIDE"),
        ..Default::default()
    };
    if let Err(e) = eframe::run_native(
        "OxIDE",
        options,
        Box::new(|cc| Ok(Box::new(app::IdeApp::new(cc)))),
    ) {
        eprintln!("Failed to run application: {}", e);
    }
}

// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright 2026 rust-embedded-ide contributors

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod core;
mod app;
mod ui;
mod templates;

fn main() {
    use crate::core::config::{WINDOW_WIDTH, WINDOW_HEIGHT};
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([WINDOW_WIDTH, WINDOW_HEIGHT])
            .with_title("Rust Embedded IDE"),
        ..Default::default()
    };
    if let Err(e) = eframe::run_native("Rust Embedded IDE", options,
        Box::new(|cc| Ok(Box::new(app::IdeApp::new(cc))))) {
        eprintln!("Failed to run application: {}", e);
    }
}

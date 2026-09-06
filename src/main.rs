// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright 2026 ALLoIDE contributors

#![windows_subsystem = "windows"]

fn main() {
    use alloide::app::{
        config::{WINDOW_HEIGHT, WINDOW_WIDTH},
        IdeApp,
    };
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([WINDOW_WIDTH, WINDOW_HEIGHT])
            .with_title("ALLoIDE"),
        ..Default::default()
    };
    if let Err(e) = eframe::run_native(
        "ALLoIDE",
        options,
        Box::new(|cc| Ok(Box::new(IdeApp::new(cc)))),
    ) {
        eprintln!("Failed to run application: {}", e);
    }
}

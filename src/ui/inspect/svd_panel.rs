// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright 2026 rust-embedded-ide contributors

pub fn ui_svd_panel(app: &mut crate::app::IdeApp, ui: &mut egui::Ui) {
    // Load button
    ui.horizontal(|ui| {
        if ui.button("📂 Load SVD").clicked() {
            if let Some(path) = rfd::FileDialog::new()
                .add_filter("SVD", &["svd", "SVD"])
                .pick_file()
            {
                match std::fs::read_to_string(&path) {
                    Ok(xml) => {
                        match crate::core::svd_parser::parse_svd(&xml) {
                            Ok(device) => {
                                app.svd_device = Some(device);
                                app.svd_expanded_peripherals.clear();
                            }
                            Err(e) => {
                                // show error in status or log
                                eprintln!("SVD parse error: {}", e);
                            }
                        }
                    }
                    Err(e) => eprintln!("SVD read error: {}", e),
                }
            }
        }
        if app.svd_device.is_some() && ui.button("✕ Clear").clicked() {
            app.svd_device = None;
        }
    });

    // Search
    ui.horizontal(|ui| {
        ui.label("🔍");
        ui.text_edit_singleline(&mut app.svd_search);
    });
    ui.separator();

    let search = app.svd_search.to_lowercase();

    match &app.svd_device {
        None => {
            ui.label(egui::RichText::new(
                "Load a .svd file to browse peripheral registers.\nSVD files are available from chip vendors (e.g., STMicroelectronics, Nordic)."
            ).small().color(egui::Color32::GRAY));
        }
        Some(device) => {
            ui.label(
                egui::RichText::new(format!("📦 {}", device.name))
                    .strong()
                    .small(),
            );
            ui.separator();

            egui::ScrollArea::vertical().show(ui, |ui| {
                // Clone to avoid borrow conflict
                let peripherals = device.peripherals.clone();
                for periph in &peripherals {
                    // Filter by search
                    if !search.is_empty()
                        && !periph.name.to_lowercase().contains(&search)
                        && !periph.description.to_lowercase().contains(&search)
                        && !periph
                            .registers
                            .iter()
                            .any(|r| r.name.to_lowercase().contains(&search))
                    {
                        continue;
                    }

                    let is_expanded = app.svd_expanded_peripherals.contains(&periph.name);
                    let header = format!("⚡ {}  0x{:08X}", periph.name, periph.base_address);

                    let resp = egui::CollapsingHeader::new(
                        egui::RichText::new(&header).monospace().small(),
                    )
                    .id_salt(&periph.name)
                    .default_open(is_expanded)
                    .show(ui, |ui| {
                        if !periph.description.is_empty() {
                            ui.label(
                                egui::RichText::new(&periph.description)
                                    .small()
                                    .color(egui::Color32::GRAY),
                            );
                        }
                        for reg in &periph.registers {
                            let abs_addr = periph.base_address + reg.address_offset;
                            let reg_label = format!(
                                "  {} +0x{:X}  [{}]",
                                reg.name, reg.address_offset, reg.access
                            );

                            ui.horizontal(|ui| {
                                ui.label(egui::RichText::new(&reg_label).monospace().small());
                                if ui
                                    .small_button("📋")
                                    .on_hover_text(format!("Copy 0x{:08X}", abs_addr))
                                    .clicked()
                                {
                                    ui.ctx().copy_text(format!("0x{:08X}", abs_addr));
                                }
                            });

                            if !reg.description.is_empty() {
                                ui.label(
                                    egui::RichText::new(format!("    {}", reg.description))
                                        .small()
                                        .color(egui::Color32::GRAY),
                                );
                            }

                            // Show fields
                            for field in &reg.fields {
                                let field_label = format!(
                                    "    [{}:{}] {} — {}",
                                    field.bit_offset + field.bit_width - 1,
                                    field.bit_offset,
                                    field.name,
                                    field.description
                                );
                                ui.label(
                                    egui::RichText::new(&field_label)
                                        .small()
                                        .color(egui::Color32::from_gray(160)),
                                );
                            }
                        }
                    });

                    // Track expansion state
                    if resp.fully_open() {
                        app.svd_expanded_peripherals.insert(periph.name.clone());
                    } else if resp.fully_closed() {
                        app.svd_expanded_peripherals.remove(&periph.name);
                    }
                }
            });
        }
    }
}

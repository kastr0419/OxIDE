// SPDX-License-Identifier: MIT OR Apache-2.0
// Pinout panel UI

use crate::app::IdeApp;
use crate::core::pinout::{PinFunction, PinInfo};
use egui::{Color32, Pos2, RichText, Sense, Stroke, Vec2};

// ─── colour + label for each function ────────────────────────────────────────

fn func_color(f: PinFunction) -> Color32 {
    match f {
        PinFunction::Gpio => Color32::from_rgb(80, 200, 90),
        PinFunction::Uart => Color32::from_rgb(255, 140, 60),
        PinFunction::Spi => Color32::from_rgb(100, 140, 255),
        PinFunction::I2C => Color32::from_rgb(240, 200, 50),
        PinFunction::Pwm => Color32::from_rgb(200, 100, 220),
        PinFunction::Adc => Color32::from_rgb(60, 200, 180),
        PinFunction::Power => Color32::from_rgb(255, 100, 100),
        PinFunction::Gnd => Color32::from_rgb(110, 110, 110),
        PinFunction::Nc => Color32::from_rgb(170, 170, 170),
    }
}

fn func_label(f: PinFunction) -> &'static str {
    match f {
        PinFunction::Gpio => "GPIO",
        PinFunction::Uart => "UART",
        PinFunction::Spi => "SPI",
        PinFunction::I2C => "I2C",
        PinFunction::Pwm => "PWM",
        PinFunction::Adc => "ADC",
        PinFunction::Power => "PWR",
        PinFunction::Gnd => "GND",
        PinFunction::Nc => "NC",
    }
}

fn func_label_ja(f: PinFunction) -> &'static str {
    match f {
        PinFunction::Gpio => "汎用 I/O",
        PinFunction::Uart => "シリアル (UART)",
        PinFunction::Spi => "SPI バス",
        PinFunction::I2C => "I2C バス",
        PinFunction::Pwm => "PWM 出力",
        PinFunction::Adc => "アナログ入力 (ADC)",
        PinFunction::Power => "電源 (VCC/3V3/5V)",
        PinFunction::Gnd => "GND",
        PinFunction::Nc => "未接続 (NC)",
    }
}

// Primary colour of a pin (first function wins)
fn pin_color(fns: &[PinFunction]) -> Color32 {
    fns.first()
        .map(|&f| func_color(f))
        .unwrap_or(Color32::from_rgb(170, 170, 170))
}

// Whether this pin passes the current filter
fn pin_visible(fns: &[PinFunction], filter: u8) -> bool {
    if filter == 0 {
        return true;
    }
    let wanted = match filter {
        1 => PinFunction::Gpio,
        2 => PinFunction::Uart,
        3 => PinFunction::Spi,
        4 => PinFunction::I2C,
        5 => PinFunction::Pwm,
        6 => PinFunction::Adc,
        7 => PinFunction::Power,
        8 => PinFunction::Gnd,
        _ => return true,
    };
    fns.contains(&wanted)
}

// ─── Legend row ───────────────────────────────────────────────────────────────

const FILTER_ENTRIES: &[(u8, &str, PinFunction)] = &[
    (1, "GPIO", PinFunction::Gpio),
    (2, "UART", PinFunction::Uart),
    (3, "SPI", PinFunction::Spi),
    (4, "I2C", PinFunction::I2C),
    (5, "PWM", PinFunction::Pwm),
    (6, "ADC", PinFunction::Adc),
    (7, "PWR", PinFunction::Power),
    (8, "GND", PinFunction::Gnd),
];

fn ui_legend_and_filter(ui: &mut egui::Ui, filter: &mut u8) {
    ui.horizontal_wrapped(|ui| {
        let sel = egui::SelectableLabel::new(*filter == 0, "ALL");
        if ui.add(sel).clicked() {
            *filter = 0;
        }
        for (id, label, func) in FILTER_ENTRIES {
            let color = func_color(*func);
            let text = RichText::new(*label)
                .color(if *filter == *id {
                    Color32::WHITE
                } else {
                    color
                })
                .strong();
            let btn = egui::Button::new(text)
                .fill(if *filter == *id {
                    color.linear_multiply(0.8)
                } else {
                    Color32::TRANSPARENT
                })
                .stroke(Stroke::new(1.0, color));
            if ui.add(btn).clicked() {
                *filter = if *filter == *id { 0 } else { *id };
            }
        }
    });
}

// ─── Diagram view ─────────────────────────────────────────────────────────────

fn ui_diagram(ui: &mut egui::Ui, pins: &[PinInfo], filter: u8, hovered: &mut Option<u8>) {
    let avail_w = ui.available_width().clamp(260.0, 460.0);
    let size = Vec2::new(avail_w, avail_w * 0.75);
    let (rect, response) = ui.allocate_exact_size(size, Sense::hover());
    let painter = ui.painter();

    // Board outline
    painter.rect_filled(rect, 8.0, Color32::from_rgb(30, 35, 40));
    painter.rect_stroke(
        rect,
        8.0,
        Stroke::new(1.5, Color32::from_rgb(70, 90, 100)),
        egui::StrokeKind::Inside,
    );

    let hover_pos = response.hover_pos();
    let mut newly_hovered: Option<u8> = None;

    let pin_r = (avail_w * 0.028).clamp(6.0, 12.0);
    let font_sm = egui::FontId::monospace((pin_r * 0.9).max(7.5));

    for p in pins.iter() {
        let visible = pin_visible(p.functions, filter);
        let cx = rect.left() + p.x * rect.width();
        let cy = rect.top() + p.y * rect.height();
        let center = Pos2::new(cx, cy);
        let color = pin_color(p.functions);
        let alpha_color = if visible {
            color
        } else {
            color.linear_multiply(0.18)
        };

        // Circle
        painter.circle_filled(center, pin_r, alpha_color);
        let is_selected = hovered.map(|n| n == p.number).unwrap_or(false);
        let stroke_color = if is_selected {
            Color32::WHITE
        } else {
            Color32::BLACK
        };
        let stroke_w = if is_selected { 2.0 } else { 1.0 };
        painter.circle_stroke(center, pin_r, Stroke::new(stroke_w, stroke_color));

        // Pin label inside circle
        if visible {
            let short: String = p.name.chars().take(4).collect();
            painter.text(
                center,
                egui::Align2::CENTER_CENTER,
                short,
                font_sm.clone(),
                Color32::from_rgb(10, 10, 10),
            );
        }

        // Hover detection
        if visible {
            if let Some(hp) = hover_pos {
                if hp.distance(center) <= pin_r + 3.0 {
                    newly_hovered = Some(p.number);
                    // Tooltip
                    let funcs: Vec<&str> = p.functions.iter().map(|&f| func_label(f)).collect();
                    response.clone().on_hover_ui_at_pointer(|tip| {
                        tip.strong(p.name);
                        tip.label(funcs.join(" / "));
                    });
                }
            }
        }
    }

    *hovered = newly_hovered;
}

// ─── Table view ───────────────────────────────────────────────────────────────

fn ui_table(ui: &mut egui::Ui, pins: &[PinInfo], filter: u8, hovered: &mut Option<u8>) {
    use egui_extras::{Column, TableBuilder};

    TableBuilder::new(ui)
        .striped(true)
        .resizable(false)
        .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
        .column(Column::exact(34.0)) // #
        .column(Column::remainder()) // Name
        .column(Column::exact(170.0)) // Functions
        .header(20.0, |mut row| {
            row.col(|ui| {
                ui.strong("#");
            });
            row.col(|ui| {
                ui.strong("名前");
            });
            row.col(|ui| {
                ui.strong("機能");
            });
        })
        .body(|mut body| {
            for p in pins.iter().filter(|p| pin_visible(p.functions, filter)) {
                let is_sel = hovered.map(|n| n == p.number).unwrap_or(false);
                body.row(20.0, |mut row| {
                    row.col(|ui| {
                        let txt = RichText::new(format!("{}", p.number)).color(if is_sel {
                            ui.visuals().warn_fg_color
                        } else {
                            Color32::GRAY
                        });
                        ui.label(txt);
                    });
                    row.col(|ui| {
                        let r = ui.selectable_label(is_sel, p.name);
                        if r.clicked() {
                            *hovered = if is_sel { None } else { Some(p.number) };
                        }
                        if r.hovered() && !is_sel {
                            *hovered = Some(p.number);
                        }
                    });
                    row.col(|ui| {
                        ui.horizontal(|ui| {
                            for &f in p.functions {
                                let c = func_color(f);
                                let badge = RichText::new(func_label(f)).color(c).small().strong();
                                ui.label(badge);
                                ui.add_space(2.0);
                            }
                        });
                    });
                });
            }
        });
}

// ─── Detail card for selected pin ────────────────────────────────────────────

fn ui_pin_detail(ui: &mut egui::Ui, pins: &[PinInfo], hovered: Option<u8>) {
    let Some(num) = hovered else {
        return;
    };
    let Some(p) = pins.iter().find(|pp| pp.number == num) else {
        return;
    };

    ui.separator();
    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.strong(format!("📌  {}", p.name));
            ui.label(
                RichText::new(format!("(Pin {})", p.number))
                    .color(Color32::GRAY)
                    .small(),
            );
        });
        ui.add_space(4.0);
        ui.horizontal_wrapped(|ui| {
            for &f in p.functions {
                let c = func_color(f);
                let badge = egui::Button::new(
                    RichText::new(format!("{} {}", func_label(f), func_label_ja(f)))
                        .color(Color32::WHITE)
                        .small(),
                )
                .fill(c.linear_multiply(0.7))
                .stroke(Stroke::new(1.0, c));
                ui.add(badge);
            }
        });
    });
}

// ─── Public entry point ───────────────────────────────────────────────────────

pub fn ui_pinout_panel(app: &mut IdeApp, ui: &mut egui::Ui) {
    let board = crate::core::board::BOARD_PRESETS
        .get(app.selected_board)
        .cloned();

    ui.vertical(|ui| {
        // ── Header ──────────────────────────────────────────────────────────
        ui.horizontal(|ui| {
            ui.strong("📌 Pinout");
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.selectable_value(&mut app.pinout_view_table, true, "📋 Table");
                ui.selectable_value(&mut app.pinout_view_table, false, "🗺 Diagram");
            });
        });

        let Some(b) = board else {
            ui.label("ボードが選択されていません");
            return;
        };
        ui.label(RichText::new(format!("🔧 {}", b.display_name)).strong());

        let Some(pinout) = crate::core::pinout::get_pinout(b.kind) else {
            ui.colored_label(
                ui.visuals().warn_fg_color,
                "このボードのピンアウトデータはありません",
            );
            return;
        };

        // ── Filter ──────────────────────────────────────────────────────────
        let visible_count = pinout
            .pins
            .iter()
            .filter(|p| pin_visible(p.functions, app.pinout_filter))
            .count();
        ui.horizontal(|ui| {
            ui.label(
                RichText::new(format!("{} ピン表示中", visible_count))
                    .small()
                    .color(Color32::GRAY),
            );
        });
        ui_legend_and_filter(ui, &mut app.pinout_filter);

        ui.add_space(4.0);

        // ── Main view ────────────────────────────────────────────────────────
        if app.pinout_view_table {
            ui_table(
                ui,
                pinout.pins,
                app.pinout_filter,
                &mut app.pinout_hovered_pin,
            );
        } else {
            ui_diagram(
                ui,
                pinout.pins,
                app.pinout_filter,
                &mut app.pinout_hovered_pin,
            );
        }

        // ── Detail card ──────────────────────────────────────────────────────
        ui_pin_detail(ui, pinout.pins, app.pinout_hovered_pin);
    });
}

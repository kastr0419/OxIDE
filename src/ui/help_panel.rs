// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright 2026 ALLoIDE contributors

#[derive(PartialEq, Clone, Copy, Default)]
pub enum RightTab {
    #[default]
    SerialMonitor,
    Agent,
    Docs,
    Pinout,
}

const DOCS: &[(&str, &str)] = &[
    // --- ボード別 ---
    ("AVR (Arduino)", include_str!("../../docs/avr.md")),
    ("ESP32", include_str!("../../docs/esp32.md")),
    (
        "nRF / micro:bit",
        include_str!("../../docs/nrf_microbit.md"),
    ),
    ("RISC-V", include_str!("../../docs/riscv.md")),
    ("RP2040 (Pico)", include_str!("../../docs/rp2040.md")),
    ("SAMD (MKR/M0)", include_str!("../../docs/samd.md")),
    ("STM32", include_str!("../../docs/stm32.md")),
    ("Teensy", include_str!("../../docs/teensy.md")),
    (
        "🔧 書き込みトラブルシューティング",
        include_str!("../../docs/troubleshooting.md"),
    ),
    (
        "Raspberry Pi Zero (ベアメタル)",
        include_str!("../../docs/rpi_zero.md"),
    ),
    (
        "📦 コンパイル成果物ガイド",
        include_str!("../../docs/artifacts.md"),
    ),
    // --- 機能ガイド ---
    ("📝 エディタ機能", include_str!("../../docs/editor.md")),
    (
        "🔨 ビルドと書き込み",
        include_str!("../../docs/build_and_flash.md"),
    ),
    (
        "📡 シリアルモニタ",
        include_str!("../../docs/serial_monitor.md"),
    ),
    ("📈 シリアルプロッタ", include_str!("../../docs/plotter.md")),
    ("🐛 デバッグパネル", include_str!("../../docs/debug.md")),
    ("🔩 SVDビューア", include_str!("../../docs/svd.md")),
    ("📌 ピンアウト表示", include_str!("../../docs/pinout.md")),
    (
        "⌨️ ショートカット一覧",
        include_str!("../../docs/shortcuts.md"),
    ),
];

pub fn ui_help_panel(app: &mut crate::app::IdeApp, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.label("📖 MCU Family:");
        egui::ComboBox::from_id_salt("doc_family_cb")
            .selected_text(DOCS[app.selected_doc].0)
            .show_ui(ui, |ui| {
                for (i, (name, _)) in DOCS.iter().enumerate() {
                    if i == 11 {
                        ui.separator(); // ボード別 / 機能ガイド の区切り
                    }
                    ui.selectable_value(&mut app.selected_doc, i, *name);
                }
            });

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.small_button("＋").clicked() {
                app.doc_font_size = (app.doc_font_size + 1.0).min(32.0);
            }
            ui.label(format!("{}px", app.doc_font_size as u32));
            if ui.small_button("－").clicked() {
                app.doc_font_size = (app.doc_font_size - 1.0).max(8.0);
            }
        });
    });
    ui.separator();

    let doc_content = DOCS[app.selected_doc].1;
    let font_size = app.doc_font_size;

    egui::ScrollArea::vertical()
        .id_salt("help_scroll")
        .auto_shrink([false, false])
        .show(ui, |ui| {
            // フォントサイズをこのスコープ内だけ変更
            let orig_style = ui.style().clone();
            {
                let style = ui.style_mut();
                let scale = font_size / 14.0;
                for font_id in style.text_styles.values_mut() {
                    font_id.size = (font_id.size * scale).clamp(6.0, 64.0);
                }
            }

            egui_commonmark::CommonMarkViewer::new().show(ui, &mut app.doc_cache, doc_content);

            // スタイルを元に戻す
            *ui.style_mut() = (*orig_style).clone();
        });
}

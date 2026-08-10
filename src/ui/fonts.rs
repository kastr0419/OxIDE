// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright 2026 rust-embedded-ide contributors

//! 日本語フォントを egui に登録するモジュール。
//!
//! OS のシステムフォントを動的に探索し、CJK グリフを持つフォントを
//! Proportional / Monospace の両ファミリーにフォールバックとして追加する。

/// OS ごとの候補フォントパス（優先順）
#[cfg(target_os = "windows")]
const FONT_CANDIDATES: &[(&str, u32)] = &[
    // (パス, ttc内インデックス)
    ("C:/Windows/Fonts/meiryo.ttc", 0),   // Meiryo Regular
    ("C:/Windows/Fonts/YuGothR.ttc", 0),  // Yu Gothic Regular
    ("C:/Windows/Fonts/msgothic.ttc", 0), // MS Gothic
    ("C:/Windows/Fonts/YuGothM.ttc", 0),  // Yu Gothic Medium
];

#[cfg(target_os = "linux")]
const FONT_CANDIDATES: &[(&str, u32)] = &[
    (
        "/usr/share/fonts/opentype/noto/NotoSansCJKjp-Regular.otf",
        0,
    ),
    (
        "/usr/share/fonts/truetype/noto/NotoSansCJKjp-Regular.ttf",
        0,
    ),
    ("/usr/share/fonts/noto-cjk/NotoSansCJK-Regular.ttc", 0),
    (
        "/usr/share/fonts/google-noto-cjk/NotoSansCJK-Regular.ttc",
        0,
    ),
    // IPAフォント
    ("/usr/share/fonts/truetype/ipafont-gothic/ipag.ttf", 0),
    ("/usr/share/fonts/ipa-gothic/ipag.ttf", 0),
];

#[cfg(target_os = "macos")]
const FONT_CANDIDATES: &[(&str, u32)] = &[
    ("/System/Library/Fonts/ヒラギノ角ゴシック W3.ttc", 0),
    ("/Library/Fonts/Osaka.ttf", 0),
];

#[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
const FONT_CANDIDATES: &[(&str, u32)] = &[];

/// egui の FontDefinitions に日本語フォントを追加する。
///
/// システムフォントが見つかった場合は Proportional / Monospace の末尾に
/// フォールバックとして登録する（既存の欧文フォントを置き換えない）。
/// フォントが見つからない場合は何もしない（tofu 表示のまま）。
pub fn install_japanese_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    let mut installed = false;

    for (path, index) in FONT_CANDIDATES {
        match std::fs::read(path) {
            Ok(data) => {
                let font_data = egui::FontData {
                    font: std::borrow::Cow::Owned(data),
                    index: *index,
                    tweak: egui::FontTweak::default(),
                };
                fonts
                    .font_data
                    .insert("jp_font".to_owned(), font_data.into());

                // 両ファミリーの末尾にフォールバックとして追加
                for family in [egui::FontFamily::Proportional, egui::FontFamily::Monospace] {
                    fonts
                        .families
                        .entry(family)
                        .or_default()
                        .push("jp_font".to_owned());
                }

                eprintln!("[fonts] Japanese font loaded: {} (index {})", path, index);
                installed = true;
                break;
            }
            Err(_) => continue,
        }
    }

    if !installed {
        eprintln!(
            "[fonts] Warning: No Japanese system font found. CJK characters will not render correctly.\n\
             Searched: {:?}",
            FONT_CANDIDATES.iter().map(|(p, _)| p).collect::<Vec<_>>()
        );
    }

    ctx.set_fonts(fonts);
}

// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright 2026 rust-embedded-ide contributors

//! ボード定義の統合テスト

use oxide::core::board::{BoardKind, BOARD_PRESETS, FlashToolKind};

#[test]
fn all_boards_have_valid_target_triple() {
    for p in BOARD_PRESETS {
        assert!(!p.target_triple.is_empty(),
            "Board '{}' has empty target_triple", p.display_name);
        // ターゲットトリプルの基本フォーマット確認
        let parts: Vec<&str> = p.target_triple.split('-').collect();
        assert!(parts.len() >= 2,
            "Board '{}' target '{}' should have at least 2 parts",
            p.display_name, p.target_triple);
    }
}

#[test]
fn avr_boards_use_avrdude() {
    for p in BOARD_PRESETS {
        if p.target_triple.starts_with("avr") {
            assert!(matches!(p.flash_tool, FlashToolKind::Avrdude),
                "AVR board '{}' should use avrdude", p.display_name);
        }
    }
}

#[test]
fn esp32_boards_use_esptool() {
    for p in BOARD_PRESETS {
        if p.target_triple.starts_with("xtensa-esp") || 
           (p.target_triple.starts_with("riscv") && 
            matches!(p.kind, BoardKind::Esp32C3 | BoardKind::Esp32C6 | BoardKind::Esp32H2)) {
            assert!(matches!(p.flash_tool, FlashToolKind::Esptool),
                "ESP board '{}' should use esptool", p.display_name);
        }
    }
}

#[test]
fn cortex_m_boards_use_probe_rs_or_other() {
    for p in BOARD_PRESETS {
        if p.target_triple.starts_with("thumbv") {
            assert!(
                matches!(p.flash_tool,
                    FlashToolKind::ProbeRs | FlashToolKind::Picotool |
                    FlashToolKind::Bossac  | FlashToolKind::StFlash  |
                    FlashToolKind::NrfJprog | FlashToolKind::TeensyLoader |
                    FlashToolKind::DaplinkHex | FlashToolKind::OpenOcd
                ),
                "Cortex-M board '{}' should use a valid flash tool", p.display_name
            );
        }
    }
}

#[test]
fn board_count_at_least_four() {
    assert!(BOARD_PRESETS.len() >= 4,
        "Should have at least 4 boards, got {}", BOARD_PRESETS.len());
}

#[test]
fn find_board_by_kind() {
    let uno = BOARD_PRESETS.iter().find(|p| matches!(p.kind, BoardKind::ArduinoUno));
    assert!(uno.is_some(), "ArduinoUno must exist");

    let pico = BOARD_PRESETS.iter().find(|p| matches!(p.kind, BoardKind::RpiPico));
    // RpiPico は拡張後に存在する想定。存在しなければスキップ
    if let Some(pico) = pico {
        assert_eq!(pico.target_triple, "thumbv6m-none-eabi");
    }
}

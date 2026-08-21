// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright 2026 ALLoIDE contributors

use alloide::core::board::{BoardKind, BOARD_PRESETS};
use alloide::templates::blink::get_blink_template;
use alloide::templates::create_blink_project;

/// すべてのボードにテンプレートが存在することを確認
#[test]
fn all_boards_have_blink_template() {
    let missing: Vec<&str> = BOARD_PRESETS
        .iter()
        .filter(|p| get_blink_template(&p.kind).is_none())
        .map(|p| p.display_name)
        .collect();

    assert!(
        missing.is_empty(),
        "テンプレートが存在しないボード: {:?}",
        missing
    );
}

/// テンプレートの必須フィールドが空でないことを確認
#[test]
fn blink_templates_have_required_fields() {
    for preset in BOARD_PRESETS.iter() {
        let tmpl = get_blink_template(&preset.kind)
            .unwrap_or_else(|| panic!("No template for {}", preset.display_name));

        assert!(
            !tmpl.main_rs.is_empty(),
            "{}: main_rs が空",
            preset.display_name
        );
        assert!(
            !tmpl.cargo_toml.is_empty(),
            "{}: cargo_toml が空",
            preset.display_name
        );
        assert!(
            !tmpl.cargo_config.is_empty(),
            "{}: cargo_config が空",
            preset.display_name
        );
        assert!(
            !tmpl.rust_toolchain.is_empty(),
            "{}: rust_toolchain が空",
            preset.display_name
        );
    }
}

/// main.rs にボード固有のキーワードが含まれることを確認
#[test]
fn blink_main_rs_contains_fn_main() {
    for preset in BOARD_PRESETS.iter() {
        let tmpl = get_blink_template(&preset.kind).unwrap();
        // rpi_zero uses `fn rust_main` (linker ENTRY point), others use `fn main`
        assert!(
            tmpl.main_rs.contains("fn main") || tmpl.main_rs.contains("fn rust_main"),
            "{}: main_rs にエントリポイント関数がない",
            preset.display_name
        );
    }
}

/// Cargo.toml に [package] セクションが含まれることを確認
#[test]
fn blink_cargo_toml_has_package_section() {
    for preset in BOARD_PRESETS.iter() {
        let tmpl = get_blink_template(&preset.kind).unwrap();
        assert!(
            tmpl.cargo_toml.contains("[package]"),
            "{}: Cargo.toml に [package] がない",
            preset.display_name
        );
    }
}

/// STM32ボードには memory.x が含まれることを確認
#[test]
fn stm32_templates_have_memory_x() {
    let stm32_boards = [
        BoardKind::Stm32F1,
        BoardKind::Stm32F4,
        BoardKind::Stm32L4,
        BoardKind::Stm32F7,
        BoardKind::Stm32H7,
        BoardKind::Stm32G0,
    ];
    for board in &stm32_boards {
        let tmpl = get_blink_template(board).unwrap();
        assert!(tmpl.memory_x.is_some(), "{:?}: memory.x がない", board);
    }
}

/// create_blink_project がファイルシステムに正しく書き込むことを確認
#[test]
fn create_blink_project_writes_files() {
    let tmp = std::env::temp_dir().join("rust_embedded_test_blink");
    let _ = std::fs::remove_dir_all(&tmp);

    create_blink_project(&tmp, &BoardKind::ArduinoUno).expect("create_blink_project が失敗");

    assert!(tmp.join("src/main.rs").exists(), "src/main.rs が存在しない");
    assert!(tmp.join("Cargo.toml").exists(), "Cargo.toml が存在しない");
    assert!(
        tmp.join(".cargo/config.toml").exists(),
        ".cargo/config.toml が存在しない"
    );
    assert!(
        tmp.join("rust-toolchain.toml").exists(),
        "rust-toolchain.toml が存在しない"
    );

    std::fs::remove_dir_all(&tmp).ok();
}

/// ESP32テンプレートが Xtensa ターゲットを使用していることを確認
#[test]
fn esp32_template_uses_xtensa_target() {
    let tmpl = get_blink_template(&BoardKind::Esp32).unwrap();
    assert!(
        tmpl.cargo_config.contains("xtensa") || tmpl.cargo_config.contains("esp32"),
        "ESP32 cargo config に Xtensa ターゲットがない"
    );
}

/// RISC-Vテンプレートが正しいターゲットを持つことを確認
#[test]
fn riscv_templates_have_riscv_target() {
    for board in &[BoardKind::Gd32Vf103, BoardKind::Ch32V003] {
        let tmpl = get_blink_template(board).unwrap();
        assert!(
            tmpl.cargo_config.contains("riscv"),
            "{:?}: cargo config に riscv ターゲットがない",
            board
        );
    }
}

// 追加テスト: .cargo/config.toml が書き出されることを確認
#[test]
fn create_blink_project_writes_cargo_config() {
    let tmp = std::env::temp_dir().join("test_blink_cargo_config");
    let _ = std::fs::remove_dir_all(&tmp);
    std::fs::create_dir_all(&tmp).unwrap();

    let result =
        alloide::templates::create_blink_project(&tmp, &alloide::core::board::BoardKind::Stm32F4);
    assert!(
        result.is_ok(),
        "create_blink_project failed: {:?}",
        result.err()
    );

    // .cargo/config.toml が書き出されていること
    assert!(
        tmp.join(".cargo").join("config.toml").exists(),
        ".cargo/config.toml not written"
    );

    // cargo build 時に使われる target が config.toml に含まれていること
    let config_content = std::fs::read_to_string(tmp.join(".cargo").join("config.toml")).unwrap();
    assert!(
        config_content.contains("thumbv7em-none-eabihf"),
        "target not in config.toml"
    );

    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn create_blink_project_avr_has_no_memory_x() {
    let tmp = std::env::temp_dir().join("test_blink_avr");
    let _ = std::fs::remove_dir_all(&tmp);
    std::fs::create_dir_all(&tmp).unwrap();

    let result =
        alloide::templates::create_blink_project(&tmp, &alloide::core::board::BoardKind::ArduinoUno);
    assert!(result.is_ok());

    // AVR は memory.x なし
    assert!(
        !tmp.join("memory.x").exists(),
        "AVR should not have memory.x"
    );

    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn create_blink_project_stm32_has_memory_x() {
    let tmp = std::env::temp_dir().join("test_blink_stm32_mem");
    let _ = std::fs::remove_dir_all(&tmp);
    std::fs::create_dir_all(&tmp).unwrap();

    let result =
        alloide::templates::create_blink_project(&tmp, &alloide::core::board::BoardKind::Stm32F4);
    assert!(result.is_ok());

    // STM32 は memory.x あり
    assert!(tmp.join("memory.x").exists(), "STM32 should have memory.x");

    let _ = std::fs::remove_dir_all(&tmp);
}

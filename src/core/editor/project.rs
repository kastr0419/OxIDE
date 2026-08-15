// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright 2026 rust-embedded-ide contributors

use crate::core::board::BoardKind;
use anyhow::Result;
use std::path::Path;

pub struct ProjectInfo {
    pub board: Option<BoardKind>,
    pub project_name: Option<String>,
}

/// プロジェクトディレクトリを開き、ボードを検出する
pub fn open_project(dir: &Path) -> Result<ProjectInfo> {
    let main_rs_path = dir.join("src").join("main.rs");
    if !main_rs_path.exists() {
        return Err(anyhow::anyhow!("src/main.rs が見つかりません: {:?}", dir));
    }
    let board = detect_board(dir);
    let project_name = dir
        .file_name()
        .and_then(|n| n.to_str())
        .map(|s| s.to_string());
    Ok(ProjectInfo {
        board,
        project_name,
    })
}

fn detect_board(dir: &Path) -> Option<BoardKind> {
    let cargo_toml = std::fs::read_to_string(dir.join("Cargo.toml")).ok()?;
    detect_board_from_cargo_toml(&cargo_toml)
}

pub fn detect_board_from_cargo_toml(s: &str) -> Option<BoardKind> {
    if s.contains("microbit-v2") {
        return Some(BoardKind::MicroBitV2);
    }
    if s.contains("rp-pico") || s.contains("rp2040-hal") {
        return Some(BoardKind::RpiPico);
    }
    if s.contains("rp2350") || s.contains("rp-pico2") {
        return Some(BoardKind::RpiPico2);
    }
    if s.contains("stm32f4xx-hal") || s.contains("stm32f4") {
        return Some(BoardKind::Stm32F4);
    }
    if s.contains("stm32f1xx-hal") || s.contains("stm32f1") {
        return Some(BoardKind::Stm32F1);
    }
    if s.contains("stm32l4xx-hal") || s.contains("stm32l4") {
        return Some(BoardKind::Stm32L4);
    }
    if s.contains("stm32f7xx-hal") || s.contains("stm32f7") {
        return Some(BoardKind::Stm32F7);
    }
    if s.contains("stm32h7xx-hal") || s.contains("stm32h7") {
        return Some(BoardKind::Stm32H7);
    }
    if s.contains("stm32g0xx-hal") || s.contains("stm32g0") {
        return Some(BoardKind::Stm32G0);
    }
    if s.contains("nrf52840") {
        return Some(BoardKind::NrF52840);
    }
    if s.contains("nrf51") {
        return Some(BoardKind::NrF51822);
    }
    if s.contains("atsamd51") || s.contains("metro_m4") {
        return Some(BoardKind::Samd51);
    }
    if s.contains("atsamd21") || s.contains("metro_m0") {
        return Some(BoardKind::Samd21);
    }
    if s.contains("teensy4") {
        return Some(BoardKind::Teensy4);
    }
    if s.contains("gd32vf103") {
        return Some(BoardKind::Gd32Vf103);
    }
    if s.contains("ch32v003") {
        return Some(BoardKind::Ch32V003);
    }
    if s.contains("esp32s3") || s.contains("esp-hal") && s.contains("s3") {
        return Some(BoardKind::Esp32S3);
    }
    if s.contains("esp32s2") || s.contains("esp-hal") && s.contains("s2") {
        return Some(BoardKind::Esp32S2);
    }
    if s.contains("esp32c6") {
        return Some(BoardKind::Esp32C6);
    }
    if s.contains("esp32c3") || s.contains("esp-hal") && s.contains("c3") {
        return Some(BoardKind::Esp32C3);
    }
    if s.contains("esp32h2") {
        return Some(BoardKind::Esp32H2);
    }
    if s.contains("esp-hal") || s.contains("esp32") {
        return Some(BoardKind::Esp32);
    }
    if s.contains("arduino-hal") || s.contains("avr-hal") {
        return Some(BoardKind::ArduinoUno);
    }
    None
}

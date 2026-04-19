// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright 2026 rust-embedded-ide contributors

use crate::core::board::BoardKind;

pub mod avr;
pub mod rp;
pub mod stm32;
pub mod microbit;
pub mod esp;
pub mod nrf;
pub mod samd;
pub mod teensy;
pub mod rpi_zero;
pub mod riscv;

/// Lチカプロジェクトのテンプレートファイル群
pub struct BlinkTemplate {
    /// src/main.rs の内容
    pub main_rs: &'static str,
    /// Cargo.toml の内容
    pub cargo_toml: &'static str,
    /// .cargo/config.toml の内容
    pub cargo_config: &'static str,
    /// rust-toolchain.toml の内容
    pub rust_toolchain: &'static str,
    /// memory.x の内容（Cortex-Mのみ Some）
    pub memory_x: Option<&'static str>,
    /// build.rs の内容（必要な場合のみ Some）
    pub build_rs: Option<&'static str>,
    /// linker.ld の内容（RPi Zero等、カスタムリンカスクリプトが必要なボード用）
    pub linker_ld: Option<&'static str>,
    /// カスタムターゲット JSON の内容（RPi Zero等）
    pub target_json: Option<(&'static str, &'static str)>,
}

/// ボードに対応するLチカテンプレートを返す
pub fn get_blink_template(board: &BoardKind) -> Option<BlinkTemplate> {
    match board {
        // AVR
        BoardKind::ArduinoUno       => Some(crate::templates::blink::avr::arduino_uno()),
        BoardKind::ArduinoNano      => Some(crate::templates::blink::avr::arduino_nano()),
        BoardKind::ArduinoMega      => Some(crate::templates::blink::avr::arduino_mega()),
        BoardKind::ArduinoLeonardo  => Some(crate::templates::blink::avr::arduino_leonardo()),
        // RPi Pico
        BoardKind::RpiPico          => Some(crate::templates::blink::rp::rpi_pico()),
        BoardKind::RpiPico2         => Some(crate::templates::blink::rp::rpi_pico2()),
        // STM32
        BoardKind::Stm32F1          => Some(crate::templates::blink::stm32::stm32f1()),
        BoardKind::Stm32F4          => Some(crate::templates::blink::stm32::stm32f4()),
        BoardKind::Stm32L4          => Some(crate::templates::blink::stm32::stm32l4()),
        BoardKind::Stm32F7          => Some(crate::templates::blink::stm32::stm32f7()),
        BoardKind::Stm32H7          => Some(crate::templates::blink::stm32::stm32h7()),
        BoardKind::Stm32G0          => Some(crate::templates::blink::stm32::stm32g0()),
        // micro:bit
        BoardKind::MicroBitV2       => Some(crate::templates::blink::microbit::microbit_v2()),
        // ESP32
        BoardKind::Esp32            => Some(crate::templates::blink::esp::esp32()),
        BoardKind::Esp32S2          => Some(crate::templates::blink::esp::esp32s2()),
        BoardKind::Esp32S3          => Some(crate::templates::blink::esp::esp32s3()),
        BoardKind::Esp32C3          => Some(crate::templates::blink::esp::esp32c3()),
        BoardKind::Esp32C6          => Some(crate::templates::blink::esp::esp32c6()),
        BoardKind::Esp32H2          => Some(crate::templates::blink::esp::esp32h2()),
        // nRF
        BoardKind::NrF52840         => Some(crate::templates::blink::nrf::nrf52840()),
        BoardKind::NrF51822         => Some(crate::templates::blink::nrf::nrf51822()),
        // SAMD
        BoardKind::Samd21           => Some(crate::templates::blink::samd::samd21()),
        BoardKind::Samd51           => Some(crate::templates::blink::samd::samd51()),
        BoardKind::ArduinoDue       => Some(crate::templates::blink::samd::arduino_due()),
        // Teensy
        BoardKind::Teensy4          => Some(crate::templates::blink::teensy::teensy4()),
        // RISC-V
        BoardKind::Gd32Vf103        => Some(crate::templates::blink::riscv::gd32vf103()),
        BoardKind::Ch32V003         => Some(crate::templates::blink::riscv::ch32v003()),
        BoardKind::RpiZero           => Some(crate::templates::blink::rpi_zero::rpi_zero()),
    }
}

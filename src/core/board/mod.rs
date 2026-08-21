// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright 2026 ALLoIDE contributors

#![allow(dead_code)]

/// CPUアーキテクチャファミリー（表示・フィルタ用）
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CpuArch {
    AvrMega,    // AVR 8-bit ATmega
    AvrTiny,    // AVR 8-bit ATtiny
    CortexM0,   // ARM Cortex-M0 / M0+
    CortexM3,   // ARM Cortex-M3
    CortexM4,   // ARM Cortex-M4 / M4F
    CortexM7,   // ARM Cortex-M7
    CortexM33,  // ARM Cortex-M33 (v8-M)
    XtensaLx6,  // ESP32 Xtensa LX6
    XtensaLx7,  // ESP32-S2/S3 Xtensa LX7
    RiscV32,    // RISC-V 32-bit
    ArmV6Arm11, // ARM1176JZF-S (Raspberry Pi Zero)
}

/// ボード識別子
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BoardKind {
    // AVR
    ArduinoUno,
    ArduinoNano,
    ArduinoMega,
    ArduinoLeonardo,
    // ARM Cortex-M0 / M0+
    RpiPico,
    RpiPico2,
    RpiZero,
    Samd21,
    ArduinoDue,
    NrF51822,
    // ARM Cortex-M3
    Stm32F1,
    // ARM Cortex-M4 / M4F
    Stm32F4,
    Stm32L4,
    NrF52840,
    Samd51,
    MicroBitV2,
    // ARM Cortex-M7
    Stm32F7,
    Stm32H7,
    Teensy4,
    // ARM Cortex-M33
    Stm32G0,
    // Xtensa (ESP32系)
    Esp32,
    Esp32S2,
    Esp32S3,
    // RISC-V
    Esp32C3,
    Esp32C6,
    Esp32H2,
    Gd32Vf103,
    Ch32V003,
}

/// フラッシュ書き込みツール
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FlashToolKind {
    Avrdude,
    Esptool,
    SdCard,
    DaplinkHex,
    ProbeRs,
    OpenOcd,
    StFlash,
    Picotool,
    Bossac,
    NrfJprog,
    TeensyLoader,
}

/// USB VID/PID テーブルエントリ
#[derive(Debug, Clone)]
pub struct UsbId {
    pub vid: u16,
    pub pid: u16,
    pub description: &'static str,
}

/// memory.x リンカスクリプト生成用のメモリレイアウト定義
#[derive(Debug, Clone, Copy)]
pub struct MemoryLayout {
    pub flash_origin: u32,
    pub flash_length_kb: u32,
    pub ram_origin: u32,
    pub ram_length_kb: u32,
}

/// ボードプリセット定義
#[derive(Debug, Clone)]
pub struct BoardPreset {
    pub kind: BoardKind,
    pub display_name: &'static str,
    pub cpu_arch: CpuArch,
    pub target_triple: &'static str,
    pub avrdude_mcu: Option<&'static str>,
    pub flash_tool: FlashToolKind,
    pub default_baud: u32,
    pub default_port_hint: Option<&'static str>,
    pub usb_ids: &'static [UsbId],
    /// ツールチェーン注記（カスタムtoolchainが必要な場合）
    pub toolchain_note: Option<&'static str>,
    pub probe_rs_chip: &'static str,
    /// ビルド時に注入する RUSTFLAGS（.cargo/config.toml が存在しない場合）
    pub rustflags: &'static [&'static str],
    pub flash_offset: u32, // esptool 書き込みオフセット（非ESP系は 0）
    /// memory.x 自動生成用メモリレイアウト（Cortex-M/RISC-V 等で必要）
    pub memory_layout: Option<MemoryLayout>,
}

mod presets;
mod usb_ids;

pub mod detector;
pub mod pinout;

pub use presets::BOARD_PRESETS;

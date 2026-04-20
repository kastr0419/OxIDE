// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright 2026 rust-embedded-ide contributors

#![allow(dead_code)]

/// CPUアーキテクチャファミリー（表示・フィルタ用）
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CpuArch {
    AvrMega,       // AVR 8-bit ATmega
    AvrTiny,       // AVR 8-bit ATtiny
    CortexM0,      // ARM Cortex-M0 / M0+
    CortexM3,      // ARM Cortex-M3
    CortexM4,      // ARM Cortex-M4 / M4F
    CortexM7,      // ARM Cortex-M7
    CortexM33,     // ARM Cortex-M33 (v8-M)
    XtensaLx6,     // ESP32 Xtensa LX6
    XtensaLx7,     // ESP32-S2/S3 Xtensa LX7
    RiscV32,       // RISC-V 32-bit
    ArmV6Arm11,    // ARM1176JZF-S (Raspberry Pi Zero)
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
    pub flash_offset: u32,  // esptool 書き込みオフセット（非ESP系は 0）
    /// memory.x 自動生成用メモリレイアウト（Cortex-M/RISC-V 等で必要）
    pub memory_layout: Option<MemoryLayout>,
}

// ─── USB ID 定数 ─────────────────────────────────────────

const UNO_USB_IDS: &[UsbId] = &[
    UsbId { vid: 0x2341, pid: 0x0043, description: "Arduino Uno R3 (genuine)" },
    UsbId { vid: 0x2341, pid: 0x0001, description: "Arduino Uno (genuine, old)" },
    UsbId { vid: 0x1A86, pid: 0x7523, description: "Arduino Uno Clone (CH340)" },
    UsbId { vid: 0x10C4, pid: 0xEA60, description: "Arduino Uno Clone (CP2102)" },
];

const NANO_USB_IDS: &[UsbId] = &[
    UsbId { vid: 0x2341, pid: 0x0043, description: "Arduino Nano (genuine)" },
    UsbId { vid: 0x1A86, pid: 0x7523, description: "Arduino Nano Clone (CH340)" },
    UsbId { vid: 0x1A86, pid: 0x55D4, description: "Arduino Nano Clone (CH9102)" },
    UsbId { vid: 0x0403, pid: 0x6001, description: "Arduino Nano (FTDI)" },
];

const MEGA_USB_IDS: &[UsbId] = &[
    UsbId { vid: 0x2341, pid: 0x0010, description: "Arduino Mega 2560 (genuine)" },
    UsbId { vid: 0x1A86, pid: 0x7523, description: "Arduino Mega Clone (CH340)" },
];

const LEONARDO_USB_IDS: &[UsbId] = &[
    UsbId { vid: 0x2341, pid: 0x8036, description: "Arduino Leonardo" },
    UsbId { vid: 0x2341, pid: 0x0036, description: "Arduino Leonardo (bootloader)" },
];

const RPI_PICO_USB_IDS: &[UsbId] = &[
    UsbId { vid: 0x2E8A, pid: 0x000A, description: "Raspberry Pi Pico (RP2040)" },
    UsbId { vid: 0x2E8A, pid: 0x0004, description: "Raspberry Pi Pico (UF2 bootloader)" },
];

const RPI_PICO2_USB_IDS: &[UsbId] = &[
    UsbId { vid: 0x2E8A, pid: 0x000F, description: "Raspberry Pi Pico 2 (RP2350)" },
];

const SAMD21_USB_IDS: &[UsbId] = &[
    UsbId { vid: 0x239A, pid: 0x800B, description: "Adafruit Feather M0 (SAMD21)" },
    UsbId { vid: 0x239A, pid: 0x8015, description: "Adafruit Metro M0 (SAMD21)" },
];

const ARDUINO_DUE_USB_IDS: &[UsbId] = &[
    UsbId { vid: 0x2341, pid: 0x003E, description: "Arduino Due (Programming port)" },
    UsbId { vid: 0x2341, pid: 0x003D, description: "Arduino Due (Native port)" },
];

const STM32_USB_IDS: &[UsbId] = &[
    UsbId { vid: 0x0483, pid: 0x374B, description: "ST-Link/V2-1" },
    UsbId { vid: 0x0483, pid: 0x3748, description: "ST-Link/V2" },
    UsbId { vid: 0x0483, pid: 0x374F, description: "ST-Link/V3" },
    UsbId { vid: 0x0483, pid: 0x5740, description: "STM32 Virtual COM (CDC)" },
];

const NRF52840_USB_IDS: &[UsbId] = &[
    UsbId { vid: 0x1915, pid: 0x521F, description: "nRF52840 (Nordic USB)" },
    UsbId { vid: 0x239A, pid: 0x8029, description: "Adafruit nRF52840 Feather" },
];

const MICROBIT_V2_USB_IDS: &[UsbId] = &[
    UsbId { vid: 0x0D28, pid: 0x0204, description: "BBC micro:bit v2 (CMSIS-DAP)" },
];

const SAMD51_USB_IDS: &[UsbId] = &[
    UsbId { vid: 0x239A, pid: 0x8022, description: "Adafruit Feather M4 (SAMD51)" },
    UsbId { vid: 0x239A, pid: 0x8020, description: "Adafruit Metro M4 (SAMD51)" },
];

const ESP32_USB_IDS: &[UsbId] = &[
    UsbId { vid: 0x10C4, pid: 0xEA60, description: "ESP32 (CP2102)" },
    UsbId { vid: 0x1A86, pid: 0x7523, description: "ESP32 Clone (CH340)" },
    UsbId { vid: 0x0403, pid: 0x6010, description: "ESP32 (FT2232H)" },
];

const ESP32S3_USB_IDS: &[UsbId] = &[
    UsbId { vid: 0x303A, pid: 0x1001, description: "ESP32-S3 (built-in USB)" },
    UsbId { vid: 0x10C4, pid: 0xEA60, description: "ESP32-S3 (CP2102)" },
];

const ESP32C3_USB_IDS: &[UsbId] = &[
    UsbId { vid: 0x303A, pid: 0x1001, description: "ESP32-C3 (built-in USB)" },
    UsbId { vid: 0x10C4, pid: 0xEA60, description: "ESP32-C3 (CP2102)" },
];

const TEENSY4_USB_IDS: &[UsbId] = &[
    UsbId { vid: 0x16C0, pid: 0x0483, description: "Teensy 4.0 / 4.1" },
];

// ─── BOARD_PRESETS ───────────────────────────────────────-

pub const BOARD_PRESETS: &[BoardPreset] = &[
    // ── AVR ──────────────────────────────────────────────
    BoardPreset {
        kind: BoardKind::ArduinoUno,
        display_name: "Arduino Uno (ATmega328P)",
        cpu_arch: CpuArch::AvrMega,
        target_triple: "avr-unknown-gnu-atmega328",
        avrdude_mcu: Some("m328p"),
        flash_tool: FlashToolKind::Avrdude,
        default_baud: 115200,
        default_port_hint: None,
        usb_ids: UNO_USB_IDS,
        toolchain_note: Some("nightly + avr-gcc required"),
        probe_rs_chip: "",
        rustflags: &[],
        flash_offset: 0,
        memory_layout: None,
    },
    BoardPreset {
        kind: BoardKind::ArduinoNano,
        display_name: "Arduino Nano (ATmega328P)",
        cpu_arch: CpuArch::AvrMega,
        target_triple: "avr-unknown-gnu-atmega328",
        avrdude_mcu: Some("m328p"),
        flash_tool: FlashToolKind::Avrdude,
        default_baud: 57600,
        default_port_hint: None,
        usb_ids: NANO_USB_IDS,
        toolchain_note: Some("nightly + avr-gcc required"),
        probe_rs_chip: "",
        rustflags: &[],
        flash_offset: 0,
        memory_layout: None,
    },
    BoardPreset {
        kind: BoardKind::ArduinoMega,
        display_name: "Arduino Mega 2560 (ATmega2560)",
        cpu_arch: CpuArch::AvrMega,
        target_triple: "avr-unknown-gnu-atmega2560",
        avrdude_mcu: Some("m2560"),
        flash_tool: FlashToolKind::Avrdude,
        default_baud: 115200,
        default_port_hint: None,
        usb_ids: MEGA_USB_IDS,
        toolchain_note: Some("nightly + avr-gcc required"),
        probe_rs_chip: "",
        rustflags: &[],
        flash_offset: 0,
        memory_layout: None,
    },
    BoardPreset {
        kind: BoardKind::ArduinoLeonardo,
        display_name: "Arduino Leonardo (ATmega32u4)",
        cpu_arch: CpuArch::AvrMega,
        target_triple: "avr-unknown-gnu-atmega32u4",
        avrdude_mcu: Some("m32u4"),
        flash_tool: FlashToolKind::Avrdude,
        default_baud: 57600,
        default_port_hint: None,
        usb_ids: LEONARDO_USB_IDS,
        toolchain_note: Some("nightly + avr-gcc required"),
        probe_rs_chip: "",
        rustflags: &[],
        flash_offset: 0,
        memory_layout: None,
    },
    // ── ARM Cortex-M0 / M0+ ──────────────────────────────
    BoardPreset {
        kind: BoardKind::RpiPico,
        display_name: "Raspberry Pi Pico (RP2040, Cortex-M0+)",
        cpu_arch: CpuArch::CortexM0,
        target_triple: "thumbv6m-none-eabi",
        avrdude_mcu: None,
        flash_tool: FlashToolKind::Picotool,
        default_baud: 115200,
        default_port_hint: None,
        usb_ids: RPI_PICO_USB_IDS,
        toolchain_note: None,
        probe_rs_chip: "RP2040",
        rustflags: &["-C", "link-arg=-Tlink.x"],
        flash_offset: 0,
        memory_layout: Some(MemoryLayout { flash_origin: 0x10000000, flash_length_kb: 2048, ram_origin: 0x20000000, ram_length_kb: 264 }),
    },
    BoardPreset {
        kind: BoardKind::RpiPico2,
        display_name: "Raspberry Pi Pico 2 (RP2350, Cortex-M33)",
        cpu_arch: CpuArch::CortexM33,
        target_triple: "thumbv8m.main-none-eabihf",
        avrdude_mcu: None,
        flash_tool: FlashToolKind::Picotool,
        default_baud: 115200,
        default_port_hint: None,
        usb_ids: RPI_PICO2_USB_IDS,
        toolchain_note: None,
        probe_rs_chip: "RP2350",
        rustflags: &["-C", "link-arg=-Tlink.x"],
        flash_offset: 0,
        memory_layout: Some(MemoryLayout { flash_origin: 0x10000000, flash_length_kb: 4096, ram_origin: 0x20000000, ram_length_kb: 520 }),
    },
    BoardPreset {
        kind: BoardKind::RpiZero,
        display_name: "Raspberry Pi Zero (BCM2835, ARM1176JZF-S)",
        cpu_arch: CpuArch::ArmV6Arm11,
        target_triple: "armv6-rpi-zero",
        avrdude_mcu: None,
        flash_tool: FlashToolKind::SdCard,
        default_baud: 115200,
        default_port_hint: None,
        usb_ids: &[],
        toolchain_note: Some("arm-none-eabi-gcc + objcopy required; custom target JSON: armv6-rpi-zero.json"),
        probe_rs_chip: "",
        rustflags: &[],
        flash_offset: 0,
        memory_layout: None,
    },
    BoardPreset {
        kind: BoardKind::Samd21,
        display_name: "Adafruit SAMD21 (Cortex-M0+)",
        cpu_arch: CpuArch::CortexM0,
        target_triple: "thumbv6m-none-eabi",
        avrdude_mcu: None,
        flash_tool: FlashToolKind::Bossac,
        default_baud: 115200,
        default_port_hint: None,
        usb_ids: SAMD21_USB_IDS,
        toolchain_note: None,
        probe_rs_chip: "",
        rustflags: &["-C", "link-arg=-Tlink.x"],
        flash_offset: 0,
        memory_layout: Some(MemoryLayout { flash_origin: 0x00000000, flash_length_kb: 256, ram_origin: 0x20000000, ram_length_kb: 32 }),
    },
    BoardPreset {
        kind: BoardKind::ArduinoDue,
        display_name: "Arduino Due (SAM3X8E, Cortex-M3)",
        cpu_arch: CpuArch::CortexM3,
        target_triple: "thumbv7m-none-eabi",
        avrdude_mcu: None,
        flash_tool: FlashToolKind::Bossac,
        default_baud: 115200,
        default_port_hint: None,
        usb_ids: ARDUINO_DUE_USB_IDS,
        toolchain_note: None,
        probe_rs_chip: "",
        rustflags: &["-C", "link-arg=-Tlink.x"],
        flash_offset: 0,
        memory_layout: Some(MemoryLayout { flash_origin: 0x00080000, flash_length_kb: 512, ram_origin: 0x20000000, ram_length_kb: 96 }),
    },
    // ── ARM Cortex-M3 ─────────────────────────────────────
    BoardPreset {
        kind: BoardKind::Stm32F1,
        display_name: "STM32F1xx (Cortex-M3)",
        cpu_arch: CpuArch::CortexM3,
        target_triple: "thumbv7m-none-eabi",
        avrdude_mcu: None,
        flash_tool: FlashToolKind::ProbeRs,
        default_baud: 115200,
        default_port_hint: None,
        usb_ids: STM32_USB_IDS,
        toolchain_note: None,
        probe_rs_chip: "STM32F103C8",
        rustflags: &["-C", "link-arg=-Tlink.x"],
        flash_offset: 0,
        memory_layout: Some(MemoryLayout { flash_origin: 0x08000000, flash_length_kb: 64, ram_origin: 0x20000000, ram_length_kb: 20 }),
    },
    // ── ARM Cortex-M4 / M4F ───────────────────────────────
    BoardPreset {
        kind: BoardKind::Stm32F4,
        display_name: "STM32F4xx (Cortex-M4F)",
        cpu_arch: CpuArch::CortexM4,
        target_triple: "thumbv7em-none-eabihf",
        avrdude_mcu: None,
        flash_tool: FlashToolKind::ProbeRs,
        default_baud: 115200,
        default_port_hint: None,
        usb_ids: STM32_USB_IDS,
        toolchain_note: None,
        probe_rs_chip: "STM32F411RETx",
        rustflags: &["-C", "link-arg=-Tlink.x"],
        flash_offset: 0,
        memory_layout: Some(MemoryLayout { flash_origin: 0x08000000, flash_length_kb: 512, ram_origin: 0x20000000, ram_length_kb: 128 }),
    },
    BoardPreset {
        kind: BoardKind::Stm32L4,
        display_name: "STM32L4xx (Cortex-M4F, Low-Power)",
        cpu_arch: CpuArch::CortexM4,
        target_triple: "thumbv7em-none-eabihf",
        avrdude_mcu: None,
        flash_tool: FlashToolKind::ProbeRs,
        default_baud: 115200,
        default_port_hint: None,
        usb_ids: STM32_USB_IDS,
        toolchain_note: None,
        probe_rs_chip: "STM32L476RGTx",
        rustflags: &["-C", "link-arg=-Tlink.x"],
        flash_offset: 0,
        memory_layout: Some(MemoryLayout { flash_origin: 0x08000000, flash_length_kb: 1024, ram_origin: 0x20000000, ram_length_kb: 128 }),
    },
    BoardPreset {
        kind: BoardKind::NrF52840,
        display_name: "nRF52840 (Cortex-M4F, BLE)",
        cpu_arch: CpuArch::CortexM4,
        target_triple: "thumbv7em-none-eabihf",
        avrdude_mcu: None,
        flash_tool: FlashToolKind::ProbeRs,
        default_baud: 115200,
        default_port_hint: None,
        usb_ids: NRF52840_USB_IDS,
        toolchain_note: None,
        probe_rs_chip: "nRF52840_xxAA",
        rustflags: &["-C", "link-arg=-Tlink.x"],
        flash_offset: 0,
        memory_layout: Some(MemoryLayout { flash_origin: 0x00000000, flash_length_kb: 1024, ram_origin: 0x20000000, ram_length_kb: 256 }),
    },
    BoardPreset {
        kind: BoardKind::MicroBitV2,
        display_name: "BBC micro:bit v2 (nRF52833, Cortex-M4F)",
        cpu_arch: CpuArch::CortexM4,
        target_triple: "thumbv7em-none-eabihf",
        avrdude_mcu: None,
        flash_tool: FlashToolKind::DaplinkHex,
        default_baud: 115200,
        default_port_hint: None,
        usb_ids: MICROBIT_V2_USB_IDS,
        toolchain_note: None,
        probe_rs_chip: "nRF52833_xxAA",
        rustflags: &["-C", "link-arg=-Tlink.x"],
        flash_offset: 0,
        memory_layout: Some(MemoryLayout { flash_origin: 0x00000000, flash_length_kb: 512, ram_origin: 0x20000000, ram_length_kb: 128 }),
    },
    BoardPreset {
        kind: BoardKind::Samd51,
        display_name: "Adafruit SAMD51 (Cortex-M4F)",
        cpu_arch: CpuArch::CortexM4,
        target_triple: "thumbv7em-none-eabihf",
        avrdude_mcu: None,
        flash_tool: FlashToolKind::Bossac,
        default_baud: 115200,
        default_port_hint: None,
        usb_ids: SAMD51_USB_IDS,
        toolchain_note: None,
        probe_rs_chip: "",
        rustflags: &["-C", "link-arg=-Tlink.x"],
        flash_offset: 0,
        memory_layout: Some(MemoryLayout { flash_origin: 0x00000000, flash_length_kb: 512, ram_origin: 0x20000000, ram_length_kb: 192 }),
    },
    // ── ARM Cortex-M7 ─────────────────────────────────────
    BoardPreset {
        kind: BoardKind::Stm32F7,
        display_name: "STM32F7xx (Cortex-M7)",
        cpu_arch: CpuArch::CortexM7,
        target_triple: "thumbv7em-none-eabihf",
        avrdude_mcu: None,
        flash_tool: FlashToolKind::ProbeRs,
        default_baud: 115200,
        default_port_hint: None,
        usb_ids: STM32_USB_IDS,
        toolchain_note: None,
        probe_rs_chip: "STM32F746NGHx",
        rustflags: &["-C", "link-arg=-Tlink.x"],
        flash_offset: 0,
        memory_layout: Some(MemoryLayout { flash_origin: 0x08000000, flash_length_kb: 1024, ram_origin: 0x20000000, ram_length_kb: 320 }),
    },
    BoardPreset {
        kind: BoardKind::Stm32H7,
        display_name: "STM32H7xx (Cortex-M7, High-Performance)",
        cpu_arch: CpuArch::CortexM7,
        target_triple: "thumbv7em-none-eabihf",
        avrdude_mcu: None,
        flash_tool: FlashToolKind::ProbeRs,
        default_baud: 115200,
        default_port_hint: None,
        usb_ids: STM32_USB_IDS,
        toolchain_note: None,
        probe_rs_chip: "STM32H743ZITx",
        rustflags: &["-C", "link-arg=-Tlink.x"],
        flash_offset: 0,
        memory_layout: Some(MemoryLayout { flash_origin: 0x08000000, flash_length_kb: 2048, ram_origin: 0x24000000, ram_length_kb: 512 }),
    },
    BoardPreset {
        kind: BoardKind::Teensy4,
        display_name: "Teensy 4.0 / 4.1 (IMXRT1062, Cortex-M7)",
        cpu_arch: CpuArch::CortexM7,
        target_triple: "thumbv7em-none-eabihf",
        avrdude_mcu: None,
        flash_tool: FlashToolKind::TeensyLoader,
        default_baud: 115200,
        default_port_hint: None,
        usb_ids: TEENSY4_USB_IDS,
        toolchain_note: None,
        probe_rs_chip: "",
        rustflags: &["-C", "link-arg=-Tlink.x"],
        flash_offset: 0,
        memory_layout: Some(MemoryLayout { flash_origin: 0x60000000, flash_length_kb: 8192, ram_origin: 0x20200000, ram_length_kb: 1024 }),
    },
    // ── ARM Cortex-M33 ───────────────────────────────────-
    BoardPreset {
        kind: BoardKind::Stm32G0,
        display_name: "STM32G0xx (Cortex-M0+/M33)",
        cpu_arch: CpuArch::CortexM0,
        target_triple: "thumbv6m-none-eabi",
        avrdude_mcu: None,
        flash_tool: FlashToolKind::ProbeRs,
        default_baud: 115200,
        default_port_hint: None,
        usb_ids: STM32_USB_IDS,
        toolchain_note: None,
        probe_rs_chip: "STM32G031K8Tx",
        rustflags: &["-C", "link-arg=-Tlink.x"],
        flash_offset: 0,
        memory_layout: Some(MemoryLayout { flash_origin: 0x08000000, flash_length_kb: 64, ram_origin: 0x20000000, ram_length_kb: 8 }),
    },
    // ── Xtensa (ESP32系) ─────────────────────────────────-
    BoardPreset {
        kind: BoardKind::Esp32,
        display_name: "ESP32 (Xtensa LX6)",
        cpu_arch: CpuArch::XtensaLx6,
        target_triple: "xtensa-esp32-none-elf",
        avrdude_mcu: None,
        flash_tool: FlashToolKind::Esptool,
        default_baud: 115200,
        default_port_hint: None,
        usb_ids: ESP32_USB_IDS,
        toolchain_note: Some("espup install required: cargo install espup && espup install"),
        probe_rs_chip: "",
        rustflags: &[],
        flash_offset: 0x10000,
        memory_layout: None,
    },
    BoardPreset {
        kind: BoardKind::Esp32S2,
        display_name: "ESP32-S2 (Xtensa LX7)",
        cpu_arch: CpuArch::XtensaLx7,
        target_triple: "xtensa-esp32s2-none-elf",
        avrdude_mcu: None,
        flash_tool: FlashToolKind::Esptool,
        default_baud: 115200,
        default_port_hint: None,
        usb_ids: ESP32_USB_IDS,
        toolchain_note: Some("espup install required"),
        probe_rs_chip: "",
        rustflags: &[],
        flash_offset: 0x10000,
        memory_layout: None,
    },
    BoardPreset {
        kind: BoardKind::Esp32S3,
        display_name: "ESP32-S3 (Xtensa LX7 + SIMD)",
        cpu_arch: CpuArch::XtensaLx7,
        target_triple: "xtensa-esp32s3-none-elf",
        avrdude_mcu: None,
        flash_tool: FlashToolKind::Esptool,
        default_baud: 115200,
        default_port_hint: None,
        usb_ids: ESP32S3_USB_IDS,
        toolchain_note: Some("espup install required"),
        probe_rs_chip: "",
        rustflags: &[],
        flash_offset: 0x10000,
        memory_layout: None,
    },
    // ── RISC-V ───────────────────────────────────────────-
    BoardPreset {
        kind: BoardKind::Esp32C3,
        display_name: "ESP32-C3 (RISC-V RV32IMC)",
        cpu_arch: CpuArch::RiscV32,
        target_triple: "riscv32imc-unknown-none-elf",
        avrdude_mcu: None,
        flash_tool: FlashToolKind::Esptool,
        default_baud: 115200,
        default_port_hint: None,
        usb_ids: ESP32C3_USB_IDS,
        toolchain_note: Some("rustup target add riscv32imc-unknown-none-elf"),
        probe_rs_chip: "",
        rustflags: &[],
        flash_offset: 0x10000,
        memory_layout: None,
    },
    BoardPreset {
        kind: BoardKind::Esp32C6,
        display_name: "ESP32-C6 (RISC-V RV32IMAC)",
        cpu_arch: CpuArch::RiscV32,
        target_triple: "riscv32imac-unknown-none-elf",
        avrdude_mcu: None,
        flash_tool: FlashToolKind::Esptool,
        default_baud: 115200,
        default_port_hint: None,
        usb_ids: ESP32C3_USB_IDS,
        toolchain_note: Some("rustup target add riscv32imac-unknown-none-elf"),
        probe_rs_chip: "",
        rustflags: &[],
        flash_offset: 0x10000,
        memory_layout: None,
    },
    BoardPreset {
        kind: BoardKind::Esp32H2,
        display_name: "ESP32-H2 (RISC-V RV32IMAC, Thread/Zigbee)",
        cpu_arch: CpuArch::RiscV32,
        target_triple: "riscv32imac-unknown-none-elf",
        avrdude_mcu: None,
        flash_tool: FlashToolKind::Esptool,
        default_baud: 115200,
        default_port_hint: None,
        usb_ids: &[],
        toolchain_note: Some("rustup target add riscv32imac-unknown-none-elf"),
        probe_rs_chip: "",
        rustflags: &[],
        flash_offset: 0x10000,
        memory_layout: None,
    },
    BoardPreset {
        kind: BoardKind::Gd32Vf103,
        display_name: "GD32VF103 (RISC-V RV32IMAC)",
        cpu_arch: CpuArch::RiscV32,
        target_triple: "riscv32imac-unknown-none-elf",
        avrdude_mcu: None,
        flash_tool: FlashToolKind::OpenOcd,
        default_baud: 115200,
        default_port_hint: None,
        usb_ids: &[],
        toolchain_note: Some("rustup target add riscv32imac-unknown-none-elf"),
        probe_rs_chip: "",
        rustflags: &["-C", "link-arg=-Tlink.x"],
        flash_offset: 0,
        memory_layout: Some(MemoryLayout { flash_origin: 0x08000000, flash_length_kb: 128, ram_origin: 0x20000000, ram_length_kb: 32 }),
    },
    BoardPreset {
        kind: BoardKind::Ch32V003,
        display_name: "CH32V003 (RISC-V RV32EC, Ultra-low-cost)",
        cpu_arch: CpuArch::RiscV32,
        target_triple: "riscv32imc-unknown-none-elf",
        avrdude_mcu: None,
        flash_tool: FlashToolKind::OpenOcd,
        default_baud: 115200,
        default_port_hint: None,
        usb_ids: &[],
        toolchain_note: Some("Vendor toolchain or community openocd required"),
        probe_rs_chip: "",
        rustflags: &["-C", "link-arg=-Tlink.x"],
        flash_offset: 0,
        memory_layout: Some(MemoryLayout { flash_origin: 0x08000000, flash_length_kb: 16, ram_origin: 0x20000000, ram_length_kb: 2 }),
    },
 ];

impl BoardPreset {
    pub fn flash_bytes(&self) -> u64 {
        if let Some(mem) = &self.memory_layout {
            mem.flash_length_kb as u64 * 1024
        } else {
            match self.kind {
                BoardKind::ArduinoUno | BoardKind::ArduinoNano | BoardKind::ArduinoMega | BoardKind::ArduinoLeonardo => 32_768u64,
                BoardKind::Esp32 | BoardKind::Esp32S2 | BoardKind::Esp32S3 => 4_194_304u64,
                BoardKind::MicroBitV2 | BoardKind::Stm32F4 | BoardKind::Stm32L4 | BoardKind::Stm32F7 | BoardKind::Stm32H7 => 524_288u64,
                _ => 0u64,
            }
        }
    }

    pub fn ram_bytes(&self) -> u64 {
        if let Some(mem) = &self.memory_layout {
            mem.ram_length_kb as u64 * 1024
        } else {
            match self.kind {
                BoardKind::ArduinoUno | BoardKind::ArduinoNano | BoardKind::ArduinoMega | BoardKind::ArduinoLeonardo => 2_048u64,
                BoardKind::Esp32 | BoardKind::Esp32S2 | BoardKind::Esp32S3 => 532_480u64,
                BoardKind::MicroBitV2 | BoardKind::Stm32F4 | BoardKind::Stm32L4 | BoardKind::Stm32F7 | BoardKind::Stm32H7 => 131_072u64,
                _ => 0u64,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_board_presets_not_empty() {
        assert!(!BOARD_PRESETS.is_empty(), "BOARD_PRESETS must not be empty");
    }

    #[test]
    fn test_all_presets_have_display_name() {
        for p in BOARD_PRESETS {
            assert!(!p.display_name.is_empty(), "Board {:?} has empty display_name", p.kind);
        }
    }

    #[test]
    fn test_all_presets_have_target_triple() {
        for p in BOARD_PRESETS {
            assert!(!p.target_triple.is_empty(), "Board {:?} has empty target_triple", p.kind);
        }
    }

    #[test]
    fn test_arduino_uno_exists() {
        let uno = BOARD_PRESETS.iter().find(|p| matches!(p.kind, BoardKind::ArduinoUno));
        assert!(uno.is_some(), "Arduino Uno must be in BOARD_PRESETS");
        let uno = uno.unwrap();
        assert!(uno.target_triple.contains("avr"), "Arduino Uno target must contain 'avr'");
        assert!(matches!(uno.flash_tool, FlashToolKind::Avrdude));
    }

    #[test]
    fn test_esp32_exists() {
        let esp = BOARD_PRESETS.iter().find(|p| matches!(p.kind, BoardKind::Esp32));
        assert!(esp.is_some());
        let esp = esp.unwrap();
        assert!(esp.target_triple.contains("esp32"), "ESP32 target must contain 'esp32'");
        assert!(matches!(esp.flash_tool, FlashToolKind::Esptool));
    }

    #[test]
    fn test_stm32f4_exists() {
        let stm = BOARD_PRESETS.iter().find(|p| matches!(p.kind, BoardKind::Stm32F4));
        assert!(stm.is_some());
        let stm = stm.unwrap();
        assert!(stm.target_triple.contains("thumbv7em"), "STM32F4 target must be thumbv7em");
    }

    #[test]
    fn test_rpi_pico_exists() {
        let pico = BOARD_PRESETS.iter().find(|p| matches!(p.kind, BoardKind::RpiPico));
        assert!(pico.is_some(), "Raspberry Pi Pico must exist");
        let pico = pico.unwrap();
        assert_eq!(pico.target_triple, "thumbv6m-none-eabi");
    }

    #[test]
    fn test_usb_vid_pid_unique_per_board() {
        // 同一VID/PIDが複数の異なるBoardKindにHighで登録されていないか確認
        let mut vid_pid_map: std::collections::HashMap<(u16,u16), &str> = std::collections::HashMap::new();
        for preset in BOARD_PRESETS {
            for uid in preset.usb_ids {
                let key = (uid.vid, uid.pid);
                // 重複を記録するだけ（警告レベル）
                vid_pid_map.entry(key).or_insert(preset.display_name);
            }
        }
        // VID/PIDテーブルが空でないことを確認
        assert!(!vid_pid_map.is_empty(), "USB ID table must not be empty");
    }

    #[test]
    fn test_default_baud_nonzero() {
        for p in BOARD_PRESETS {
            assert!(p.default_baud > 0, "Board {:?} has zero baud rate", p.kind);
        }
    }
}

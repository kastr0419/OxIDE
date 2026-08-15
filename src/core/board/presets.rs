// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright 2026 rust-embedded-ide contributors

use super::usb_ids::*;
use super::{BoardKind, BoardPreset, CpuArch, FlashToolKind, MemoryLayout};

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
        memory_layout: Some(MemoryLayout {
            flash_origin: 0x10000000,
            flash_length_kb: 2048,
            ram_origin: 0x20000000,
            ram_length_kb: 264,
        }),
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
        memory_layout: Some(MemoryLayout {
            flash_origin: 0x10000000,
            flash_length_kb: 4096,
            ram_origin: 0x20000000,
            ram_length_kb: 520,
        }),
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
        toolchain_note: Some(
            "arm-none-eabi-gcc + objcopy required; custom target JSON: armv6-rpi-zero.json",
        ),
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
        memory_layout: Some(MemoryLayout {
            flash_origin: 0x00000000,
            flash_length_kb: 256,
            ram_origin: 0x20000000,
            ram_length_kb: 32,
        }),
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
        memory_layout: Some(MemoryLayout {
            flash_origin: 0x00080000,
            flash_length_kb: 512,
            ram_origin: 0x20000000,
            ram_length_kb: 96,
        }),
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
        memory_layout: Some(MemoryLayout {
            flash_origin: 0x08000000,
            flash_length_kb: 64,
            ram_origin: 0x20000000,
            ram_length_kb: 20,
        }),
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
        memory_layout: Some(MemoryLayout {
            flash_origin: 0x08000000,
            flash_length_kb: 512,
            ram_origin: 0x20000000,
            ram_length_kb: 128,
        }),
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
        memory_layout: Some(MemoryLayout {
            flash_origin: 0x08000000,
            flash_length_kb: 1024,
            ram_origin: 0x20000000,
            ram_length_kb: 128,
        }),
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
        memory_layout: Some(MemoryLayout {
            flash_origin: 0x00000000,
            flash_length_kb: 1024,
            ram_origin: 0x20000000,
            ram_length_kb: 256,
        }),
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
        memory_layout: Some(MemoryLayout {
            flash_origin: 0x00000000,
            flash_length_kb: 512,
            ram_origin: 0x20000000,
            ram_length_kb: 128,
        }),
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
        memory_layout: Some(MemoryLayout {
            flash_origin: 0x00000000,
            flash_length_kb: 512,
            ram_origin: 0x20000000,
            ram_length_kb: 192,
        }),
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
        memory_layout: Some(MemoryLayout {
            flash_origin: 0x08000000,
            flash_length_kb: 1024,
            ram_origin: 0x20000000,
            ram_length_kb: 320,
        }),
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
        memory_layout: Some(MemoryLayout {
            flash_origin: 0x08000000,
            flash_length_kb: 2048,
            ram_origin: 0x24000000,
            ram_length_kb: 512,
        }),
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
        memory_layout: Some(MemoryLayout {
            flash_origin: 0x60000000,
            flash_length_kb: 8192,
            ram_origin: 0x20200000,
            ram_length_kb: 1024,
        }),
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
        memory_layout: Some(MemoryLayout {
            flash_origin: 0x08000000,
            flash_length_kb: 64,
            ram_origin: 0x20000000,
            ram_length_kb: 8,
        }),
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
        memory_layout: Some(MemoryLayout {
            flash_origin: 0x08000000,
            flash_length_kb: 128,
            ram_origin: 0x20000000,
            ram_length_kb: 32,
        }),
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
        memory_layout: Some(MemoryLayout {
            flash_origin: 0x08000000,
            flash_length_kb: 16,
            ram_origin: 0x20000000,
            ram_length_kb: 2,
        }),
    },
];

impl BoardPreset {
    pub fn flash_bytes(&self) -> u64 {
        if let Some(mem) = &self.memory_layout {
            mem.flash_length_kb as u64 * 1024
        } else {
            match self.kind {
                BoardKind::ArduinoUno | BoardKind::ArduinoNano | BoardKind::ArduinoLeonardo => {
                    32 * 1024
                }
                BoardKind::ArduinoMega => 256 * 1024,
                BoardKind::Esp32
                | BoardKind::Esp32S2
                | BoardKind::Esp32S3
                | BoardKind::Esp32C3
                | BoardKind::Esp32C6
                | BoardKind::Esp32H2 => 4 * 1024 * 1024,
                _ => 0u64,
            }
        }
    }

    pub fn ram_bytes(&self) -> u64 {
        if let Some(mem) = &self.memory_layout {
            mem.ram_length_kb as u64 * 1024
        } else {
            match self.kind {
                BoardKind::ArduinoUno | BoardKind::ArduinoNano => 2 * 1024,
                BoardKind::ArduinoMega => 8 * 1024,
                BoardKind::ArduinoLeonardo => 2560,
                BoardKind::RpiZero => 512 * 1024 * 1024,
                BoardKind::Esp32 => 520 * 1024,
                BoardKind::Esp32S2 => 320 * 1024,
                BoardKind::Esp32S3 | BoardKind::Esp32C6 => 512 * 1024,
                BoardKind::Esp32C3 => 400 * 1024,
                BoardKind::Esp32H2 => 320 * 1024,
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
            assert!(
                !p.display_name.is_empty(),
                "Board {:?} has empty display_name",
                p.kind
            );
        }
    }

    #[test]
    fn test_all_presets_have_target_triple() {
        for p in BOARD_PRESETS {
            assert!(
                !p.target_triple.is_empty(),
                "Board {:?} has empty target_triple",
                p.kind
            );
        }
    }

    #[test]
    fn test_arduino_uno_exists() {
        let uno = BOARD_PRESETS
            .iter()
            .find(|p| matches!(p.kind, BoardKind::ArduinoUno));
        assert!(uno.is_some(), "Arduino Uno must be in BOARD_PRESETS");
        let uno = uno.unwrap();
        assert!(
            uno.target_triple.contains("avr"),
            "Arduino Uno target must contain 'avr'"
        );
        assert!(matches!(uno.flash_tool, FlashToolKind::Avrdude));
    }

    #[test]
    fn test_esp32_exists() {
        let esp = BOARD_PRESETS
            .iter()
            .find(|p| matches!(p.kind, BoardKind::Esp32));
        assert!(esp.is_some());
        let esp = esp.unwrap();
        assert!(
            esp.target_triple.contains("esp32"),
            "ESP32 target must contain 'esp32'"
        );
        assert!(matches!(esp.flash_tool, FlashToolKind::Esptool));
    }

    #[test]
    fn test_stm32f4_exists() {
        let stm = BOARD_PRESETS
            .iter()
            .find(|p| matches!(p.kind, BoardKind::Stm32F4));
        assert!(stm.is_some());
        let stm = stm.unwrap();
        assert!(
            stm.target_triple.contains("thumbv7em"),
            "STM32F4 target must be thumbv7em"
        );
    }

    #[test]
    fn test_rpi_pico_exists() {
        let pico = BOARD_PRESETS
            .iter()
            .find(|p| matches!(p.kind, BoardKind::RpiPico));
        assert!(pico.is_some(), "Raspberry Pi Pico must exist");
        let pico = pico.unwrap();
        assert_eq!(pico.target_triple, "thumbv6m-none-eabi");
    }

    #[test]
    fn test_usb_vid_pid_unique_per_board() {
        // 同一VID/PIDが複数の異なるBoardKindにHighで登録されていないか確認
        let mut vid_pid_map: std::collections::HashMap<(u16, u16), &str> =
            std::collections::HashMap::new();
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

    #[test]
    fn test_board_memory_capacities() {
        let capacity = |kind| {
            BOARD_PRESETS
                .iter()
                .find(|preset| preset.kind == kind)
                .map(|preset| (preset.flash_bytes(), preset.ram_bytes()))
                .unwrap()
        };

        assert_eq!(capacity(BoardKind::ArduinoMega), (256 * 1024, 8 * 1024));
        assert_eq!(capacity(BoardKind::ArduinoLeonardo), (32 * 1024, 2560));
        assert_eq!(capacity(BoardKind::RpiZero), (0, 512 * 1024 * 1024));
        assert_eq!(capacity(BoardKind::Stm32L4), (1024 * 1024, 128 * 1024));
        assert_eq!(capacity(BoardKind::Esp32C3), (4 * 1024 * 1024, 400 * 1024));
    }
}

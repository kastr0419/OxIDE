// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright 2026 ALLoIDE contributors

use super::UsbId;

// ─── USB ID 定数 ─────────────────────────────────────────

pub(super) const UNO_USB_IDS: &[UsbId] = &[
    UsbId {
        vid: 0x2341,
        pid: 0x0043,
        description: "Arduino Uno R3 (genuine)",
    },
    UsbId {
        vid: 0x2341,
        pid: 0x0001,
        description: "Arduino Uno (genuine, old)",
    },
    UsbId {
        vid: 0x1A86,
        pid: 0x7523,
        description: "Arduino Uno Clone (CH340)",
    },
    UsbId {
        vid: 0x10C4,
        pid: 0xEA60,
        description: "Arduino Uno Clone (CP2102)",
    },
];

pub(super) const NANO_USB_IDS: &[UsbId] = &[
    UsbId {
        vid: 0x2341,
        pid: 0x0043,
        description: "Arduino Nano (genuine)",
    },
    UsbId {
        vid: 0x1A86,
        pid: 0x7523,
        description: "Arduino Nano Clone (CH340)",
    },
    UsbId {
        vid: 0x1A86,
        pid: 0x55D4,
        description: "Arduino Nano Clone (CH9102)",
    },
    UsbId {
        vid: 0x0403,
        pid: 0x6001,
        description: "Arduino Nano (FTDI)",
    },
];

pub(super) const MEGA_USB_IDS: &[UsbId] = &[
    UsbId {
        vid: 0x2341,
        pid: 0x0010,
        description: "Arduino Mega 2560 (genuine)",
    },
    UsbId {
        vid: 0x1A86,
        pid: 0x7523,
        description: "Arduino Mega Clone (CH340)",
    },
];

pub(super) const LEONARDO_USB_IDS: &[UsbId] = &[
    UsbId {
        vid: 0x2341,
        pid: 0x8036,
        description: "Arduino Leonardo",
    },
    UsbId {
        vid: 0x2341,
        pid: 0x0036,
        description: "Arduino Leonardo (bootloader)",
    },
];

pub(super) const RPI_PICO_USB_IDS: &[UsbId] = &[
    UsbId {
        vid: 0x2E8A,
        pid: 0x000A,
        description: "Raspberry Pi Pico (RP2040)",
    },
    UsbId {
        vid: 0x2E8A,
        pid: 0x0004,
        description: "Raspberry Pi Pico (UF2 bootloader)",
    },
];

pub(super) const RPI_PICO2_USB_IDS: &[UsbId] = &[UsbId {
    vid: 0x2E8A,
    pid: 0x000F,
    description: "Raspberry Pi Pico 2 (RP2350)",
}];

pub(super) const SAMD21_USB_IDS: &[UsbId] = &[
    UsbId {
        vid: 0x239A,
        pid: 0x800B,
        description: "Adafruit Feather M0 (SAMD21)",
    },
    UsbId {
        vid: 0x239A,
        pid: 0x8015,
        description: "Adafruit Metro M0 (SAMD21)",
    },
];

pub(super) const ARDUINO_DUE_USB_IDS: &[UsbId] = &[
    UsbId {
        vid: 0x2341,
        pid: 0x003E,
        description: "Arduino Due (Programming port)",
    },
    UsbId {
        vid: 0x2341,
        pid: 0x003D,
        description: "Arduino Due (Native port)",
    },
];

pub(super) const STM32_USB_IDS: &[UsbId] = &[
    UsbId {
        vid: 0x0483,
        pid: 0x374B,
        description: "ST-Link/V2-1",
    },
    UsbId {
        vid: 0x0483,
        pid: 0x3748,
        description: "ST-Link/V2",
    },
    UsbId {
        vid: 0x0483,
        pid: 0x374F,
        description: "ST-Link/V3",
    },
    UsbId {
        vid: 0x0483,
        pid: 0x5740,
        description: "STM32 Virtual COM (CDC)",
    },
];

pub(super) const NRF52840_USB_IDS: &[UsbId] = &[
    UsbId {
        vid: 0x1915,
        pid: 0x521F,
        description: "nRF52840 (Nordic USB)",
    },
    UsbId {
        vid: 0x239A,
        pid: 0x8029,
        description: "Adafruit nRF52840 Feather",
    },
];

pub(super) const MICROBIT_V2_USB_IDS: &[UsbId] = &[UsbId {
    vid: 0x0D28,
    pid: 0x0204,
    description: "BBC micro:bit v2 (CMSIS-DAP)",
}];

pub(super) const SAMD51_USB_IDS: &[UsbId] = &[
    UsbId {
        vid: 0x239A,
        pid: 0x8022,
        description: "Adafruit Feather M4 (SAMD51)",
    },
    UsbId {
        vid: 0x239A,
        pid: 0x8020,
        description: "Adafruit Metro M4 (SAMD51)",
    },
];

pub(super) const ESP32_USB_IDS: &[UsbId] = &[
    UsbId {
        vid: 0x10C4,
        pid: 0xEA60,
        description: "ESP32 (CP2102)",
    },
    UsbId {
        vid: 0x1A86,
        pid: 0x7523,
        description: "ESP32 Clone (CH340)",
    },
    UsbId {
        vid: 0x0403,
        pid: 0x6010,
        description: "ESP32 (FT2232H)",
    },
];

pub(super) const ESP32S3_USB_IDS: &[UsbId] = &[
    UsbId {
        vid: 0x303A,
        pid: 0x1001,
        description: "ESP32-S3 (built-in USB)",
    },
    UsbId {
        vid: 0x10C4,
        pid: 0xEA60,
        description: "ESP32-S3 (CP2102)",
    },
];

pub(super) const ESP32C3_USB_IDS: &[UsbId] = &[
    UsbId {
        vid: 0x303A,
        pid: 0x1001,
        description: "ESP32-C3 (built-in USB)",
    },
    UsbId {
        vid: 0x10C4,
        pid: 0xEA60,
        description: "ESP32-C3 (CP2102)",
    },
];

pub(super) const TEENSY4_USB_IDS: &[UsbId] = &[UsbId {
    vid: 0x16C0,
    pid: 0x0483,
    description: "Teensy 4.0 / 4.1",
}];

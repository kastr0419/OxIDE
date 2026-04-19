# OxIDE — a small embedded Rust IDE

A cross-platform GUI IDE for writing embedded firmware in Rust, inspired by the simplicity of the Arduino IDE. Built with egui / eframe and focused on a clean, minimal workflow: edit, build, inspect, and flash.

日本語版: [README.ja.md](README.ja.md)

🚀 Why OxIDE

- Designed for hobbyists and embedded Rust newcomers who want a lightweight, integrated editing/building/flashing workflow.
- Provides focused tooling for common microcontroller families (AVR, RP2040, ESP32, STM32, nRF, etc.).

✨ Implemented features (accurate to source)

- Editor & file explorer: open/save files, workspace file list, multiple tabs.
- Build system: run Cargo builds (with automatic target injection and memory.x generation for presets). Uses `cargo` on PATH.
- Flashing: integrated flash pipeline for supported boards (avrdude, esptool, probe-rs, DAPLink copy, UF2/img where applicable). Some flash backends require external tools installed.
- Serial monitor & plotter: connect to serial ports, send/receive text; a simple CSV/value plotter is included.
- Board picker & auto-detect: choose board preset, refresh ports, and run auto-detection (USB VID/PID, probe-rs, esptool fallbacks).
- Pinout viewer: visual pin maps (diagram + table) for several boards.
- ELF analyzer: basic ELF section and symbol listing (uses `object` crate).
- Stack analyzer: attempts stack-size estimation using `nm`/`arm-none-eabi-nm`.
- SVD viewer: load & browse .svd files (registers, fields).
- Rust Analyzer (LSP) client: embedded client to start `rust-analyzer` for completion and diagnostics (requires rust-analyzer on PATH or configured path).
- Debug UI skeletons: debug / RTT panels are present and wire up to debug command channels (requires external debug tooling to be useful).

Note: Features implemented in the UI call into core modules. Some backends depend on external command-line tools (avrdude, esptool.py, probe-rs, objcopy, nm, rust-analyzer). If a required tool isn't installed the IDE will show an error or fall back.

📋 Project templates (Blink templates available)

OxIDE provides blink/project templates for the following boards (templates generate a ready-to-build Cargo project):

- AVR: Arduino Uno, Arduino Nano, Arduino Mega, Arduino Leonardo
- RP2040: Raspberry Pi Pico, Raspberry Pi Pico 2
- STM32: STM32F1, STM32F4, STM32L4, STM32F7, STM32H7, STM32G0
- micro:bit: micro:bit V2
- ESP32 family: ESP32, ESP32-S2, ESP32-S3, ESP32-C3, ESP32-C6, ESP32-H2
- Nordic: nRF52840, nRF51822
- SAMD: SAMD21, SAMD51, Arduino Due (SAM)
- Teensy: Teensy 4
- RISC-V / others: GD32VF103, CH32V003, Raspberry Pi Zero (baremetal)

(See source: src/templates/blink/mod.rs)

✅ Full Build & Flash support (BOARD_PRESETS)

These presets include build target, flash tool selection and are wired into the Build & Flash workflow in the UI:

- Arduino Uno (ATmega328P) — avrdude (note: nightly + avr-gcc often required)
- Arduino Nano (ATmega328P) — avrdude (note: nightly + avr-gcc often required)
- Arduino Mega 2560 (ATmega2560) — avrdude
- Arduino Leonardo (ATmega32u4) — avrdude
- Raspberry Pi Pico (RP2040) — picotool / UF2-style workflow
- ESP32 (Xtensa LX6) — esptool (espup/esp toolchain required)

(See source: src/core/board/presets.rs)

🗺️ Pinout viewer (boards with built-in pin data)

The pinout viewer contains curated pin maps for these boards:

- Arduino Uno (used also for Arduino Nano)
- micro:bit V2 (nRF52833)
- ESP32 (DevKit-style)
- STM32F4 Discovery

(See source: src/core/pinout.rs)

Prerequisites

- Rust toolchain (stable) and Cargo.
- For LSP: rust-analyzer (optional, recommended for completions/diagnostics).
- External tools depending on boards: avrdude, esptool.py, probe-rs, arm-none-eabi-objcopy/objcopy, nm/arm-none-eabi-nm, etc.

Build from source

1. Clone the repository to a folder and cd into it.
2. Build: cargo build --release
3. Run: cargo run --release

Quick start

1. Start OxIDE.
2. In Settings set your workspace directory.
3. Select a board in the Board picker.
4. (Optional) Click "Load Template" to generate a blink project for the selected board.
5. Edit files, click ▶ Build, then ⚡ Flash (or Build & Flash).

Contributing

- Bug reports and PRs welcome. Follow repository coding guidelines and run tests where provided.

License

Dual licensed: MIT OR Apache-2.0. See LICENSE-MIT and LICENSE-APACHE.

日本語版: [README.ja.md](README.ja.md)

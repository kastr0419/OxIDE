# OxIDE

> Arduino-IDE-style simplicity, for Rust embedded development.

[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE-MIT)
[![CI](https://github.com/your-username/oxide/actions/workflows/ci.yml/badge.svg)](https://github.com/your-username/oxide/actions/workflows/ci.yml)

OxIDE is a GUI IDE for writing embedded firmware **in Rust** — with the same
"select board → write code → flash" workflow that Arduino IDE made famous,
but targeting 27+ boards and the full Rust embedded ecosystem.

[日本語版 README](README.ja.md)

---

## Why OxIDE?

|  | Arduino IDE | VS Code + plugins | **OxIDE** |
|--|:-----------:|:-----------------:|:---------:|
| Language | C/C++ | Any | **Rust** |
| Setup complexity | Low | High | **Low** |
| LSP / autocomplete | ✗ | ✓ (manual) | **✓ built-in** |
| Serial monitor | ✓ | plugin | **✓ built-in** |
| Serial plotter | ✗ | plugin | **✓ built-in** |
| Pinout viewer | ✗ | ✗ | **✓ built-in** |
| ELF / SVD / RTT debug | ✗ | plugin | **✓ built-in** |
| Board support | Arduino-focused | Any | **27+ boards** |

---

## ✨ Features

- **Code Editor** — syntax highlighting + rust-analyzer LSP integration (autocomplete, diagnostics, go-to-definition)
- **One-click Build & Flash** — `cargo build` → avrdude / esptool / probe-rs, all wired up automatically
- **Serial Monitor** — connect, send/receive, configurable baud rate
- **Serial Plotter** — real-time graph of numeric serial output
- **Pinout Viewer** — interactive visual pin diagram for every supported board
- **ELF Analyzer** — inspect binary size, sections, and symbol table
- **SVD Register Viewer** — browse and decode peripheral registers from SVD files
- **RTT Debug Panel** — real-time transfer output via probe-rs (no serial cable needed)
- **Stack Analyzer** — call graph and stack usage estimation
- **Project Templates** — new-project scaffolding for all supported targets (blink and more)
- **Board Auto-detect** — USB VID/PID detection identifies connected boards automatically

---

## 📦 Supported Boards

### ✅ Implemented

| Board | CPU | Architecture | Flash Tool |
|-------|-----|-------------|-----------|
| Arduino Uno | ATmega328P | AVR 8-bit | avrdude |
| Arduino Nano | ATmega328P | AVR 8-bit | avrdude |
| Arduino Mega 2560 | ATmega2560 | AVR 8-bit | avrdude |
| Arduino Leonardo | ATmega32u4 | AVR 8-bit | avrdude |
| Raspberry Pi Pico | RP2040 (Cortex-M0+) | ARM 32-bit | picotool |
| ESP32 | Xtensa LX6 | Xtensa 32-bit | esptool |

### 🔜 Planned (next releases)

ESP32-S3, ESP32-C3, STM32F1/F4/H7/L4, nRF52840,
BBC micro:bit v2, Adafruit SAMD21/SAMD51,
Arduino Due, Teensy 4.0, Raspberry Pi Pico 2 — see [SUPPORTED_CPUS.md](SUPPORTED_CPUS.md) for full details.

### 🔬 Experimental

GD32VF103, CH32V003, nRF51822, BBC micro:bit v1, MSP430G2553

---

## 🛠 Prerequisites

| Tool | Required for | Install |
|------|-------------|---------|
| Rust (stable) | All targets | [rustup.rs](https://rustup.rs) |
| avrdude | Arduino / AVR | `winget install avrdude` / `apt install avrdude` |
| esptool | ESP32 series | `pip install esptool` |
| probe-rs | STM32, nRF, RP2040 | `cargo install probe-rs-tools` |
| rust-analyzer | LSP features | bundled or `rustup component add rust-analyzer` |

For AVR targets, a nightly toolchain is also required:
```sh
rustup toolchain install nightly
rustup component add rust-src --toolchain nightly
```

---

## 🚀 Build from Source

```sh
git clone https://github.com/your-username/oxide.git
cd oxide
cargo build --release
./target/release/oxide        # Linux
.\target\release\oxide.exe    # Windows
```

Requires Rust 1.70+ and a C linker (MSVC on Windows, gcc on Linux).

---

## ⚡ Quick Start

1. Launch OxIDE
2. **File → New Project** — choose your board and a template (e.g. Blink)
3. Write your Rust firmware in the editor
4. Select your board and serial port in the left panel
5. Click **Build & Flash** — done

---

## 🤝 Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines, coding conventions, and the commit message format.

---

## 📄 License

Dual-licensed under [MIT](LICENSE-MIT) or [Apache 2.0](LICENSE-APACHE) — your choice.

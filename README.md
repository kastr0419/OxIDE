# ALLoIDE — a small embedded Rust IDE

A cross-platform GUI IDE for writing embedded firmware in Rust, inspired by the simplicity of the Arduino IDE. Built with egui / eframe and focused on a clean, minimal workflow: edit, build, inspect, and flash.

Japanese README: [README.ja.md](README.ja.md)

## 🚀 Why ALLoIDE

- Designed for hobbyists and embedded Rust newcomers who want a lightweight, integrated editing/building/flashing workflow.
- Provides focused tooling for common microcontroller families (AVR, RP2040, ESP32, STM32, nRF, etc.).

## ✨ Implemented features

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

## 📋 Project templates

ALLoIDE provides blink/project templates for the following boards (templates generate a ready-to-build Cargo project):

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

## ✅ Full Build & Flash support

These presets include build target, flash tool selection and are wired into the Build & Flash workflow in the UI:

- Arduino Uno (ATmega328P) — avrdude (note: nightly + avr-gcc often required)
- Arduino Nano (ATmega328P) — avrdude (note: nightly + avr-gcc often required)
- Arduino Mega 2560 (ATmega2560) — avrdude
- Arduino Leonardo (ATmega32u4) — avrdude
- Raspberry Pi Pico (RP2040) — picotool / UF2-style workflow
- ESP32 (Xtensa LX6) — esptool (espup/esp toolchain required)

(See source: src/core/board/presets.rs)

## 🧪 Virtual MCU environment

Select any board in the Board picker and choose **OxIDE Virtual Board** as the port to exercise the Build → Flash → Serial workflow without hardware.

The virtual environment works with every preset currently shown in the Board picker:

- AVR: Arduino Uno / Nano / Mega 2560 / Leonardo
- Raspberry Pi: Pico / Pico 2 / Zero
- STM32: F1 / F4 / L4 / F7 / H7 / G0
- ESP32: ESP32 / S2 / S3 / C3 / C6 / H2
- Nordic / micro:bit: nRF51822 / nRF52840 / micro:bit V2
- SAM: SAMD21 / SAMD51 / Arduino Due
- Others: Teensy 4 / GD32VF103 / CH32V003

| Action | Virtual behavior |
|---|---|
| Build | Runs the real Cargo build for the selected board; its toolchain is still required |
| Flash | Checks that an artifact exists and simulates a successful write; no hardware is modified |
| Serial | Emits `sensor:0` through `sensor:99` periodically and replies to input with `echo:` |
| CPU and GPIO | Supported boards execute the real ELF and simulate the LED GPIO in Renode |
| Other peripherals | Not emulated |
| Hardware debugging | Not supported |

To simulate CPU/GPIO, install [Renode](https://renode.io/), add it to PATH, build the firmware, and click **CPU/GPIO Sim**. Renode timing is not identical to physical hardware.

Audit against official Renode platforms:

| ALLoIDE preset | CPU/GPIO | Renode model / reason |
|---|---|---|
| SAMD21 | Supported | `atsamd21j17d-aft.repl`, PA17 |
| STM32F1 | Supported | `stm32f103.repl`, PC13 |
| STM32F7 | Supported | `stm32f746.repl`, PB7 |
| STM32H7 | Supported | `stm32h743.repl`, PB14 |
| STM32G0 | Supported | `stm32g0.repl`, PA5 |
| nRF52840 | Supported | `nrf52840.repl`, P0.13 |
| SAMD51 | Unsupported | CPU platform has no GPIO model |
| Arduino Uno / Nano / Mega / Leonardo | Unsupported | No matching official AVR model |
| Raspberry Pi Pico / Pico 2 / Zero | Unsupported | No matching RP2040 / RP2350 / BCM2835 model |
| Arduino Due | Unsupported | No matching SAM3X8E model |
| STM32F4 / STM32L4 | Unsupported | No matching STM32F411 / STM32L476 model |
| nRF51822 / micro:bit V2 | Unsupported | No matching nRF51 / nRF52833 model |
| Teensy 4 | Unsupported | No matching i.MX RT1062 model |
| ESP32 / S2 / S3 / C3 / C6 / H2 | Unsupported | No matching ESP32 model |
| GD32VF103 / CH32V003 | Unsupported | No matching official MCU/GPIO model |

Unsupported boards keep the mock Flash and Serial workflow. Similar but different MCU models are intentionally not used.

## 🗺️ Pinout viewer

The pinout viewer contains curated pin maps for these boards:

- Arduino Uno (used also for Arduino Nano)
- micro:bit V2 (nRF52833)
- ESP32 (DevKit-style)
- STM32F4 Discovery

(See source: src/core/board/pinout.rs)

## 📦 Installation

### Windows — Installer (Recommended)

All-in-one installer that automatically sets up Rust and avrdude.

1. Download `OxIDE_Setup_*.exe` from the [Releases page](https://github.com/kastr0419/ALLoIDE/releases/latest)
2. Run it and follow the wizard
3. Launch **ALLoIDE** from the Start Menu or Desktop (the current installer shortcut is named **OxIDE**)

> Bundled: oxide.exe + rustup (auto-installs Rust) + avrdude v8.1

### Linux — One-liner

```bash
curl -sSf https://raw.githubusercontent.com/kastr0419/ALLoIDE/master/installer/install.sh | bash
```

Supported distributions: Ubuntu/Debian, Fedora/RHEL, Arch Linux, openSUSE

Options:

```bash
bash install.sh --prefix=/usr/local   # specify install prefix
bash install.sh --no-rust             # skip Rust installation
bash install.sh --no-tools            # skip avrdude installation
bash install.sh --version=v0.1.0      # specify version
```

### Portable (Windows / Linux)

Download an archive from the [Releases page](https://github.com/kastr0419/ALLoIDE/releases/latest):

| File | OS |
|---|---|
| `oxide-windows-x86_64.zip` | Windows 64-bit |
| `oxide-linux-x86_64.tar.gz` | Linux 64-bit |

### Build from Source

```sh
git clone https://github.com/kastr0419/ALLoIDE.git
cd ALLoIDE
cargo build --release
./target/release/oxide          # Linux
.\target\release\oxide.exe      # Windows
```

---

## 🛠 Prerequisites

### 1. Set up Rust

ALLoIDE requires Rust both to **run itself** (build from source) and to **compile your firmware**.

```sh
# Install rustup (the Rust toolchain manager)
# Windows: download and run https://rustup.rs
# Linux/macOS:
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# After install, make sure cargo is in your PATH, then verify:
rustc --version   # e.g. rustc 1.78.0 (...)
cargo --version   # e.g. cargo 1.78.0 (...)
```

> **Windows note:** You also need a C linker. Install
> [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
> and select the **C++ build tools** workload (includes MSVC + Windows SDK).
> Linux/macOS users need `gcc` (`sudo apt install build-essential` / `xcode-select --install`).

### Always required

| Tool | Install |
|------|---------|
| **Rust (stable)** | see above |
| **C linker** | Windows: MSVC (Visual Studio Build Tools) · Linux/macOS: `gcc` |

### Per-board tools

**Arduino / AVR** (Uno, Nano, Mega, Leonardo)
```sh
# Nightly toolchain + AVR source (required to cross-compile for AVR)
rustup toolchain install nightly
rustup component add rust-src --toolchain nightly

# avr-gcc (compiler backend)
# Windows: install WinAVR or via MSYS2: pacman -S avr-gcc avr-libc
# Linux:   sudo apt install gcc-avr binutils-avr avr-libc
# macOS:   brew install avr-gcc

# avrdude (flash tool)
# Windows: winget install avrdude   or  https://github.com/avrdudes/avrdude/releases
# Linux:   sudo apt install avrdude
```

**Raspberry Pi Pico (RP2040)**
```sh
rustup target add thumbv6m-none-eabi
# picotool for flashing: https://github.com/raspberrypi/picotool
# Or use UF2 drag-and-drop: hold BOOTSEL on power-on, copy the .uf2 file
```

**ESP32**
```sh
# espup installs the Xtensa Rust toolchain + target
cargo install espup
espup install

# esptool.py (flash tool)
pip install esptool
```

**STM32 / nRF52840 / DAPLink boards**
```sh
# probe-rs (flash/debug via J-Link, ST-Link, CMSIS-DAP)
cargo install probe-rs-tools

# Target triple — example for STM32F4:
rustup target add thumbv7em-none-eabihf

# ELF conversion (for .hex/.bin output, used by DAPLink flash)
cargo install cargo-binutils
rustup component add llvm-tools-preview
# Alternative: install arm-none-eabi-binutils from https://developer.arm.com/downloads
```

### Optional tools

| Tool | Purpose |
|------|---------|
| `rust-analyzer` | LSP features in the editor (completion, diagnostics). Install via `rustup component add rust-analyzer` or from [rust-analyzer.github.io](https://rust-analyzer.github.io) |
| `nm` / `arm-none-eabi-nm` | Stack analyzer panel (estimates stack usage from symbol table) |

## 🚀 Quick start

1. Start ALLoIDE (current executable/window name: **OxIDE**).
2. In **Settings**, set your workspace directory.
3. Select a board in the **Board picker**.
4. (Optional) Click **Load Template** to generate a blink project for the selected board.
5. Edit files, click ▶ **Build**, then ⚡ **Flash** (or **Build & Flash**).

## 🤝 Contributing

Bug reports and PRs are welcome. Follow repository coding guidelines and run tests where provided.

## 📄 License

Dual licensed: MIT OR Apache-2.0. See [LICENSE-MIT](LICENSE-MIT) and [LICENSE-APACHE](LICENSE-APACHE).

Japanese README: [README.ja.md](README.ja.md)

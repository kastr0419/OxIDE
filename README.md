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

## 📦 Installation

### Windows — インストーラー（推奨）

Rust・avrdude を自動セットアップする all-in-one インストーラーです。

1. [Releases ページ](https://github.com/kastr0419/OxIDE/releases/latest) から `OxIDE_Setup_*.exe` をダウンロード
2. 実行してウィザードに従う
3. スタートメニュー / デスクトップから **OxIDE** を起動

> 同梱: oxide.exe + rustup (Rust 自動インストール) + avrdude v8.1

### Linux — ワンライナー

```bash
curl -sSf https://raw.githubusercontent.com/kastr0419/OxIDE/master/installer/install.sh | bash
```

対応ディストリビューション: Ubuntu/Debian, Fedora/RHEL, Arch Linux, openSUSE

オプション:

```bash
bash install.sh --prefix=/usr/local   # インストール先を指定
bash install.sh --no-rust             # Rust インストールをスキップ
bash install.sh --no-tools            # avrdude インストールをスキップ
bash install.sh --version=v0.1.0      # バージョン指定
```

### ポータブル（Windows / Linux 共通）

[Releases ページ](https://github.com/kastr0419/OxIDE/releases/latest) からアーカイブをダウンロード:

| ファイル | OS |
|---|---|
| `oxide-windows-x86_64.zip` | Windows 64-bit |
| `oxide-linux-x86_64.tar.gz` | Linux 64-bit |

### ソースからビルド

```sh
git clone https://github.com/kastr0419/OxIDE.git
cd OxIDE
cargo build --release
./target/release/oxide          # Linux
.\target\release\oxide.exe      # Windows
```

---

## 🛠 Prerequisites

### 1. Set up Rust

OxIDE requires Rust both to **run itself** (build from source) and to **compile your firmware**.

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

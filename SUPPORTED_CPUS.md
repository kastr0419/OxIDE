# マイコンCPU対応一覧 / Supported MCU CPU Reference

> ALLoIDE が対応する（または将来対応予定の）マイコンCPUアーキテクチャ一覧。
> 調査日: 2026-04-16

---

## 凡例

| 記号 | 意味 |
|------|------|
| ✅ 実装済み | 現バージョンで選択・コンパイル・書き込みが可能 |
| 🔜 実装予定 | 次バージョンで対応予定 |
| 🔬 実験的 | 動作確認中・コミュニティサポート |
| ❌ 未対応 | Rust コンパイラのサポートなし |

---

## 1. 現在の実装状況サマリー

| ステータス | 件数 |
|-----------|------|
| ✅ 実装済み | 4 ボード |
| 🔜 実装予定 | 16 ボード |
| 🔬 実験的 | 5 ボード |
| ❌ 未対応 | PIC/8051/SuperH 等 |

---

## 2. CPUアーキテクチャ別 Rust ターゲット対応表

### ARM Cortex-M シリーズ

| CPUコア | Rust ターゲットトリプル | Tier | 代表チップ | 実装状況 |
|---------|----------------------|------|-----------|---------|
| Cortex-M0 / M0+ | `thumbv6m-none-eabi` | Tier 2 | nRF51, RP2040, SAMD21 | 🔜 実装予定 |
| Cortex-M3 | `thumbv7m-none-eabi` | Tier 2 | STM32F1, SAM3X | 🔜 実装予定 |
| Cortex-M4 (FPUなし) | `thumbv7em-none-eabi` | Tier 2 | STM32F3 | 🔜 実装予定 |
| Cortex-M4F (FPUあり) | `thumbv7em-none-eabihf` | Tier 2 | STM32F4, nRF52, SAMD51 | ✅ 実装済み |
| Cortex-M7 | `thumbv7em-none-eabihf` | Tier 2 | STM32F7, STM32H7, IMXRT | 🔜 実装予定 |
| Cortex-M23 | `thumbv8m.base-none-eabi` | Tier 2-3 | STM32G0, M23系 | 🔬 実験的 |
| Cortex-M33 | `thumbv8m.main-none-eabihf` | Tier 2-3 | nRF9160, STM32L5, RP2350 | 🔜 実装予定 |
| Cortex-M55 / M85 | `thumbv8m.main-none-eabihf` | Tier 3 | AI向けSoC | 🔬 実験的 |

### AVR (8-bit)

| CPUコア | Rust ターゲットトリプル | Tier | 代表チップ | 実装状況 |
|---------|----------------------|------|-----------|---------|
| AVR ATmega328P | `avr-unknown-gnu-atmega328` | Tier 3 | Arduino Uno/Nano | ✅ 実装済み |
| AVR ATmega2560 | `avr-unknown-gnu-atmega2560` | Tier 3 | Arduino Mega | 🔜 実装予定 |
| AVR ATmega32u4 | `avr-unknown-gnu-atmega32u4` | Tier 3 | Arduino Leonardo | 🔜 実装予定 |
| AVR ATtiny | `avr-unknown-gnu-attiny*` | Tier 3 | ATtiny85 等 | 🔬 実験的 |

### Xtensa (ESP32 系)

| CPUコア | Rust ターゲットトリプル | Tier | 代表チップ | 実装状況 |
|---------|----------------------|------|-----------|---------|
| Xtensa LX6 | `xtensa-esp32-none-elf` | Tier 3* | ESP32 | ✅ 実装済み |
| Xtensa LX7 | `xtensa-esp32s2-none-elf` | Tier 3* | ESP32-S2 | 🔜 実装予定 |
| Xtensa LX7 (S3) | `xtensa-esp32s3-none-elf` | Tier 3* | ESP32-S3 | 🔜 実装予定 |

> *esp-rs カスタムツールチェーンが必要 (`espup` でインストール)

### RISC-V

| CPUコア | Rust ターゲットトリプル | Tier | 代表チップ | 実装状況 |
|---------|----------------------|------|-----------|---------|
| RV32IMC | `riscv32imc-unknown-none-elf` | Tier 2 | ESP32-C3 | 🔜 実装予定 |
| RV32IMAC | `riscv32imac-unknown-none-elf` | Tier 2 | GD32VF103, ESP32-C6, CH32V | 🔜 実装予定 |
| RV32EC | `riscv32imc-unknown-none-elf` | Tier 3 | CH32V003 | 🔬 実験的 |

### その他

| CPUコア | Rust ターゲットトリプル | Tier | 代表チップ | 実装状況 |
|---------|----------------------|------|-----------|---------|
| MSP430 | `msp430-none-elf` | Tier 3 | MSP430G2553 | 🔬 実験的 |
| PIC (8/16/32) | なし | ❌ 未対応 | PIC16/18/32 | ❌ 未対応 |
| 8051 / 8052 | なし | ❌ 未対応 | 8051系 | ❌ 未対応 |
| SuperH / Renesas RX | なし | ❌ 未対応 | SH/RX系 | ❌ 未対応 |
| MIPS (PIC32) | 困難 | 実験的 | PIC32MX | ❌ 未対応 |

---

## 3. ボード別詳細一覧

### ✅ 実装済みボード

| ボード | CPU | アーキテクチャ | ターゲット | フラッシュツール | デフォルトBaud |
|--------|-----|--------------|----------|----------------|--------------|
| Arduino Uno | ATmega328P | AVR 8-bit | `avr-atmega328p` | avrdude | 115200 |
| Arduino Nano | ATmega328P | AVR 8-bit | `avr-atmega328p` | avrdude | 115200 |
| ESP32 | Xtensa LX6 | Xtensa 32-bit | `xtensa-esp32-none-elf` | esptool.py | 115200 |
| STM32F4xx | Cortex-M4F | ARM 32-bit | `thumbv7em-none-eabihf` | probe-rs | 115200 |

### 🔜 実装予定ボード (優先度順)

| 優先度 | ボード | CPU | ターゲット | フラッシュツール |
|--------|--------|-----|----------|----------------|
| 1 | Raspberry Pi Pico | RP2040 (Cortex-M0+) | `thumbv6m-none-eabi` | picotool / probe-rs / UF2 |
| 2 | Raspberry Pi Pico 2 | RP2350 (Cortex-M33) | `thumbv8m.main-none-eabihf` | picotool / probe-rs |
| 3 | ESP32-S3 | Xtensa LX7 | `xtensa-esp32s3-none-elf` | esptool.py |
| 4 | ESP32-C3 | RISC-V RV32IMC | `riscv32imc-unknown-none-elf` | esptool.py |
| 5 | ESP32-C6 | RISC-V RV32IMAC | `riscv32imac-unknown-none-elf` | esptool.py |
| 6 | STM32F1xx | Cortex-M3 | `thumbv7m-none-eabi` | probe-rs / st-flash |
| 7 | STM32H7xx | Cortex-M7 | `thumbv7em-none-eabihf` | probe-rs / st-flash |
| 8 | STM32L4xx | Cortex-M4F | `thumbv7em-none-eabihf` | probe-rs / st-flash |
| 9 | nRF52840 | Cortex-M4F | `thumbv7em-none-eabihf` | probe-rs / nrfjprog |
| 10 | BBC micro:bit v2 | nRF52833 (Cortex-M4F) | `thumbv7em-none-eabihf` | probe-rs / UF2 |
| 11 | Arduino Mega | ATmega2560 | `avr-unknown-gnu-atmega2560` | avrdude |
| 12 | Arduino Leonardo | ATmega32u4 | `avr-unknown-gnu-atmega32u4` | avrdude |
| 13 | Adafruit SAMD21 | Cortex-M0+ | `thumbv6m-none-eabi` | bossac / UF2 |
| 14 | Adafruit SAMD51 | Cortex-M4F | `thumbv7em-none-eabihf` | bossac / UF2 |
| 15 | Arduino Due | SAM3X8E (Cortex-M3) | `thumbv7m-none-eabi` | bossac |
| 16 | Teensy 4.0 | IMXRT1062 (Cortex-M7) | `thumbv7em-none-eabihf` | teensy_loader_cli |

### 🔬 実験的サポートボード

| ボード | CPU | ターゲット | 注意事項 |
|--------|-----|----------|---------|
| GD32VF103 | RISC-V RV32IMAC | `riscv32imac-unknown-none-elf` | gd32vf103-hal (コミュニティ) |
| CH32V003 | RISC-V RV32EC | `riscv32imc-unknown-none-elf` | ベンダツール必要 |
| nRF51822 | Cortex-M0 | `thumbv6m-none-eabi` | 古い世代、限定サポート |
| BBC micro:bit v1 | nRF51 (Cortex-M0) | `thumbv6m-none-eabi` | 古い世代 |
| MSP430G2553 | MSP430 | `msp430-none-elf` | mspdebug 必要 |

---

## 4. ツールチェーン要件

| アーキテクチャ | 必要なツール | インストール方法 |
|--------------|------------|----------------|
| ARM Cortex-M 全般 | `rustup target add thumbv*` | 標準 rustup |
| AVR | `avr-gcc`, `avr-hal` | nightly + `cargo +nightly build -Z build-std` |
| Xtensa (ESP32) | `espup` | `cargo install espup && espup install` |
| RISC-V | `rustup target add riscv32*` | 標準 rustup |
| MSP430 | `msp430-none-elf gcc` | 別途インストール |

---

## 5. フラッシュツール一覧

| ツール | 対応アーキテクチャ | Windows | Linux | 入手方法 |
|--------|-----------------|---------|-------|---------|
| avrdude | AVR | ✅ | ✅ | `winget install avrdude` / `apt install avrdude` |
| esptool.py | ESP32 系全て | ✅ | ✅ | `pip install esptool` |
| probe-rs | Cortex-M / RISC-V | ✅ | ✅ | `cargo install probe-rs-tools` |
| st-flash | STM32 | ✅ | ✅ | stlink パッケージ |
| picotool | RP2040/RP2350 | ✅ | ✅ | CMake ビルドまたはパッケージ |
| bossac | SAMD21/51, Due | ✅ | ✅ | Arduino IDE 付属 |
| nrfjprog | nRF 系 | ✅ | ✅ | Nordic SDK |
| teensy_loader_cli | Teensy 系 | ✅ | ✅ | PJRC 公式 |
| mspdebug | MSP430 | 🔬 | ✅ | パッケージマネージャ |

---

## 6. USB VID/PID 自動判別テーブル

| VID | PID | ボード / チップ |
|-----|-----|----------------|
| 0x2341 | 0x0043 | Arduino Uno R3 (genuine) |
| 0x2341 | 0x0001 | Arduino Uno (genuine, old) |
| 0x1A86 | 0x7523 | Arduino (CH340 クローン) |
| 0x1A86 | 0x55D4 | Arduino Nano (CH9102) |
| 0x0403 | 0x6001 | Arduino/Teensy (FTDI FT232R) |
| 0x10C4 | 0xEA60 | ESP32 (CP2102) |
| 0x303A | 0x1001 | ESP32-S3 (built-in USB) |
| 0x303A | 0x0002 | ESP32-C3 (built-in USB) |
| 0x0483 | 0x374B | STM32 ST-Link/V2-1 |
| 0x0483 | 0x3748 | STM32 ST-Link/V2 |
| 0x0483 | 0x374F | STM32 ST-Link/V3 |
| 0x0483 | 0x5740 | STM32 CDC (Virtual COM) |
| 0x2E8A | 0x000A | Raspberry Pi Pico (RP2040) |
| 0x2E8A | 0x000F | Raspberry Pi Pico 2 (RP2350) |
| 0x2341 | 0x8036 | Arduino Leonardo |
| 0x2341 | 0x0010 | Arduino Mega 2560 |
| 0x239A | 0x800B | Adafruit SAMD21 (Feather) |
| 0x239A | 0x8022 | Adafruit SAMD51 (Feather M4) |
| 0x1915 | 0x521F | nRF52840 (Nordic USB) |

---

## 7. 参考資料

- [The Embedded Rust Book](https://docs.rust-embedded.org/book/)
- [Rust Platform Support (Tier一覧)](https://doc.rust-lang.org/nightly/rustc/platform-support.html)
- [esp-rs (ESP32 Rust サポート)](https://github.com/esp-rs)
- [avr-hal (AVR Rust サポート)](https://github.com/Rahix/avr-hal)
- [probe-rs (デバッグ・フラッシュツール)](https://probe.rs/)
- [embassy (非同期組み込みフレームワーク)](https://embassy.dev/)

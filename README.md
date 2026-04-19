# Rust Embedded IDE

Arduino IDEのRust版 — RustでマイコンをプログラムできるGUI IDE

## ✨ 機能
- コードエディタ（Rustシンタックスハイライト）
- 複数ボード対応（Arduino Uno/Nano、ESP32、STM32F4）
- コンパイル（cargo build）
- マイコンへの書き込み（avrdude / esptool / probe-rs）
- シリアルモニタ（送受信）
- 新規プロジェクトテンプレート生成

## 🛠 対応環境
- OS: Windows / Linux
- Rust: 1.70+
- GUI: egui (純Rust)

## 🚀 使い方
（ビルド・起動手順）

## 📦 対応ボード
| ボード | ターゲット | 書き込みツール |
|--------|-----------|--------------|
| Arduino Uno | avr-atmega328p | avrdude |
| Arduino Nano | avr-atmega328p | avrdude |
| ESP32 | xtensa-esp32-none-elf | esptool |
| STM32F4 | thumbv7em-none-eabihf | probe-rs |

## 🤝 コントリビューション
CONTRIBUTING.md を参照してください。

## 📄 ライセンス
本プロジェクトは MIT License または Apache License 2.0 のいずれかの条件で利用できます。

- [MIT License](LICENSE-MIT)
- [Apache License 2.0](LICENSE-APACHE)

ライセンスバッジ:
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE-MIT)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE-APACHE)
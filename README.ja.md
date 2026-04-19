# OxIDE

> Arduino IDE の「シンプルさ」を、Rust 組み込み開発へ。

[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE-MIT)
[![CI](https://github.com/your-username/oxide/actions/workflows/ci.yml/badge.svg)](https://github.com/your-username/oxide/actions/workflows/ci.yml)

OxIDE は **Rust** でマイコンのファームウェアを書くための GUI IDE です。  
Arduino IDE が広めた「ボードを選んで → コードを書いて → 焼く」というシンプルなワークフローを、Rust 組み込み開発と 27 種以上のボードに対応して実現します。

[English README](README.md)

---

## なぜ OxIDE？

|  | Arduino IDE | VS Code + 拡張 | **OxIDE** |
|--|:-----------:|:-------------:|:---------:|
| 言語 | C/C++ | 何でも | **Rust** |
| セットアップの手間 | 少ない | 多い | **少ない** |
| LSP / オートコンプリート | ✗ | ✓（手動設定） | **✓ 組み込み済み** |
| シリアルモニタ | ✓ | プラグイン | **✓ 組み込み済み** |
| シリアルプロッタ | ✗ | プラグイン | **✓ 組み込み済み** |
| ピン配置ビューア | ✗ | ✗ | **✓ 組み込み済み** |
| ELF / SVD / RTT デバッグ | ✗ | プラグイン | **✓ 組み込み済み** |
| 対応ボード数 | Arduino 中心 | 何でも | **27 種以上** |

---

## ✨ 機能

- **コードエディタ** — シンタックスハイライト + rust-analyzer LSP 統合（補完・エラー表示・定義ジャンプ）
- **ワンクリック ビルド & 書き込み** — `cargo build` から avrdude / esptool / probe-rs まで自動連携
- **シリアルモニタ** — 接続・送受信・ボーレート設定
- **シリアルプロッタ** — シリアル出力の数値データをリアルタイムグラフ表示
- **ピン配置ビューア** — 対応全ボードのインタラクティブなピン図
- **ELF アナライザ** — バイナリサイズ・セクション・シンボルテーブルの確認
- **SVD レジスタビューア** — SVD ファイルからペリフェラルレジスタを参照・デコード
- **RTT デバッグパネル** — probe-rs 経由のリアルタイム転送出力（シリアルケーブル不要）
- **スタックアナライザ** — コールグラフとスタック使用量の推定
- **プロジェクトテンプレート** — 全対応ターゲット向けの新規プロジェクト雛形（Blink 等）
- **ボード自動検出** — USB VID/PID による接続ボードの自動認識

---

## 📦 対応ボード

## 📦 対応ボード

### ✅ フルサポート（ビルド & 書き込み）

| ボード | CPU | アーキテクチャ | 書き込みツール |
|--------|-----|-------------|--------------|
| Arduino Uno | ATmega328P | AVR 8-bit | avrdude |
| Arduino Nano | ATmega328P | AVR 8-bit | avrdude |
| Arduino Mega 2560 | ATmega2560 | AVR 8-bit | avrdude |
| Arduino Leonardo | ATmega32u4 | AVR 8-bit | avrdude |
| Raspberry Pi Pico | RP2040 (Cortex-M0+) | ARM 32-bit | picotool |
| ESP32 | Xtensa LX6 | Xtensa 32-bit | esptool |

### 📋 プロジェクトテンプレート（27ボード）

「New Project」でのテンプレート生成は以下のすべてのボードに対応しています：

**AVR** — Uno, Nano, Mega, Leonardo  
**ARM Cortex-M** — Raspberry Pi Pico, Pico 2, STM32F1/F4/L4/F7/H7/G0, micro:bit V2, nRF52840, nRF51822, SAMD21, SAMD51, Arduino Due, Teensy 4  
**ESP32** — ESP32, S2, S3, C3, C6, H2  
**RISC-V** — GD32VF103, CH32V003  
**その他** — Raspberry Pi Zero（ベアメタル）

### 🗺️ ピン配置ビューア（4ボード）

Arduino Uno · ESP32 · BBC micro:bit V2 · STM32F4xx

### 🔜 今後の対応予定

STM32、nRF52840、Raspberry Pi Pico 2、ESP32-S3/C3 のビルド & 書き込みサポート — 詳細は [SUPPORTED_CPUS.md](SUPPORTED_CPUS.md) を参照。

---

## 🛠 前提ツール

| ツール | 用途 | インストール |
|--------|------|-------------|
| Rust (stable) | 全ターゲット | [rustup.rs](https://rustup.rs) |
| avrdude | Arduino / AVR | `winget install avrdude` / `apt install avrdude` |
| esptool | ESP32 系 | `pip install esptool` |
| probe-rs | STM32, nRF, RP2040 | `cargo install probe-rs-tools` |
| rust-analyzer | LSP 機能 | 同梱 or `rustup component add rust-analyzer` |

AVR ターゲットを使う場合は nightly ツールチェーンも必要です：

```sh
rustup toolchain install nightly
rustup component add rust-src --toolchain nightly
```

---

## 🚀 ソースからビルド

```sh
git clone https://github.com/your-username/oxide.git
cd oxide
cargo build --release
./target/release/oxide        # Linux
.\target\release\oxide.exe    # Windows
```

Rust 1.70 以上と C リンカ（Windows: MSVC、Linux: gcc）が必要です。

---

## ⚡ クイックスタート

1. OxIDE を起動
2. **File → New Project** でボードとテンプレート（例: Blink）を選択
3. エディタで Rust ファームウェアを編集
4. 左パネルでボードとシリアルポートを選択
5. **Build & Flash** をクリック — 完了！

---

## 🤝 コントリビューション

コーディング規約・コミットメッセージ形式など、詳細は [CONTRIBUTING.md](CONTRIBUTING.md) を参照してください。

---

## 📄 ライセンス

[MIT](LICENSE-MIT) または [Apache 2.0](LICENSE-APACHE) のデュアルライセンスです。いずれかを選択して利用できます。

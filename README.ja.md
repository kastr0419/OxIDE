# OxIDE — 小さな組み込み Rust IDE

Rust で組み込みファームウェアを書くためのクロスプラットフォーム GUI IDE。Arduino IDE のシンプルなワークフローに触発され、egui / eframe 上に最小限で使いやすい編集→ビルド→書き込みの体験を提供します。

English: [README.md](README.md)

## 🚀 OxIDE の特徴

- 初心者やホビイスト向けに軽量で統合された開発体験を提供します。
- AVR、RP2040、ESP32、STM32、nRF など主要 MCU ファミリ向けの実用的なツール群を備えます。

## ✨ 実装済みの機能

- エディタ & ファイルエクスプローラ：開く／保存、ワークスペース一覧、複数タブ。
- ビルド：Cargo を実行（プリセットに基づくターゲット注入、memory.x の自動生成）。
- 書き込み：avrdude / esptool / probe-rs / DAPLink へのコピーなど、対応バックエンドへの連携（外部ツールが必要）
- シリアルモニタ & プロッタ：シリアル接続、送受信、数値データの簡易プロット
- ボード選択 & 自動検出：USB VID/PID / probe-rs / esptool による検出
- ピン配置ビューア：図と表でピンを可視化
- ELF アナライザ：セクション・シンボル一覧（object クレート利用）
- スタックアナライザ：nm ベースのスタック推定
- SVD ビューア：.svd ファイルの読み込みとレジスタ閲覧
- Rust Analyzer (LSP) クライアント：補完・診断のために rust-analyzer を起動して利用可能
- デバッグ UI のスケルトン（RTT 等）：外部デバッグツールと組み合わせて利用します

## 📋 プロジェクトテンプレート

以下のボード向けにテンプレートを提供します（ビルド可能な Cargo プロジェクトを生成）：

- AVR: Arduino Uno, Arduino Nano, Arduino Mega, Arduino Leonardo
- RP2040: Raspberry Pi Pico, Raspberry Pi Pico 2
- STM32: STM32F1, STM32F4, STM32L4, STM32F7, STM32H7, STM32G0
- micro:bit V2
- ESP32 系: ESP32, ESP32-S2, ESP32-S3, ESP32-C3, ESP32-C6, ESP32-H2
- Nordic: nRF52840, nRF51822
- SAMD 系: SAMD21, SAMD51, Arduino Due
- Teensy 4
- RISC-V / その他: GD32VF103, CH32V003, Raspberry Pi Zero (bare-metal)

(ソース参照: src/templates/blink/mod.rs)

## ✅ フル ビルド & 書き込み対応

UI の Build & Flash ワークフローに統合されているプリセット:

- Arduino Uno (ATmega328P) — avrdude（注: AVR 向けに nightly + avr-gcc が必要になる場合あり）
- Arduino Nano (ATmega328P) — avrdude
- Arduino Mega 2560 (ATmega2560) — avrdude
- Arduino Leonardo (ATmega32u4) — avrdude
- Raspberry Pi Pico (RP2040) — picotool / UF2 ワークフロー
- ESP32 (Xtensa LX6) — esptool（espup 等のセットアップが必要）

(ソース参照: src/core/board/presets.rs)

## 🗺️ ピン配置ビューア

次のボードに対してピンデータが内蔵されています：

- Arduino Uno（Arduino Nano は Uno のデータを共有）
- micro:bit V2 (nRF52833)
- ESP32 (DevKit スタイル)
- STM32F4 Discovery

(ソース参照: src/core/pinout.rs)

## 📦 インストール

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

## 🛠 前提ツール

### 1. Rust 環境のセットアップ

OxIDE は**自身のビルド**にも、**ファームウェアのコンパイル**にも Rust が必要です。

```sh
# rustup（Rust ツールチェーンマネージャ）をインストール
# Windows: https://rustup.rs からインストーラをダウンロードして実行
# Linux/macOS:
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# インストール後、PATH を反映してバージョンを確認
rustc --version   # 例: rustc 1.78.0 (...)
cargo --version   # 例: cargo 1.78.0 (...)
```

> **Windows の注意:** C リンカも必要です。
> [Visual Studio Build Tools](https://visualstudio.microsoft.com/ja/visual-cpp-build-tools/) をインストールし、
> **C++ によるデスクトップ開発** ワークロードを選択してください（MSVC + Windows SDK が含まれます）。
> Linux / macOS は `gcc`（`sudo apt install build-essential` / `xcode-select --install`）で対応できます。

### 共通（必須）

| ツール | インストール |
|--------|-------------|
| **Rust (stable)** | 上記参照 |
| **C リンカ** | Windows: MSVC (Visual Studio Build Tools) · Linux/macOS: `gcc` |

### ボード別ツール

**Arduino / AVR**（Uno / Nano / Mega / Leonardo）
```sh
# nightly ツールチェーン + AVR ソース（AVR クロスコンパイルに必須）
rustup toolchain install nightly
rustup component add rust-src --toolchain nightly

# avr-gcc（コンパイラバックエンド）
# Windows: WinAVR または MSYS2: pacman -S avr-gcc avr-libc
# Linux:   sudo apt install gcc-avr binutils-avr avr-libc
# macOS:   brew install avr-gcc

# avrdude（書き込みツール）
# Windows: winget install avrdude  または  https://github.com/avrdudes/avrdude/releases
# Linux:   sudo apt install avrdude
```

**Raspberry Pi Pico (RP2040)**
```sh
rustup target add thumbv6m-none-eabi
# picotool: https://github.com/raspberrypi/picotool
# または UF2 ドラッグ＆ドロップ: BOOTSEL を押しながら電源を入れ .uf2 をコピー
```

**ESP32**
```sh
# espup が Xtensa 向け Rust ツールチェーンとターゲットをインストール
cargo install espup
espup install

# esptool.py（書き込みツール）
pip install esptool
```

**STM32 / nRF52840 / DAPLink 対応ボード**
```sh
# probe-rs（J-Link / ST-Link / CMSIS-DAP 経由の書き込み・デバッグ）
cargo install probe-rs-tools

# ターゲットの例（STM32F4）
rustup target add thumbv7em-none-eabihf

# ELF 変換（DAPLink 書き込み用 .hex/.bin 生成）
cargo install cargo-binutils
rustup component add llvm-tools-preview
# 代替: https://developer.arm.com/downloads から arm-none-eabi-binutils を導入
```

### 任意ツール

| ツール | 用途 |
|--------|------|
| `rust-analyzer` | エディタの LSP 機能（補完・診断）。`rustup component add rust-analyzer` または [rust-analyzer.github.io](https://rust-analyzer.github.io) |
| `nm` / `arm-none-eabi-nm` | スタックアナライザパネル（シンボルテーブルからスタック使用量を推定） |

## 🚀 クイックスタート

1. OxIDE を起動
2. **Settings** でワークスペースを設定
3. **Board picker** でボードを選択
4. （任意）**Load Template** で Blink プロジェクトを生成
5. 編集 → ▶ **Build** → ⚡ **Flash**

## 🤝 貢献

バグ報告・PR 歓迎。詳細は [CONTRIBUTING.md](CONTRIBUTING.md) を参照してください。

## 📄 ライセンス

MIT または Apache-2.0 のデュアルライセンスです。[LICENSE-MIT](LICENSE-MIT) / [LICENSE-APACHE](LICENSE-APACHE) を参照してください。

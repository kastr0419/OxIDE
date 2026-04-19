# OxIDE — 小さな組み込み Rust IDE

Rust で組み込みファームウェアを書くためのクロスプラットフォーム GUI IDE。Arduino IDE のシンプルなワークフローに触発され、egui / eframe 上に最小限で使いやすい編集→ビルド→書き込みの体験を提供します。

English: [README.md](README.md)

🚀 OxIDE の特徴

- 初心者やホビイスト向けに軽量で統合された開発体験を提供します。
- AVR、RP2040、ESP32、STM32、nRF など主要 MCU ファミリ向けの実用的なツール群を備えます。

✨ 実装済みの機能（ソースに基づく正確な一覧）

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

📋 プロジェクトテンプレート（Blink）

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

✅ フル ビルド & 書き込み対応（BOARD_PRESETS）

UI の Build & Flash ワークフローに統合されているプリセット:

- Arduino Uno (ATmega328P) — avrdude（注: AVR 向けに nightly + avr-gcc が必要になる場合あり）
- Arduino Nano (ATmega328P) — avrdude
- Arduino Mega 2560 (ATmega2560) — avrdude
- Arduino Leonardo (ATmega32u4) — avrdude
- Raspberry Pi Pico (RP2040) — picotool / UF2 ワークフロー
- ESP32 (Xtensa LX6) — esptool（espup 等のセットアップが必要）

(ソース参照: src/core/board/presets.rs)

🗺️ ピン配置ビューア（組み込みデータあり）

次のボードに対してピンデータが内蔵されています：

- Arduino Uno（Arduino Nano は Uno のデータを共有）
- micro:bit V2 (nRF52833)
- ESP32 (DevKit スタイル)
- STM32F4 Discovery

(ソース参照: src/core/pinout.rs)

前提ツール

- Rust（stable）と Cargo
- LSP 利用時は rust-analyzer（任意）
- ボードや解析機能に応じて外部ツールが必要: avrdude, esptool.py, probe-rs, objcopy, nm 等

ソースからビルド

1. リポジトリをクローンしてカレントに移動
2. `cargo build --release`
3. `cargo run --release`

クイックスタート

1. OxIDE を起動
2. Settings でワークスペースを設定
3. Board picker でボードを選択
4. （任意）Load Template で Blink プロジェクトを生成
5. 編集 → ▶ Build → ⚡ Flash

貢献

バグ報告・PR歓迎。詳細は CONTRIBUTING.md を参照してください。

ライセンス

MIT または Apache-2.0 のデュアルライセンスです。

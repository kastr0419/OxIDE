# 要求仕様書 — rust-embedded-ide

> バージョン: 0.1  
> 作成日: 2026-04-19  
> ライセンス: MIT OR Apache-2.0

---

## 1. プロジェクト概要

**rust-embedded-ide** は、Rust でマイコンをプログラムするための GUI 統合開発環境（IDE）です。
Arduino IDE を参考に、ワンクリックでコンパイル・書き込み・シリアルモニタができる操作性を目指しつつ、
Rust の強力なエコシステム（probe-rs、rust-analyzer 等）との統合を提供します。

### 主な想定ユーザー

| ユーザー層 | 説明 |
|-----------|------|
| 組み込み Rust 入門者 | Arduino 経験者で Rust 組み込みを始めたい人 |
| ホビイスト | GUI で手軽にマイコン開発したい人 |
| 教育機関 | Rust 組み込みの授業・ワークショップ環境 |

---

## 2. 対応環境

| 項目 | 要件 |
|------|------|
| OS | Windows 10/11、Linux（Ubuntu 20.04 以降） |
| Rust | 1.70 以降（AVR は nightly 必須） |
| GUI | ネイティブ（egui / eframe 0.31） |
| アーキテクチャ | x86-64 |

---

## 3. 対応ボード

### 3.1 AVR（avr-gcc + avrdude 必須）

| ボード | MCU | Flash | RAM | 書き込みツール |
|--------|-----|-------|-----|--------------|
| Arduino Uno | ATmega328P | 32 KB | 2 KB | avrdude |
| Arduino Nano | ATmega328P | 32 KB | 2 KB | avrdude |
| Arduino Mega 2560 | ATmega2560 | 256 KB | 8 KB | avrdude |
| Arduino Leonardo | ATmega32U4 | 32 KB | 2.5 KB | avrdude |

### 3.2 ARM Cortex-M0/M0+

| ボード | MCU | Flash | RAM | 書き込みツール |
|--------|-----|-------|-----|--------------|
| Raspberry Pi Pico | RP2040 | 2 MB | 264 KB | picotool (UF2) |
| Raspberry Pi Pico 2 | RP2350 | 4 MB | 520 KB | picotool (UF2) |
| Raspberry Pi Zero | ARM1176JZF-S | — | — | SD カード |
| SAMD21 (Adafruit) | ATSAMD21G18 | 256 KB | 32 KB | BOSSAC |
| Arduino Due | ATSAM3X8E | 512 KB | 96 KB | BOSSAC |
| nRF51822 | nRF51822 | 256 KB | 16 KB | probe-rs |

### 3.3 ARM Cortex-M3

| ボード | MCU | Flash | RAM | 書き込みツール |
|--------|-----|-------|-----|--------------|
| STM32F103 (Blue Pill) | STM32F103C8 | 64 KB | 20 KB | probe-rs |

### 3.4 ARM Cortex-M4/M4F

| ボード | MCU | Flash | RAM | 書き込みツール |
|--------|-----|-------|-----|--------------|
| STM32F4 Discovery | STM32F407VG | 1 MB | 192 KB | probe-rs |
| STM32L4 | STM32L476RG | 1 MB | 128 KB | probe-rs |
| nRF52840 (Adafruit) | nRF52840 | 1 MB | 256 KB | probe-rs |
| SAMD51 (Adafruit M4) | ATSAMD51J19 | 512 KB | 192 KB | BOSSAC |
| BBC micro:bit V2 | nRF52833 | 512 KB | 128 KB | probe-rs (CMSIS-DAP) |

### 3.5 ARM Cortex-M7

| ボード | MCU | Flash | RAM | 書き込みツール |
|--------|-----|-------|-----|--------------|
| STM32F7 | STM32F746NG | 1 MB | 320 KB | probe-rs |
| STM32H7 | STM32H743ZI | 2 MB | 1 MB | probe-rs |
| Teensy 4.x | IMXRT1062 | 2 MB | 1 MB | teensy_loader_cli |

### 3.6 ARM Cortex-M33

| ボード | MCU | Flash | RAM | 書き込みツール |
|--------|-----|-------|-----|--------------|
| STM32G0 | STM32G071RB | 128 KB | 36 KB | probe-rs |

### 3.7 Xtensa ESP32 系

| ボード | MCU | Flash | RAM | 書き込みツール |
|--------|-----|-------|-----|--------------|
| ESP32 DevKitC | ESP32 | 4 MB | 520 KB | esptool.py |
| ESP32-S2 | ESP32-S2 | 4 MB | 320 KB | esptool.py |
| ESP32-S3 | ESP32-S3 | 8 MB | 512 KB | esptool.py |

### 3.8 RISC-V

| ボード | MCU | Flash | RAM | 書き込みツール |
|--------|-----|-------|-----|--------------|
| ESP32-C3 | ESP32-C3 | 4 MB | 400 KB | esptool.py |
| ESP32-C6 | ESP32-C6 | 4 MB | 512 KB | esptool.py |
| ESP32-H2 | ESP32-H2 | 4 MB | 320 KB | esptool.py |
| GD32VF103 | GD32VF103 | 128 KB | 32 KB | probe-rs |
| CH32V003 | CH32V003 | 16 KB | 2 KB | probe-rs |

---

## 4. 機能要件

### FR-1: コードエディタ

| ID | 要件 |
|----|------|
| FR-1-1 | Rust ソースファイル（.rs）を開く・編集・保存できる |
| FR-1-2 | 複数ファイルをタブで管理できる |
| FR-1-3 | 未保存変更を「*」マークで表示する |
| FR-1-4 | ファイルツリーでワークスペース内のファイルを一覧・選択できる |
| FR-1-5 | シンタックスハイライト（egui_extras syntect）を表示する |
| FR-1-6 | カーソル位置（行・列）をステータスバーに表示する |
| FR-1-7 | Tab キーでインデント（スペース 4）を挿入する |
| FR-1-8 | `{`・`(`・`[` の自動ペアリング（closing bracket 自動挿入）を行う |
| FR-1-9 | コードスニペットを検索・挿入できる |
| FR-1-10 | ブレークポイントをエディタ左端に設定できる |

### FR-2: プロジェクト管理

| ID | 要件 |
|----|------|
| FR-2-1 | 新規プロジェクトをテンプレートから作成できる（ボード別ブリンクサンプル） |
| FR-2-2 | 既存の Cargo プロジェクトをワークスペースとして開ける |
| FR-2-3 | プロジェクト設定（ボード種別・ポート・ボーレート）を `.toml` ファイルに保存・読み込みできる |
| FR-2-4 | ワークスペース内に新規ファイルを作成できる |

### FR-3: ボード・ポート選択

| ID | 要件 |
|----|------|
| FR-3-1 | BOARD_PRESETS に登録されたボードをコンボボックスで選択できる |
| FR-3-2 | 接続中のシリアルポートを一覧表示し選択できる |
| FR-3-3 | ボードの USB VID/PID による自動検出機能を持つ |
| FR-3-4 | probe-rs 経由での自動検出を行う（ARM/RISC-V ボード） |
| FR-3-5 | 検出結果をポップアップ通知で表示する |

### FR-4: コンパイル

| ID | 要件 |
|----|------|
| FR-4-1 | `cargo build --release` を選択ボードのターゲットトリプルで実行できる |
| FR-4-2 | ビルドログ（stdout/stderr）をリアルタイムにビルドパネルに表示する |
| FR-4-3 | ビルド成功・失敗をステータスバーに表示する |
| FR-4-4 | ビルド中は Build ボタンを無効化する |
| FR-4-5 | Flash メモリ使用量をプログレスバー（KB/最大 KB）で表示する |
| FR-4-6 | RAM 使用量を同様にプログレスバーで表示する |

### FR-5: マイコンへの書き込み（Flash）

| ID | 要件 |
|----|------|
| FR-5-1 | ビルド成功後に選択中のボード・ポートへ書き込みを実行できる |
| FR-5-2 | 対応書き込みツール: avrdude / esptool.py / probe-rs / picotool / BOSSAC / OpenOCD / st-flash / nrfjprog / teensy_loader_cli / DAPLink (.hex) |
| FR-5-3 | 書き込み中はログをリアルタイムに表示する |
| FR-5-4 | 「Build & Flash」ボタンでビルド成功直後に自動書き込みを実行できる |
| FR-5-5 | 書き込み対象の ELF パスを自動検索する（target/〈triple〉/release/〈package〉） |
| FR-5-6 | 非 UTF-8 パスを含む環境でも正常に動作する（PathBuf 使用） |

### FR-6: シリアルモニタ

| ID | 要件 |
|----|------|
| FR-6-1 | 選択ポート・ボーレートでシリアル接続・切断できる |
| FR-6-2 | 受信データを最大 500 行のログとしてスクロール表示する |
| FR-6-3 | テキスト入力フィールドから送信できる（Enter キーまたはボタン） |
| FR-6-4 | 受信ログのクリアができる |
| FR-6-5 | 接続状態（接続中 / 切断）をインジケーターで表示する |
| FR-6-6 | シリアル受信エラー（ポート切断等）を自動検知してログに表示する |

### FR-7: シリアルプロッタ

| ID | 要件 |
|----|------|
| FR-7-1 | シリアル受信データからチャンネル別の数値を解析してリアルタイムグラフに表示する |
| FR-7-2 | 解析フォーマット: `key:value,key:value,...` または `value1,value2,...`（CSV） |
| FR-7-3 | 各チャンネルを色分けして egui_plot に表示する |
| FR-7-4 | 最大保持サンプル数を設定できる |

### FR-8: ピンアウト表示

| ID | 要件 |
|----|------|
| FR-8-1 | 選択ボードのピンアウト図をダイアグラム形式で表示する |
| FR-8-2 | ピンアウトをテーブル形式でも表示できる（ビュー切替） |
| FR-8-3 | ピン機能（GPIO / UART / SPI / I2C / PWM / ADC / Power / GND）でフィルタリングできる |
| FR-8-4 | ピンにカーソルを合わせると機能説明のツールチップを表示する |
| FR-8-5 | ピンを選択すると詳細カード（番号・名前・機能バッジ）を表示する |
| FR-8-6 | 対応ボード: Arduino Uno、ESP32 DevKitC、micro:bit V2、STM32F4 Discovery |

### FR-9: ELF ビューア

| ID | 要件 |
|----|------|
| FR-9-1 | ビルド成果物の ELF ファイルを解析してセクション一覧を表示する |
| FR-9-2 | 各セクション（.text / .data / .bss 等）のアドレス・サイズを表示する |
| FR-9-3 | シンボル一覧（関数・変数）をサイズ降順で表示する |
| FR-9-4 | Flash / RAM 使用量の内訳を表示する |

### FR-10: スタック解析

| ID | 要件 |
|----|------|
| FR-10-1 | `nm` ツールを用いてスタック使用量を関数別に推定する |
| FR-10-2 | 関数名フィルタで絞り込み検索できる |
| FR-10-3 | スタック推定値の合計を表示する |

### FR-11: デバッガ連携（probe-rs）

| ID | 要件 |
|----|------|
| FR-11-1 | probe-rs 経由でターゲットに接続・切断できる |
| FR-11-2 | 実行・停止（Halt）・ステップ実行・リセットを制御できる |
| FR-11-3 | CPU レジスタ（PC、SP、汎用レジスタ）の値をリアルタイム表示する |
| FR-11-4 | 指定アドレスのメモリダンプを 16 進数で表示する |
| FR-11-5 | RTT（Real-Time Transfer）ログをチャンネル別にリアルタイム表示する |
| FR-11-6 | ブレークポイントを設定・解除できる |

### FR-12: SVD レジスタビューア

| ID | 要件 |
|----|------|
| FR-12-1 | SVD（System View Description）XML ファイルを読み込める |
| FR-12-2 | 周辺回路・レジスタ・ビットフィールドを階層ツリーで表示する |
| FR-12-3 | レジスタの現在値（デバッグ接続中）を表示する |

### FR-13: LSP 連携（rust-analyzer）

| ID | 要件 |
|----|------|
| FR-13-1 | rust-analyzer の存在を自動チェックし、未インストールの場合はインストールボタンを表示する |
| FR-13-2 | プロジェクトを開いたとき rust-analyzer を自動起動して LSP セッションを確立する |
| FR-13-3 | 入力中にコード補完候補をポップアップ表示する |
| FR-13-4 | 診断情報（エラー・警告）をエディタ上に表示する |
| FR-13-5 | LSP の初期化シーケンス（initialize → initialized → didOpen）を順守する |

### FR-14: ツールチェーン管理

| ID | 要件 |
|----|------|
| FR-14-1 | rust-analyzer のインストール状態を確認できる |
| FR-14-2 | 設定画面からツールのインストールをトリガーできる |
| FR-14-3 | 外部ツール（avrdude / esptool / probe-rs 等）の存在を `which` で検索し、未発見時に警告する |

### FR-15: 設定

| ID | 要件 |
|----|------|
| FR-15-1 | ワークスペースディレクトリを設定・変更できる |
| FR-15-2 | ダーク / ライトテーマを切り替えられる |
| FR-15-3 | デフォルトボーレートを設定できる |
| FR-15-4 | 設定を `AppConfig` として TOML ファイルに永続保存する |
| FR-15-5 | 設定ファイルの保存先は OS 標準設定ディレクトリ（`dirs` クレート使用） |

### FR-16: ヘルプ・ドキュメント

| ID | 要件 |
|----|------|
| FR-16-1 | アプリ内にユーザードキュメント（Markdown）を組み込み表示できる |
| FR-16-2 | ドキュメントはフォントサイズ変更に対応する |
| FR-16-3 | Markdown のシンタックスハイライト（egui_commonmark）に対応する |

---

## 5. 非機能要件

### NFR-1: パフォーマンス

| ID | 要件 |
|----|------|
| NFR-1-1 | UI の描画フレームレートは 60 FPS を維持する（重い処理はバックグラウンドスレッドで実行） |
| NFR-1-2 | シリアルログの上限を 500 行とし、メモリ使用量が無制限に増大しないようにする |
| NFR-1-3 | ビルド・書き込み・LSP などの重い処理は必ず別スレッドで実行し、UI をブロックしない |

### NFR-2: 安全性・堅牢性

| ID | 要件 |
|----|------|
| NFR-2-1 | パニック（`unwrap()` の乱用）を禁止し、`anyhow::Result` でエラーを伝播する |
| NFR-2-2 | 配列・スライスへのインデックスアクセスは `.get()` で境界チェックを行う |
| NFR-2-3 | バックグラウンドスレッドのリソースリーク（特にシリアル書き込みスレッド）を防ぐ |
| NFR-2-4 | 非 UTF-8 パスを含む環境（特に Windows）でクラッシュしない |

### NFR-3: 移植性

| ID | 要件 |
|----|------|
| NFR-3-1 | Windows / Linux 両対応（macOS は任意） |
| NFR-3-2 | パス操作には `std::path::PathBuf` / `Path` を使用し、文字列結合を行わない |
| NFR-3-3 | Windows AppLocker / WDAC 環境では WSL または管理者権限での実行を推奨する（ドキュメント記載） |

### NFR-4: 保守性

| ID | 要件 |
|----|------|
| NFR-4-1 | Core 層（`src/core/`）は `egui` / `eframe` に依存しない |
| NFR-4-2 | 新しいボードを追加する場合は `BOARD_PRESETS` に1エントリ追加するだけで完結する |
| NFR-4-3 | 新しいバックグラウンドメッセージは `AppMessage` enum にバリアントを追加し、`handle_messages()` の match で網羅する |
| NFR-4-4 | コードは `cargo fmt` でフォーマットし、`cargo clippy` で警告ゼロを保つ |
| NFR-4-5 | すべての `.rs` ファイルに SPDX ライセンスヘッダーを付与する |

### NFR-5: ライセンス

| ID | 要件 |
|----|------|
| NFR-5-1 | プロジェクト全体を MIT OR Apache-2.0 のデュアルライセンスで公開する |
| NFR-5-2 | 依存クレートはすべて MIT / Apache-2.0 / BSD 互換ライセンスであること（コピーレフト禁止） |
| NFR-5-3 | OSSライセンス審査結果を `OSS_LICENSE_AUDIT.md` に記録する |

---

## 6. アーキテクチャ制約

```
src/
├── main.rs          — eframe::run_native エントリーポイント
├── app.rs           — IdeApp 状態・AppMessage・update() レイアウト
├── core/            — バックエンド（UI 非依存）
│   ├── board.rs         ボード定数・USB ID テーブル
│   ├── compiler.rs      cargo build 実行
│   ├── flasher.rs       各書き込みツール呼び出し
│   ├── serial.rs        シリアル通信
│   ├── config.rs        TOML 設定
│   ├── pinout.rs        ピンアウトデータ
│   ├── lsp.rs           rust-analyzer LSP クライアント
│   ├── debugger.rs      probe-rs デバッグスレッド
│   ├── elf_analyzer.rs  ELF 解析
│   ├── stack_analyzer.rs スタック推定
│   ├── svd_parser.rs    SVD XML パーサ
│   ├── detector.rs      ボード自動検出
│   └── toolchain.rs     ツールチェーン管理
└── ui/              — egui パネル（core を呼び出す）
    ├── editor.rs
    ├── build_panel.rs
    ├── board_picker.rs
    ├── serial_monitor.rs
    ├── serial_plotter.rs
    ├── debug_panel.rs
    ├── rtt_panel.rs
    ├── svd_panel.rs
    ├── elf_panel.rs
    ├── stack_panel.rs
    ├── pinout_panel.rs
    ├── help_panel.rs
    ├── settings.rs
    └── file_explorer.rs
```

### メッセージフロー

```
[バックグラウンドスレッド]
    ↓ crossbeam_channel::Sender<AppMessage>
[IdeApp::handle_messages()]  ← 毎フレーム try_recv
    ↓ IdeApp のフィールド更新
[UI パネル描画]
```

---

## 7. 外部ツール依存

| ツール | 用途 | 必須/任意 |
|--------|------|----------|
| `cargo` | ビルド | 必須 |
| `rustup` | ツールチェーン管理 | 必須 |
| `avrdude` | AVR 書き込み | AVR ボード使用時 |
| `avr-gcc` | AVR クロスコンパイラ | AVR ボード使用時 |
| `esptool.py` | ESP32 書き込み | ESP32 使用時 |
| `probe-rs` | ARM/RISC-V 書き込み・デバッグ | ARM/RISC-V ボード使用時 |
| `picotool` | RP2040/RP2350 書き込み | Pico 使用時 |
| `rust-analyzer` | LSP 補完・診断 | 任意（推奨） |
| `arm-none-eabi-size` | Flash/RAM サイズ取得 | 任意 |
| `nm` | スタック解析 | 任意 |

---

## 8. 未対応・将来課題

| # | 項目 |
|---|------|
| 1 | macOS 対応（現状未検証） |
| 2 | flasher.rs のプラグイン化（match 分岐の削減） |
| 3 | ELF デバッグ情報を用いたソースレベルデバッグ |
| 4 | LSP の `response_rx` チャンネルの未使用箇所解消 |
| 5 | CI/CD パイプライン（GitHub Actions）の整備 |
| 6 | テスト（`cargo test`）カバレッジの拡充 |
| 7 | WASM 対応（ブラウザ版） |
| 8 | プラグイン API によるサードパーティボード追加 |

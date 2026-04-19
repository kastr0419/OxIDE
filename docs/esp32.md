# ESP32 クレート使用ガイド

## 対応ボード一覧
| ボード名 | チップ | features |
|---|---:|---|
| ESP32 | esp32 | "esp32" |
| ESP32-S2 | esp32s2 | "esp32s2" |
| ESP32-S3 | esp32s3 | "esp32s3" |
| ESP32-C3 | esp32c3 | "esp32c3" |
| ESP32-C6 | esp32c6 | "esp32c6" |
| ESP32-H2 | esp32h2 | "esp32h2" |

## Cargo.toml 設定
基本依存例:

```toml
[dependencies]
esp-hal = { version = "0.22", features = ["esp32"] }
esp-backtrace = { version = "0.14", features = ["esp32", "panic-handler", "println"] }
```

- 各チップごとに features を変更してください（例: `features = ["esp32s3"]`）。
- esp-backtrace は panic表示や println 機能を使う場合に有用です。各チップ向けに同様の features を指定します。

## ツールチェーン設定
各チップのターゲットトリプル:
- esp32: xtensa-esp32-none-elf
- esp32s2: xtensa-esp32s2-none-elf
- esp32s3: xtensa-esp32s3-none-elf
- esp32c3: riscv32imc-unknown-none-elf
- esp32c6/h2: riscv32imac-unknown-none-elf

注意: Xtensa 系ターゲットは rust の標準 toolchain ではなく espup によるインストールが必要です（espup がツールチェインとカスタム rustc を整備します）。

例: .cargo/config.toml の一部（テンプレートから）

```toml
[build]
target = "xtensa-esp32-none-elf"

[target.xtensa-esp32-none-elf]
runner = "espflash flash --monitor"
rustflags = ["-C", "link-arg=-nostartfiles"]

[unstable]
build-std = ["core"]
```

## GPIO ピン初期化
esp-hal の基本的な初期化は以下の通りです（テンプレートの Lチカをそのまま使用できます）。

```rust
// Example: ESP32 (from src/templates/blink/esp.rs)
#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    gpio::{Level, Output},
    main,
};

#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let delay = Delay::new();

    // GPIO2 = on-board LED on many ESP32 boards
    let mut led = Output::new(peripherals.GPIO2, Level::Low);

    loop {
        led.set_high();
        delay.delay_millis(500u32);
        led.set_low();
        delay.delay_millis(500u32);
    }
}
```

- `esp_hal::init()` はペリフェラルの初期化とピン割り当てを行います。
- `Output::new(peripherals.GPIOx, Level::Low)` で出力ピンを作成します。

## LED 制御
- 多くの ESP32 ボードのオンボード LED は GPIO2 を使用します（テンプレート参照）。
- ESP32-C3/C6/H2 系では GPIO8 をオンボード LED に使うボードが多いです（テンプレートの C3/C6/H2 を参照）。

例: ESP32-C3（src/templates/blink の抜粋）

```rust
#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    gpio::{Level, Output},
    main,
};

#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let delay = Delay::new();
    // many ESP32-C3 boards use GPIO8 for the LED
    let mut led = Output::new(peripherals.GPIO8, Level::Low);

    loop {
        led.set_high();
        delay.delay_millis(500u32);
        led.set_low();
        delay.delay_millis(500u32);
    }
}
```

## PWM 設定
esp-hal の LEDC (LED Controller) を用いることで PWM 出力が可能です。esp-hal の実装はターゲットやバージョンで API が変わるため、以下は一般的な流れの例です。

- Timer を初期化
- Channel を Timer に割り当て
- 周波数 / 分解能 を設定し、デューティ比を更新

（注）esp-hal 0.22 では API が変化しているため、正確な型名・関数名は Cargo のドキュメントまたは crate の examples を参照してください。安全に動くサンプルとしては、各ボードの公式 example を利用することを推奨します。

## UART / シリアル通信
esp-hal は UART 周りの HAL を提供します。基本手順:

- ペリフェラルから UART を取得
- ピンを TX/RX に設定
- UART を構成して読み書き

簡易的な構成手順（概念）:

```rust
// pseudocode-like (please consult esp-hal docs for exact APIs):
// let mut uart = peripherals.UART0.configure(baudrate, &config);
// uart.write(b"hello\n");
// let b = uart.read();
```

実際のコンパイル可能な UART サンプルは esp-hal の examples レポジトリを参照してください。

## タイマー / 遅延
Delay の使用は Blink テンプレートにある通りです。

```rust
let delay = Delay::new();
delay.delay_millis(500u32);
```

- Delay はブロッキング遅延です。短いテストや簡単な LED blink に適しています。

## Wi-Fi / BLE（概要）
- Wi‑Fi/BT 機能は esp-wifi / esp-idf-sys や更に高レベルのクレート群で提供されます。
- Wi‑Fi を使う場合は esp-idf ツールチェーンや追加のリンク設定が必要です（esp-idf-sys の導入、esp-idf のビルドなど）。

## 注意事項・Tips
- Xtensa 系 (esp32/esp32s2/esp32s3) は espup を使ってカスタム toolchain をインストールしてください。
- RISC‑V 系 (esp32c3/esp32c6/esp32h2) は rustup で riscv ターゲットを追加すれば良いことが多いです。
- esp-hal 0.22 以降で API が変わっています。公式リポジトリの examples と CHANGELOG を確認してください。
- デバッグ用に `esp-backtrace` を使うと panic 表示や println が利用しやすくなります。
- embedded-hal 1.0 を直接利用する場合はトレイトを明示的に use してください（prelude に依存しない）。例:

```rust
use embedded_hal::digital::OutputPin;
use embedded_hal::delay::DelayMs;
```

## 割り込み
ESP32 系では外部割り込みやタイマ割り込み、RTOS を利用した割り込み処理が一般的です。Xtensa 系は割り込みベクタテーブルの取り扱いに注意が必要です。割り込みハンドラは短くし、必要に応じてフラグや信号でメインループに処理を委譲してください。

## よくあるエラーと対処法
- ツールチェーン不一致: espup で導入したツールチェーンと Cargo のターゲットが一致しているか確認してください。
- リンカエラー: `-C link-arg` / start-up ファイルが不足している場合は BSP の README を参照し、必要な rustflags を .cargo/config に追加してください。
- UART/GPIO が動作しない: ピンの割当や機能(alt function)設定が HAL のバージョンで変わっていないか確認してください。

---

参考: リポジトリ内のテンプレート実装（examples）:
- `src/templates/blink/esp.rs` に各チップ向けの動作する L チカ サンプルがあります。これらはコンパイル可能な最小サンプルとして利用できます。

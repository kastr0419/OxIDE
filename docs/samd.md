# SAMD (Microchip SAM) クレート使用ガイド

## 対応ボード一覧
| ボード名 | チップ | クレート | ターゲット |
|---|---|---|---|
| Arduino Zero | SAMD21 | arduino-zero = "0.13" | thumbv6m-none-eabi |
| Adafruit Feather M4 | SAMD51 | feather-m4 = "0.10" | thumbv7em-none-eabihf |
| Arduino Due | SAM3X8E | (platform-specific) | thumbv7em-none-eabi |

## Cargo.toml 設定
例: SAMD21 (Arduino Zero)

```toml
[package]
name = "blink"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "blink"
test = false
bench = false

[dependencies]
arduino-zero = "0.13"
cortex-m = "0.7"
cortex-m-rt = "0.7"
panic-halt = "0.2"
```

例: SAMD51 (Feather M4)

```toml
[dependencies]
feather-m4 = "0.10"
cortex-m = "0.7"
cortex-m-rt = "0.7"
panic-halt = "0.2"
```

ターゲットは .cargo/config または .cargo/config.toml で指定できます:

```toml
[build]
target = "thumbv6m-none-eabi"  # SAMD21
# or
# target = "thumbv7em-none-eabihf"  # SAMD51
```

## ツールチェーン設定
- SAMD21: thumbv6m-none-eabi
- SAMD51: thumbv7em-none-eabihf

rustup でターゲットを追加してください:

```text
rustup target add thumbv6m-none-eabi
rustup target add thumbv7em-none-eabihf
```

## クロック設定
atsamd-hal の GenericClockController を使ってクロックを初期化します。多くの BSP テンプレートでは外部 32.768kHz RTC 用クロックを使うパターンが使われます。

```rust
use hal::clock::GenericClockController;

let mut peripherals = pac::Peripherals::take().unwrap();
let mut clocks = GenericClockController::with_external_32kosc(
    peripherals.GCLK, &mut peripherals.PM, &mut peripherals.SYSCTRL,
    &mut peripherals.NVMCTRL,
);
```

## GPIO ピン初期化
BSP が提供する Pins 構造体からピンを取り出して into_push_pull_output() を呼び出します。

```rust
let pins = bsp::Pins::new(peripherals.PORT);
let mut led = pins.led_sck.into_push_pull_output();
```

SAMD51 の場合の例:

```rust
let pins = bsp::Pins::new(peripherals.PORT);
let mut led = pins.led.into_push_pull_output();
```

## LED 制御
出力を High/Low にするだけです。BSP によって LED の名前が異なります（led_sck, led, d13 など）。

```rust
led.set_high().unwrap();
delay.delay_ms(500u32);
led.set_low().unwrap();
```

## サンプル: SAMD21 blink
以下は templates にある実際の動作サンプル（Arduino Zero 用）です。

```rust
// Arduino Zero / SAMD21 blink
// LED: PA17 (D13)
#![no_std]
#![no_main]

use arduino_zero as bsp;
use bsp::entry;
use bsp::hal;
use bsp::pac;
use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::prelude::*;
use panic_halt as _;

#[entry]
fn main() -> ! {
    let mut peripherals = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK, &mut peripherals.PM, &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );
    let pins = bsp::Pins::new(peripherals.PORT);
    let mut led = pins.led_sck.into_push_pull_output();
    let mut delay = Delay::new(core.SYST, &mut clocks);
    loop {
        led.set_high().unwrap();
        delay.delay_ms(500u32);
        led.set_low().unwrap();
        delay.delay_ms(500u32);
    }
}
```

## サンプル: SAMD51 blink (Feather M4)

```rust
// SAMD51 blink
// LED: PA23
#![no_std]
#![no_main]

use feather_m4 as bsp;
use bsp::entry;
use bsp::hal;
use bsp::pac;
use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::prelude::*;
use panic_halt as _;

#[entry]
fn main() -> ! {
    let mut peripherals = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK, &mut peripherals.PM, &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );
    let pins = bsp::Pins::new(peripherals.PORT);
    let mut led = pins.led.into_push_pull_output();
    let mut delay = Delay::new(core.SYST, &mut clocks);
    loop {
        led.set_high().unwrap();
        delay.delay_ms(500u32);
        led.set_low().unwrap();
        delay.delay_ms(500u32);
    }
}
```

## PWM 設定
atsamd-hal では TCC/TC を用いて PWM を生成します。BSP と HAL のバージョンで API が変わるため、以下はパターン例です。実際の関数名・型は使用する BSP のドキュメントやソースを参照してください。

```rust
// Pseudocode / pattern
// 1) clocks を初期化
// 2) TCC/TC ペリフェラルを初期化
// 3) ピンを PWM 出力に切り替え（BSP の into_pwm / into_function 等）
// 4) period/duty を設定して有効化
```

具体的実装は BSP の examples を参照してください。

## UART / シリアル通信
BSP はしばしば UART/Serial をラップした API を提供します。典型的な流れは:

```rust
// 1) pins から TX/RX を取得
// 2) BSP の uart/serial 構築関数を呼ぶ
// 3) read/write を行う

let mut serial = bsp::UART::new(peripherals.SERCOM2, pins.uart_tx, pins.uart_rx, &mut clocks, 115200);
serial.write_str("Hello\n").unwrap();
```

（関数名は BSP に依存します。feather-m4/arduino-zero の examples を参照してください。）

## タイマー / 遅延
cortex-m の SYST を利用した Delay は次のように作ります。

```rust
use hal::delay::Delay;
let core = pac::CorePeripherals::take().unwrap();
let mut delay = Delay::new(core.SYST, &mut clocks);
delay.delay_ms(500u32);
```

## SPI / I2C（基本）
atsamd-hal の SPI/I2C は SERCOM を利用して構築します。大まかな流れ:

- SPI: pins (sck, mosi, miso) を取得 → hal::spi::Spi::new(...)
- I2C: pins (sda, scl) を取得 → hal::i2c::I2c::new(...)

詳細は BSP の examples を参照してください。

## 注意事項・Tips
- 多くの BSP は内部で atsamd-hal を使用しているため、BSP の API を優先して利用してください。
- ピン名・LED 名は BSP ごとに異なります（led_sck, led, d13 等）。templates のサンプルや BSP の pins.rs を確認すること。
- クロック/メモリレイアウトはチップごとに異なる。テンプレートの memory.x を参考に調整してください。
- ビルドターゲット・リンカスクリプト（memory.x）は正しいものを選んでください。
- BSP の examples ディレクトリは良い出発点です。
- embedded-hal のトレイトを直接利用する場合は明示的に use してください（例: `use embedded_hal::digital::OutputPin;`）。

## よくあるエラーと対処法
- SERCOM 初期化失敗: SERCOM のピン割当・クロック供給を確認してください。
- リンカエラー: memory.x が BSP と一致しているか確認してください。

---

"参考: repository templates にある `src/templates/blink/samd.rs` を元に作成しました。"

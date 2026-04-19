# AVR (Arduino) クレート使用ガイド

このドキュメントは arduino-hal / avr-hal を使って AVR (Arduino) ボード向けに Rust プロジェクトを作成する際の実践的ガイドです。

## 対応ボード一覧
| ボード名 | チップ | クレート |
|---|---:|---|
| Arduino Uno | ATmega328P | arduino-hal (feature = "arduino-uno") |
| Arduino Nano | ATmega328P | arduino-hal (feature = "arduino-nano") |
| Arduino Mega 2560 | ATmega2560 | arduino-hal (feature = "arduino-mega2560") |
| Arduino Leonardo | ATmega32U4 | arduino-hal (feature = "arduino-leonardo") |

## Cargo.toml 設定
各ボードごとの最小構成の Cargo.toml 例（arduino-uno）:

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
arduino-hal = { git = "https://github.com/Rahix/avr-hal", features = ["arduino-uno"] }
panic-halt = "0.2"

[profile.dev]
panic = "abort"
lto = true
opt-level = "s"

[profile.release]
panic = "abort"
lto = true
opt-level = "s"
```

他ボードは `features` の値を `"arduino-nano"`, `"arduino-mega2560"`, `"arduino-leonardo"` に変えてください。

## ツールチェーン設定
推奨: rust-toolchain.toml をプロジェクトルートに置き、nightly と rust-src を有効にします。

```toml
[toolchain]
channel = "nightly"
components = ["rust-src"]
profile = "minimal"
```

.cargo/config.toml の例（uno 用）:

```toml
[build]
target = "avr-atmega328p.json"

[target.'cfg(target_arch = "avr")']
runner = "ravedude uno -cb 115200"
rustflags = ["-C", "opt-level=s"]
```

（ravedude を使うと cargo run でフラッシュとシリアルが統合できます。ravedude は別途インストールしてください。）

## GPIO ピン初期化
arduino-hal の基本パターン:

```rust
#![no_std]
#![no_main]
use panic_halt as _;
use arduino_hal::prelude::*;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    // D13 を出力に設定
    let mut led = pins.d13.into_output();

    loop { }
}
```

## LED 制御
オンボードLED (一般に D13) の制御例:

```rust
// トグル
led.toggle();

// ON / OFF
led.set_high().ok();
led.set_low().ok();
```

テンプレートの例（完全な blink）:

```rust
#![no_std]
#![no_main]
use panic_halt as _;
use arduino_hal::prelude::*;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut led = pins.d13.into_output();

    loop {
        led.toggle();
        arduino_hal::delay_ms(500);
    }
}
```

## PWM 設定
`arduino_hal::simple_pwm` を使った PWM の例（Timer1、D9/D10）:

```rust
#![no_std]
#![no_main]
use panic_halt as _;
use arduino_hal::prelude::*;
use arduino_hal::simple_pwm::{Timer1Pwm, Prescaler};

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    // Timer1 を Prescale64 で初期化
    let mut timer1 = Timer1Pwm::new(dp.TC1, Prescaler::Prescale64);

    // D9 / D10 を PWM 出力にする
    let mut d9 = pins.d9.into_output().into_pwm(&mut timer1);
    let mut d10 = pins.d10.into_output().into_pwm(&mut timer1);

    // デューティ設定 (0..=255)
    d9.set_duty(128);
    d9.enable();

    loop { }
}
```

## UART / シリアル通信
`default_serial!` マクロで簡単初期化。`ufmt` を使った軽量フォーマット出力例:

```rust
#![no_std]
#![no_main]
use panic_halt as _;
use arduino_hal::prelude::*;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    // ufmt を使って出力
    ufmt::uwriteln!(&mut serial, "Hello from AVR\r").ok();

    loop { }
}
```

## タイマー / 遅延
ブロッキング遅延の使い方:

```rust
arduino_hal::delay_ms(1000); // 1000ms 待つ
```

`delay_ms` は、`arduino_hal::prelude::*` を使っていると利用できます。

## SPI / I2C（基本）
簡単な使用例（初期化の雛形）:

```rust
#![no_std]
#![no_main]
use panic_halt as _;
use arduino_hal::prelude::*;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    // SPI の雛形（詳細はボードドキュメント参照）
    // let mut spi = arduino_hal::Spi::new(dp.SPI, pins.d11.into_output(), pins.d12, pins.d13, arduino_hal::spi::Settings::default());

    // I2C の雛形（詳細はボードドキュメント参照）
    // let mut i2c = arduino_hal::I2c::new(dp.TWI, pins.a4, pins.a5, 100_000);

    loop { }
}
```

注: 上記は各ボードのペリフェラル名 (SPI/TWI) に依存します。詳細は arduino-hal ドキュメントの `spi` / `i2c` セクションを参照してください。

## 注意事項・Tips
- arduino-hal はリポジトリを git 依存で指定して使うことが多いです（例: `git = "https://github.com/Rahix/avr-hal"`）。リリースを使う場合は crates.io のバージョンを確認してください。
- Rust の nightly ツールチェーンが必要です。`rust-toolchain.toml` をプロジェクトに置くことで自動インストールされます（`rust-src` コンポーネントが必要）。
- AVR 用の JSON ターゲット仕様ファイル（例: `avr-atmega328p.json`）が必要です。`avr-hal-template` や `rahix/avr-hal` のドキュメントを参照して入手してください。
- 開発環境の外部依存: `avr-gcc`, `avrdude`（Windows では winget / scoop で入手可能）。
- ビルド後の実機書き込みは `ravedude` を使うと cargo ライクな体験になります（`cargo run` でビルド+フラッシュ+シリアル）。

## 割り込み
AVR では外部割り込み(INT0/INT1)やタイマ割り込みを利用できます。`avr-hal` / `arduino-hal` の場合、割り込みハンドラの登録や ISR の記述方法は BSP や使用するライブラリによって異なります。タイマ割り込みはタイマレジスタの初期化と ISR 内での処理を行います。ISR 内では軽量にし、共有資源は適切に同期してください。

## よくあるエラーと対処法
- リンカエラー (memory.x 関連): 使用するボード向けのリンクスクリプト (memory.x) を用意するか、BSP が提供するものを利用してください。
- `Peripherals::take()` が None を返す: 既に別のコードで取得されているため二重取得していないか確認してください。
- フラッシュツールが見つからない: `avrdude` / `ravedude` が PATH にあるか、.cargo/config の runner が正しいコマンドを指しているか確認してください。

---

参考:
- https://github.com/Rahix/avr-hal
- https://rahix.github.io/avr-hal/arduino_hal/index.html


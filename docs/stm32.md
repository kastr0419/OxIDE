# STM32 クレート使用ガイド

以下は STM32 系マイコン向け Rust HAL クレートの使い方ガイドです。コードサンプルは各系列で実際にコンパイル可能な形を目指していますが、ターゲットトリプルと機器に合わせて調整してください。

## 対応ボード一覧
| ボード名 | チップ | クレート | features |
|---|---|---:|---|
| STM32F103 "BluePill" | STM32F103C8 | stm32f1xx-hal | features = ["stm32f103", "medium"]
| STM32F407 Discovery | STM32F407VG | stm32f4xx-hal | features = ["stm32f407", "rt"]
| STM32L4 Discovery | STM32L432KC | stm32l4xx-hal | features = ["stm32l431", "rt"]
| STM32G0 Nucleo | STM32G071RB | stm32g0xx-hal | features = ["stm32g071", "rt"]
| STM32F7 Discovery | STM32F746NG | stm32f7xx-hal | features = ["stm32f746", "rt"]
| STM32H7 Series | STM32H743ZI | stm32h7xx-hal | features = ["stm32h743", "rt"]

## Cargo.toml 設定
各系列ごとの設定例:

- STM32F1 (BluePill):

```toml
[dependencies]
cortex-m = "0.7"
cortex-m-rt = "0.7"
panic-halt = "0.2"
stm32f1xx-hal = { version = "0.10", features = ["stm32f103", "medium"] }
```

- STM32F4 (F407):

```toml
[dependencies]
stm32f4xx-hal = { version = "0.15", features = ["stm32f407", "rt"] }
cortex-m = "0.7"
cortex-m-rt = "0.7"
panic-halt = "0.2"
```

- STM32L4:

```toml
[dependencies]
stm32l4xx-hal = { version = "0.12", features = ["stm32l4x2", "rt"] }
```

- STM32G0 / F7 / H7 等も同様にクレート名と features を合わせて指定してください。

## ツールチェーン設定
ターゲット例 (Cortex-M4 FPU): thumbv7em-none-eabihf

.cargo/config.toml の例:

```toml
[target.thumbv7em-none-eabihf]
runner = "probe-run"
rustflags = ["-C", "link-arg=-Tlink.x"]
```

memory.x の例 (STM32F103 は小さいため例示は F4 用):

```ld
/* memory.x */
MEMORY
{
  FLASH : ORIGIN = 0x08000000, LENGTH = 512K
  RAM : ORIGIN = 0x20000000, LENGTH = 128K
}
```

プローブランナー (probe-run) や OpenOCD、cargo-embed などを活用してください。

## クロック設定
RCC とクロックの初期化は HAL 提供の API を使います。下記は stm32f4xx-hal の例:

```rust
use stm32f4xx_hal::{prelude::*, stm32};

let dp = stm32::Peripherals::take().unwrap();
let rcc = dp.RCC.constrain();
let clocks = rcc.cfgr
    .use_hse(8.mhz())
    .sysclk(168.mhz())
    .pclk1(42.mhz())
    .freeze();
```

系列により API 名と可用なクロック設定が異なります（F1 では `flash` 設定や `C1` の扱いに注意）。

## GPIO ピン初期化
系列ごとに split と into_push_pull_output の初期化方法が異なります。

- STM32F1 系:

```rust
use stm32f1xx_hal::{prelude::*, stm32};
let dp = stm32::Peripherals::take().unwrap();
let mut gpioc = dp.GPIOC.split();
let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
led.set_high().unwrap();
```

- STM32F4 系:

```rust
use stm32f4xx_hal::{prelude::*, stm32};
let dp = stm32::Peripherals::take().unwrap();
let gpioc = dp.GPIOC.split();
let mut led = gpioc.pc13.into_push_pull_output();
led.set_high();
```

F1 系では CRL/CRH レジスタ参照が必要ですが、新しい HAL では API が統一されつつあります。

## LED 制御
多くのボードは LED がアクティブ LOW（ボード上で GND に接続して点灯）になっています。

```rust
// アクティブLOW の場合
led.set_low().unwrap(); // 点灯
led.set_high().unwrap(); // 消灯
```

F4 系などの HAL では `set_high()` が Result を返さない場合もあります。

## PWM 設定
TIM を利用した PWM 生成例 (F4):

```rust
use stm32f4xx_hal::{prelude::*, stm32, pwm};
let dp = stm32::Peripherals::take().unwrap();
let rcc = dp.RCC.constrain();
let clocks = rcc.cfgr.freeze();
let gpioa = dp.GPIOA.split();
let pa8 = gpioa.pa8.into_alternate(); // TIM1 CH1 など
let pwm = dp.TIM1.pwm(pa8, 1.khz(), &clocks);
let mut pwm_ch = pwm0;
pwm_ch.set_duty(pwm_ch.get_max_duty() / 2);
pwm_ch.enable();
```

API は HAL によって異なるため、crate のドキュメントを参照してください。

## UART / シリアル通信
serial モジュールの初期化例 (F1):

```rust
use stm32f1xx_hal::{prelude::*, stm32, serial};
let dp = stm32::Peripherals::take().unwrap();
let mut afio = dp.AFIO.constrain();
let mut rcc = dp.RCC.constrain();
let clocks = rcc.cfgr.freeze(&mut dp.FLASH.constrain().acr);
let gpioa = dp.GPIOA.split();
let tx = gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh);
let rx = gpioa.pa10;
let serial = serial::Serial::usart1(dp.USART1, (tx, rx), &mut afio.mapr, 115200.bps(), clocks);
let (mut tx, mut rx) = serial.split();
// tx.write(b'h').ok();
```

F4 では API がよりシンプルになります。

## タイマー / 遅延
SYST を使った遅延（core peripherals）:

```rust
use cortex_m::peripheral::SYST;
use cortex_m::delay::Delay;
let mut cp = cortex_m::Peripherals::take().unwrap();
let mut delay = Delay::new(cp.SYST, clocks.sysclk().0);
delay.delay_ms(1_u32);
```

HAL 提供の TIM ベースの遅延 (例: TIM2):

```rust
use stm32f4xx_hal::timer::Timer;
let tim2 = Timer::tim2(dp.TIM2, 1.hz(), &clocks);
tim2.delay_ms(100_u32);
```

## SPI / I2C（基本）
SPI の初期化例 (F4):

```rust
use stm32f4xx_hal::{spi::Spi, gpio::GpioExt, prelude::*};
let gpioa = dp.GPIOA.split();
let sck = gpioa.pa5.into_alternate();
let miso = gpioa.pa6.into_alternate();
let mosi = gpioa.pa7.into_alternate();
let spi = Spi::spi1(dp.SPI1, (sck, miso, mosi), spi::Mode { polarity: spi::Polarity::IdleLow, phase: spi::Phase::CaptureOnFirstTransition }, 1.mhz(), &clocks);
```

I2C の初期化例:

```rust
use stm32f4xx_hal::i2c::I2c;
let scl = gpioa.pa9.into_alternate_open_drain();
let sda = gpioa.pa10.into_alternate_open_drain();
let i2c = I2c::i2c1(dp.I2C1, (scl, sda), 100.khz(), &clocks);
```

## 注意事項・Tips
- 系列ごとのGPIO APIの差異（F1 は CRH/CRL 引数が必要）。
- memory.x の FLASH/RAM サイズはチップに合わせて正確に設定すること。
- LED がアクティブLOW のボードが多いので点灯/消灯のロジックに注意。
- 一部 HAL は RT 機能や特定 feature を有効化しないと使えない API がある。
- embedded-hal のトレイトを直接利用する場合は明示的に use してください（例: `use embedded_hal::digital::OutputPin;`）。

## よくあるエラーと対処法
- クロック設定ミスによる周辺機器非動作: RCC/PLL の設定が正しいか、HAL の example と比較してください。
- リンカエラー: リンカスクリプトや memory.x の設定を確認してください。
- GPIO が期待通りに動作しない: Alternate function 設定や AF 番号が正しいか確認してください。

## 参照コード
プロジェクトのテンプレートにあるサンプルを参照してください:
`src/templates/blink/stm32.rs` を参考にしました。

---


# ピンアウト表示 使い方ガイド

> このドキュメントでは、rust-embedded-ide のピンアウト表示パネルの使い方と、対応ボードの全ピン一覧を説明します。

---

## ピンアウトパネルとは

IDE 右側パネルの **「Pinout」タブ** に、現在選択中のボードのピン配置図が表示されます。

```
┌─────────────────────────────┐
│  Board Picker  │  Editor  │ [Docs] [Pinout] │
│                             │
│  ピン配置図（色付き丸アイコン）  │
│  ↓ ピンをホバーすると詳細表示  │
│  Pin 13: D13                │
│  Functions: [Gpio, Pwm]     │
└─────────────────────────────┘
```

### 操作方法

| 操作 | 動作 |
|------|------|
| ピンにマウスオーバー | ピン名・機能・座標を下部に表示 |
| ピンをクリック | そのピンの詳細情報を固定表示 |
| ボード選択変更 | 自動的に対応ボードのピン図に切り替わり |

### ピンの色の意味

| 色 | 機能 | 説明 |
|----|------|------|
| 🟢 緑 | GPIO | 汎用入出力 |
| 🟠 オレンジ | UART | シリアル通信（TX/RX） |
| 🔵 青 | SPI | SPI バス（MOSI/MISO/SCK/CS） |
| 🟡 黄 | I2C | I2C バス（SDA/SCL） |
| 🩷 ピンク | PWM | PWM 出力対応ピン |
| 💚 薄緑 | ADC | アナログ入力 |
| 🟤 ベージュ | Power | 電源ピン（3.3V / 5V） |
| ⬛ 黒 | GND | グランド |
| ⬜ グレー | NC / 未定義 | 未接続または機能未設定 |

---

## Arduino Uno (ATmega328P)

### ボード概要

| 項目 | 内容 |
|------|------|
| MCU | ATmega328P (8bit AVR) |
| 動作電圧 | 5V |
| クロック | 16 MHz |
| フラッシュ | 32 KB |
| SRAM | 2 KB |
| EEPROM | 1 KB |
| デジタル I/O | 14本 (うち PWM 6本) |
| アナログ入力 | 6本 (A0〜A5) |

### デジタルピン

| ピン番号 | 名前 | 機能 | 備考 |
|---------|------|------|------|
| D0 | RX | UART RX | シリアル受信 |
| D1 | TX | UART TX | シリアル送信 |
| D2 | D2 | GPIO | 外部割り込み INT0 |
| D3 | D3 | GPIO, PWM | 外部割り込み INT1 |
| D4 | D4 | GPIO | — |
| D5 | D5 | GPIO, PWM | — |
| D6 | D6 | GPIO, PWM | — |
| D7 | D7 | GPIO | — |
| D8 | D8 | GPIO | — |
| D9 | D9 | GPIO, PWM | — |
| D10 | SS | GPIO, PWM, SPI | SPI チップセレクト |
| D11 | MOSI | GPIO, PWM, SPI | SPI データ出力 |
| D12 | MISO | GPIO, SPI | SPI データ入力 |
| D13 | SCK | GPIO, SPI | SPI クロック / 内蔵 LED |

### アナログピン

| ピン番号 | 名前 | 機能 | 備考 |
|---------|------|------|------|
| A0 | A0 | ADC, GPIO | ADC チャンネル 0 |
| A1 | A1 | ADC, GPIO | ADC チャンネル 1 |
| A2 | A2 | ADC, GPIO | ADC チャンネル 2 |
| A3 | A3 | ADC, GPIO | ADC チャンネル 3 |
| A4 | SDA | ADC, GPIO, I2C | I2C データライン |
| A5 | SCL | ADC, GPIO, I2C | I2C クロックライン |

### 電源ピン

| ピン | 機能 |
|------|------|
| 3.3V | 3.3V 出力（最大 50mA） |
| 5V | 5V 出力（USB または外部電源から） |
| GND | グランド（2本） |
| VIN | 外部電源入力（7〜12V） |
| RESET | リセット（Low でリセット） |
| IOREF | I/O 電圧参照 |

### Rust でのピン操作例 (arduino-hal)

```rust
use arduino_hal::prelude::*;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    // D13 を出力に設定（内蔵 LED）
    let mut led = pins.d13.into_output();

    // A0 をアナログ入力に設定
    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());
    let a0 = pins.a0.into_analog_input(&mut adc);

    loop {
        led.toggle();
        arduino_hal::delay_ms(500);
    }
}
```

---

## Arduino Nano (ATmega328P)

Uno とほぼ同じピン配置ですが、**ブレッドボード向けの小型フォームファクタ** です。

### ピン配置の違い

| Nano ピン | 対応 Uno ピン | 備考 |
|-----------|-------------|------|
| D0〜D13 | D0〜D13 | 同一機能 |
| A0〜A7 | A0〜A5 (+ A6/A7 追加) | A6/A7 はアナログ入力専用 |
| VCC | 5V | — |
| GND | GND | 両端に配置 |

> **注意**: A6・A7 はデジタル I/O として使用不可（アナログ入力専用）

---

## BBC micro:bit V2 (nRF52833)

### ボード概要

| 項目 | 内容 |
|------|------|
| MCU | nRF52833 (ARM Cortex-M4F) |
| 動作電圧 | 3.3V |
| クロック | 64 MHz |
| フラッシュ | 512 KB |
| RAM | 128 KB |
| 無線 | Bluetooth 5.1 / 2.4GHz |
| 内蔵センサー | 加速度計、磁気センサー、温度計、マイク、スピーカー |

### エッジコネクタ（大きなパッド）

| パッド番号 | 名前 | 機能 | 備考 |
|-----------|------|------|------|
| P0 | P0 | GPIO, ADC, Touch | タッチセンサー対応 |
| P1 | P1 | GPIO, ADC, Touch | タッチセンサー対応 |
| P2 | P2 | GPIO, ADC, Touch | タッチセンサー対応 |
| 3V | 3V | Power | 3.3V 出力 |
| GND | GND | GND | グランド |

### エッジコネクタ（小さなパッド、25ピンコネクタ使用時）

| パッド | 名前 | 機能 |
|--------|------|------|
| P3 | P3 | GPIO, ADC (LED matrix 行0と共有) |
| P4 | P4 | GPIO, ADC (LED matrix 行1と共有) |
| P5 | P5 | GPIO (ボタンAと共有) |
| P6 | P6 | GPIO (LED matrix 行2と共有) |
| P7 | P7 | GPIO (LED matrix 列1と共有) |
| P8 | P8 | GPIO |
| P9 | P9 | GPIO (LED matrix 行3と共有) |
| P10 | P10 | GPIO, ADC (LED matrix 列5と共有) |
| P11 | P11 | GPIO (ボタンBと共有) |
| P12 | P12 | GPIO |
| P13 | SCK | GPIO, SPI クロック |
| P14 | MISO | GPIO, SPI MISO |
| P15 | MOSI | GPIO, SPI MOSI |
| P16 | P16 | GPIO, SPI CS |
| P19 | SCL | I2C クロック |
| P20 | SDA | I2C データ |

### 内蔵デバイス

| デバイス | 用途 |
|---------|------|
| 5×5 LED マトリクス | 表示・インジケータ |
| ボタン A / B | 汎用入力 |
| 加速度計 (LSM303AGR) | 傾き・振動検出 |
| 磁気センサー (LSM303AGR) | コンパス |
| 温度センサー (nRF 内蔵) | 温度計測 |
| マイク (MEMS) | 音入力 (V2 のみ) |
| スピーカー | 音出力 (V2 のみ) |

### Rust でのピン操作例 (microbit-bsp)

```rust
#![no_std]
#![no_main]
use microbit::{board::Board, hal::Timer};

#[cortex_m_rt::entry]
fn main() -> ! {
    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);

    // P0 を出力に設定
    let mut p0 = board.edge.e00.into_push_pull_output(
        microbit::hal::gpio::Level::Low
    );

    loop {
        p0.set_high().unwrap();
        timer.delay_ms(500u32);
        p0.set_low().unwrap();
        timer.delay_ms(500u32);
    }
}
```

---

## ESP32 (Xtensa LX6)

### ボード概要（ESP32-DevKitC）

| 項目 | 内容 |
|------|------|
| MCU | ESP32 (デュアルコア Xtensa LX6) |
| 動作電圧 | 3.3V (入力: 5V via USB) |
| クロック | 最大 240 MHz |
| フラッシュ | 4 MB (モジュールにより異なる) |
| RAM | 520 KB SRAM |
| 無線 | Wi-Fi 802.11 b/g/n / Bluetooth 4.2 |

### ピン一覧（ESP32-DevKitC 38ピン版）

| GPIO | 名前 | 機能 | 備考 |
|------|------|------|------|
| GPIO0 | GPIO0 | GPIO, ADC2_CH1 | Boot モード選択（プルアップ必須） |
| GPIO1 | TXD0 | UART TX | USB シリアル TX |
| GPIO2 | GPIO2 | GPIO, ADC2_CH2, PWM | 内蔵 LED (一部ボード) |
| GPIO3 | RXD0 | UART RX | USB シリアル RX |
| GPIO4 | GPIO4 | GPIO, ADC2_CH0, PWM | — |
| GPIO5 | GPIO5 | GPIO, SPI CS0, PWM | ストラップピン |
| GPIO12 | GPIO12 | GPIO, ADC2_CH5, SPI MISO | ストラップピン（3.3V起動に注意） |
| GPIO13 | GPIO13 | GPIO, ADC2_CH4, SPI MOSI | — |
| GPIO14 | GPIO14 | GPIO, ADC2_CH6, SPI SCK | — |
| GPIO15 | GPIO15 | GPIO, ADC2_CH3, SPI CS | ストラップピン |
| GPIO16 | GPIO16 | GPIO, UART2 RX | — |
| GPIO17 | GPIO17 | GPIO, UART2 TX | — |
| GPIO18 | GPIO18 | GPIO, SPI SCK | VSPI クロック |
| GPIO19 | GPIO19 | GPIO, SPI MISO | VSPI MISO |
| GPIO21 | GPIO21 | GPIO, I2C SDA | デフォルト I2C SDA |
| GPIO22 | GPIO22 | GPIO, I2C SCL | デフォルト I2C SCL |
| GPIO23 | GPIO23 | GPIO, SPI MOSI | VSPI MOSI |
| GPIO25 | GPIO25 | GPIO, ADC2_CH8, DAC1 | D/A 出力 |
| GPIO26 | GPIO26 | GPIO, ADC2_CH9, DAC2 | D/A 出力 |
| GPIO27 | GPIO27 | GPIO, ADC2_CH7, PWM | — |
| GPIO32 | GPIO32 | GPIO, ADC1_CH4, PWM | Touch9 |
| GPIO33 | GPIO33 | GPIO, ADC1_CH5, PWM | Touch8 |
| GPIO34 | GPIO34 | GPIO, ADC1_CH6 | **入力専用** |
| GPIO35 | GPIO35 | GPIO, ADC1_CH7 | **入力専用** |
| GPIO36 | VP | ADC1_CH0 | **入力専用** (SVP) |
| GPIO39 | VN | ADC1_CH3 | **入力専用** (SVN) |

> ⚠️ GPIO34〜GPIO39 は **入力専用**（内部プルアップ/プルダウンなし）

### 電源ピン

| ピン | 機能 |
|------|------|
| 3V3 | 3.3V 出力 |
| GND | グランド |
| 5V / VIN | USB 5V 入力 |
| EN | チップイネーブル（High = 動作） |

### Rust でのピン操作例 (esp-idf-hal)

```rust
use esp_idf_hal::gpio::*;
use esp_idf_hal::peripherals::Peripherals;

fn main() {
    let peripherals = Peripherals::take().unwrap();
    let pins = peripherals.pins;

    // GPIO2 を出力に設定
    let mut led = PinDriver::output(pins.gpio2).unwrap();

    loop {
        led.set_high().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(500));
        led.set_low().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}
```

---

## STM32F4 Discovery (STM32F407VG)

### ボード概要

| 項目 | 内容 |
|------|------|
| MCU | STM32F407VG (ARM Cortex-M4F) |
| 動作電圧 | 3.3V |
| クロック | 最大 168 MHz |
| フラッシュ | 1 MB |
| RAM | 192 KB |
| 書き込みツール | probe-rs / ST-Link (内蔵) |

### ポート別ピン一覧（主要ピン）

#### PORTA

| ピン | 機能 | 備考 |
|------|------|------|
| PA0 | GPIO, ADC12_IN0, TIM2_CH1 | ユーザーボタン |
| PA1 | GPIO, ADC12_IN1, TIM2_CH2 | — |
| PA2 | GPIO, ADC12_IN2, USART2_TX | — |
| PA3 | GPIO, ADC12_IN3, USART2_RX | — |
| PA4 | GPIO, ADC12_IN4, DAC1, SPI1_NSS | — |
| PA5 | GPIO, ADC12_IN5, DAC2, SPI1_SCK | 内蔵 LED LD2 共有 |
| PA6 | GPIO, ADC12_IN6, SPI1_MISO | — |
| PA7 | GPIO, ADC12_IN7, SPI1_MOSI | — |
| PA8 | GPIO, TIM1_CH1, MCO1 | — |
| PA9 | GPIO, USART1_TX | — |
| PA10 | GPIO, USART1_RX | — |
| PA13 | SWDIO | SWD デバッグ（書き込み用） |
| PA14 | SWCLK | SWD クロック（書き込み用） |

#### PORTB

| ピン | 機能 | 備考 |
|------|------|------|
| PB6 | GPIO, I2C1_SCL, USART1_TX | — |
| PB7 | GPIO, I2C1_SDA, USART1_RX | — |
| PB10 | GPIO, I2C2_SCL, USART3_TX | — |
| PB11 | GPIO, I2C2_SDA, USART3_RX | — |

#### PORTD（内蔵 LED）

| ピン | 機能 | 備考 |
|------|------|------|
| PD12 | GPIO, TIM4_CH1 | **緑 LED (LD4)** |
| PD13 | GPIO, TIM4_CH2 | **橙 LED (LD3)** |
| PD14 | GPIO, TIM4_CH3 | **赤 LED (LD5)** |
| PD15 | GPIO, TIM4_CH4 | **青 LED (LD6)** |

### Rust でのピン操作例 (stm32f4xx-hal)

```rust
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use stm32f4xx_hal::{pac, prelude::*};

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let gpiod = dp.GPIOD.split();
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(168.MHz()).freeze();

    // PD12 (緑 LED) を出力に設定
    let mut led = gpiod.pd12.into_push_pull_output();

    let mut delay = dp.TIM1.delay_us(&clocks);

    loop {
        led.set_high();
        delay.delay_ms(500u32);
        led.set_low();
        delay.delay_ms(500u32);
    }
}
```

---

## ピンアウトパネルの制限事項

現在のピンアウトパネルは**ビジュアルスケッチ**として機能しており、以下の制限があります：

| 項目 | 状態 |
|------|------|
| 表示ピン数 | 一部のみ（代表的なピンを表示） |
| ピン配置 | 実際の物理的な位置と異なる場合あり |
| 代替機能の表示 | プライマリ機能のみ（AF番号は未表示） |
| 全ピン表示 | 今後の機能拡張予定 |

> 正確なピン配置・代替機能の詳細は各ボードのデータシートを参照してください。

### 参考リンク

| ボード | 公式ピン資料 |
|--------|------------|
| Arduino Uno | https://docs.arduino.cc/hardware/uno-rev3/ |
| micro:bit V2 | https://tech.microbit.org/hardware/edgeconnector/ |
| ESP32 DevKitC | https://docs.espressif.com/projects/esp-idf/en/latest/esp32/hw-reference/ |
| STM32F4 Discovery | https://www.st.com/en/evaluation-tools/stm32f4discovery.html |

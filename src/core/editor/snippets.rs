// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright 2026 ALLoIDE contributors

use crate::core::board::BoardKind;

/// スニペット1件
#[derive(Debug, Clone)]
pub struct Snippet {
    pub trigger: &'static str,     // トリガーキーワード（例: "gpio_out")
    pub label: &'static str,       // 表示名（例: "GPIO Output Pin")
    pub description: &'static str, // 説明
    pub code: &'static str,        // 挿入コード
    pub category: SnippetCategory,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub enum SnippetCategory {
    Gpio,
    Uart,
    Spi,
    I2c,
    Timer,
    Interrupt,
    Main,
    Delay,
    Adc,
    Pwm,
    Misc,
}

/// 全スニペットを BoardKind に応じて返す
pub fn get_snippets(board: &BoardKind) -> Vec<&'static Snippet> {
    let common = COMMON_SNIPPETS.iter().collect::<Vec<_>>();
    let board_specific: &[Snippet] = match board {
        BoardKind::ArduinoUno | BoardKind::ArduinoNano => AVR_SNIPPETS,
        BoardKind::ArduinoMega => AVR_SNIPPETS,
        BoardKind::ArduinoLeonardo => AVR_SNIPPETS,
        BoardKind::Esp32 | BoardKind::Esp32S2 | BoardKind::Esp32S3 => ESP_XTENSA_SNIPPETS,
        BoardKind::Esp32C3 | BoardKind::Esp32C6 | BoardKind::Esp32H2 => ESP_RISCV_SNIPPETS,
        BoardKind::Stm32F4
        | BoardKind::Stm32F1
        | BoardKind::Stm32L4
        | BoardKind::Stm32F7
        | BoardKind::Stm32H7
        | BoardKind::Stm32G0 => STM32_SNIPPETS,
        BoardKind::RpiPico | BoardKind::RpiPico2 => RPI_PICO_SNIPPETS,
        BoardKind::NrF52840 | BoardKind::MicroBitV2 => NRF_SNIPPETS,
        BoardKind::Samd21 | BoardKind::Samd51 => SAMD_SNIPPETS,
        _ => CORTEX_M_GENERIC_SNIPPETS,
    };
    let mut all = common;
    all.extend(board_specific.iter());
    all
}

/// トリガー文字列でフィルタリング（予測変換用）
pub fn filter_snippets(board: &BoardKind, query: &str) -> Vec<&'static Snippet> {
    if query.is_empty() {
        return vec![];
    }
    let q = query.to_lowercase();
    get_snippets(board)
        .into_iter()
        .filter(|s| s.trigger.contains(q.as_str()) || s.label.to_lowercase().contains(q.as_str()))
        .collect()
}

// ─── 共通スニペット ──────────────────────────────────────

static COMMON_SNIPPETS: &[Snippet] = &[
    Snippet {
        trigger: "main",
        label: "main() - no_std embedded",
        description: "no_std embedded エントリーポイント",
        code: r#"#![no_std]
#![no_main]

use panic_halt as _;

#[no_mangle]
pub fn main() -> ! {
    loop {
        // your code here
    }
}"#,
        category: SnippetCategory::Main,
    },
    Snippet {
        trigger: "delay_ms",
        label: "cortex_m::delay_ms",
        description: "ミリ秒ディレイ (cortex-m)",
        code: "cortex_m::asm::delay(cycles);",
        category: SnippetCategory::Delay,
    },
];

// ─── AVR (avr-hal / arduino-hal) ────────────────────────

static AVR_SNIPPETS: &[Snippet] = &[
    Snippet {
        trigger: "arduino_init",
        label: "Arduino HAL 初期化",
        description: "arduino-hal のエントリーポイントと初期化",
        code: r#"#![no_std]
#![no_main]

use arduino_hal::prelude::*;
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 115200);

    loop {
        ufmt::uwriteln!(&mut serial, "Hello, Arduino!").unwrap();
        arduino_hal::delay_ms(1000);
    }
}"#,
        category: SnippetCategory::Main,
    },
    Snippet {
        trigger: "gpio_out",
        label: "GPIO Output (AVR LED Blink)",
        description: "デジタル出力ピン (LED点滅)",
        code: r#"let mut led = pins.d13.into_output();
loop {
    led.toggle();
    arduino_hal::delay_ms(500);
}"#,
        category: SnippetCategory::Gpio,
    },
    Snippet {
        trigger: "gpio_in",
        label: "GPIO Input (AVR)",
        description: "デジタル入力ピン",
        code: r#"let button = pins.d2.into_pull_up_input();
if button.is_low() {
    // ボタンが押されている
}"#,
        category: SnippetCategory::Gpio,
    },
    Snippet {
        trigger: "uart_avr",
        label: "UART Serial (AVR)",
        description: "シリアル通信の初期化と送受信",
        code: r#"let mut serial = arduino_hal::default_serial!(dp, pins, 115200);
ufmt::uwriteln!(&mut serial, "Hello!").unwrap();
let byte = nb::block!(serial.read()).unwrap();"#,
        category: SnippetCategory::Uart,
    },
    Snippet {
        trigger: "adc_avr",
        label: "ADC 読み取り (AVR)",
        description: "アナログ入力の読み取り",
        code: r#"let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());
let a0 = pins.a0.into_analog_input(&mut adc);
let value: u16 = a0.analog_read(&mut adc);"#,
        category: SnippetCategory::Adc,
    },
    Snippet {
        trigger: "spi_avr",
        label: "SPI 初期化 (AVR)",
        description: "SPI バスの初期化",
        code: r#"let (spi, cs) = arduino_hal::spi::Spi::new(
    dp.SPI,
    pins.d13.into_output(),        // SCK
    pins.d11.into_output(),        // MOSI
    pins.d12.into_pull_up_input(), // MISO
    pins.d10.into_output(),        // CS
    arduino_hal::spi::Settings::default(),
);"#,
        category: SnippetCategory::Spi,
    },
    Snippet {
        trigger: "i2c_avr",
        label: "I2C (TWI) 初期化 (AVR)",
        description: "I2C バスの初期化",
        code: r#"let mut i2c = arduino_hal::I2c::new(
    dp.TWI,
    pins.a4.into_pull_up_input(), // SDA
    pins.a5.into_pull_up_input(), // SCL
    50000,
);"#,
        category: SnippetCategory::I2c,
    },
];

// ─── ESP32 Xtensa (esp-hal) ─────────────────────────────

static ESP_XTENSA_SNIPPETS: &[Snippet] = &[
    Snippet {
        trigger: "esp32_init",
        label: "ESP32 esp-hal 初期化",
        description: "esp-hal のエントリーポイントと初期化",
        code: r#"#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl,
    peripherals::Peripherals,
    prelude::*,
    Delay,
};

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::max(system.clock_control).freeze();
    let mut delay = Delay::new(&clocks);

    loop {
        delay.delay_ms(1000u32);
    }
}"#,
        category: SnippetCategory::Main,
    },
    Snippet {
        trigger: "gpio_out_esp",
        label: "GPIO Output (ESP32)",
        description: "GPIO 出力ピン設定",
        code: r#"use esp_hal::gpio::{Io, Level, Output};
let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);
let mut led = Output::new(io.pins.gpio2, Level::Low);
led.set_high();"#,
        category: SnippetCategory::Gpio,
    },
    Snippet {
        trigger: "uart_esp",
        label: "UART (ESP32)",
        description: "UART 初期化と送信",
        code: r#"use esp_hal::uart::{Uart, config::Config};
let mut uart0 = Uart::new(peripherals.UART0, &clocks);
uart0.write_bytes(b"Hello ESP32!\r\n").unwrap();"#,
        category: SnippetCategory::Uart,
    },
    Snippet {
        trigger: "wifi_esp",
        label: "WiFi 初期化 (ESP32 esp-idf-hal)",
        description: "WiFi STA モード初期化",
        code: r#"// esp-idf-hal 使用時
use esp_idf_svc::wifi::{BlockingWifi, ClientConfiguration, Configuration, EspWifi};
let wifi = EspWifi::new(modem, sysloop.clone(), Some(nvs))?;
let mut wifi = BlockingWifi::wrap(wifi, sysloop)?;
wifi.set_configuration(&Configuration::Client(ClientConfiguration {
    ssid: "SSID".try_into().unwrap(),
    password: "PASSWORD".try_into().unwrap(),
    ..Default::default()
}))?;
wifi.start()?;
wifi.connect()?;"#,
        category: SnippetCategory::Misc,
    },
    Snippet {
        trigger: "i2c_esp",
        label: "I2C 初期化 (ESP32)",
        description: "I2C バスの初期化",
        code: r#"use esp_hal::i2c::I2C;
let i2c = I2C::new(
    peripherals.I2C0,
    io.pins.gpio21, // SDA
    io.pins.gpio22, // SCL
    100u32.kHz(),
    &clocks,
);"#,
        category: SnippetCategory::I2c,
    },
    Snippet {
        trigger: "spi_esp",
        label: "SPI 初期化 (ESP32)",
        description: "SPI バスの初期化",
        code: r#"use esp_hal::spi::{Spi, SpiMode};
let spi = Spi::new(peripherals.SPI2, 1u32.MHz(), SpiMode::Mode0, &clocks)
    .with_sck(io.pins.gpio18)
    .with_mosi(io.pins.gpio23)
    .with_miso(io.pins.gpio19);"#,
        category: SnippetCategory::Spi,
    },
];

// ─── ESP32-C3/C6 RISC-V (esp-hal) ───────────────────────

static ESP_RISCV_SNIPPETS: &[Snippet] = &[
    Snippet {
        trigger: "esp32c3_init",
        label: "ESP32-C3 esp-hal 初期化",
        description: "ESP32-C3 (RISC-V) エントリーポイント",
        code: r#"#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{clock::ClockControl, peripherals::Peripherals, prelude::*, Delay};

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::max(system.clock_control).freeze();
    let mut delay = Delay::new(&clocks);
    loop {
        delay.delay_ms(1000u32);
    }
}"#,
        category: SnippetCategory::Main,
    },
    Snippet {
        trigger: "gpio_out_c3",
        label: "GPIO Output (ESP32-C3)",
        description: "GPIO 出力 (RISC-V ESP32)",
        code: r#"use esp_hal::gpio::{Io, Level, Output};
let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);
let mut led = Output::new(io.pins.gpio8, Level::Low);
led.toggle();"#,
        category: SnippetCategory::Gpio,
    },
];

// ─── STM32 (stm32f4xx-hal 等) ───────────────────────────

static STM32_SNIPPETS: &[Snippet] = &[
    Snippet {
        trigger: "stm32_init",
        label: "STM32F4 HAL 初期化",
        description: "stm32f4xx-hal のエントリーポイント",
        code: r#"#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;
use stm32f4xx_hal::{pac, prelude::*};

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(168.MHz()).freeze();

    loop {}
}"#,
        category: SnippetCategory::Main,
    },
    Snippet {
        trigger: "gpio_out_stm",
        label: "GPIO Output (STM32)",
        description: "GPIO 出力ピン (LED トグル)",
        code: r#"let gpioa = dp.GPIOA.split();
let mut led = gpioa.pa5.into_push_pull_output();
led.set_high();
led.toggle();"#,
        category: SnippetCategory::Gpio,
    },
    Snippet {
        trigger: "gpio_in_stm",
        label: "GPIO Input (STM32)",
        description: "GPIO 入力ピン (プルアップ)",
        code: r#"let gpioc = dp.GPIOC.split();
let button = gpioc.pc13.into_pull_up_input();
if button.is_low() {
    // ボタンが押されている
}"#,
        category: SnippetCategory::Gpio,
    },
    Snippet {
        trigger: "uart_stm",
        label: "UART (STM32)",
        description: "USART2 の初期化",
        code: r#"use stm32f4xx_hal::serial::{Config, Serial};
let gpioa = dp.GPIOA.split();
let tx = gpioa.pa2.into_alternate();
let rx = gpioa.pa3.into_alternate();
let mut serial = Serial::new(dp.USART2, (tx, rx),
    Config::default().baudrate(115200.bps()), &clocks).unwrap();
use core::fmt::Write;
writeln!(serial, "Hello STM32!").unwrap();"#,
        category: SnippetCategory::Uart,
    },
    Snippet {
        trigger: "timer_stm",
        label: "Timer 割り込み (STM32)",
        description: "TIM2 タイマー割り込み設定",
        code: r#"use stm32f4xx_hal::timer::{Event, Timer};
let mut timer = Timer::new(dp.TIM2, &clocks).counter_hz();
timer.start(1.Hz()).unwrap();
timer.listen(Event::Update);
// NVIC で割り込み有効化
unsafe { cortex_m::peripheral::NVIC::unmask(pac::Interrupt::TIM2); }"#,
        category: SnippetCategory::Timer,
    },
    Snippet {
        trigger: "i2c_stm",
        label: "I2C (STM32)",
        description: "I2C1 の初期化",
        code: r#"use stm32f4xx_hal::i2c::I2c;
let gpiob = dp.GPIOB.split();
let scl = gpiob.pb6.into_alternate_open_drain();
let sda = gpiob.pb7.into_alternate_open_drain();
let i2c = I2c::new(dp.I2C1, (scl, sda), 400.kHz(), &clocks);"#,
        category: SnippetCategory::I2c,
    },
    Snippet {
        trigger: "spi_stm",
        label: "SPI (STM32)",
        description: "SPI1 の初期化",
        code: r#"use stm32f4xx_hal::spi::{Mode, Phase, Polarity, Spi};
let gpioa = dp.GPIOA.split();
let sck  = gpioa.pa5.into_alternate();
let miso = gpioa.pa6.into_alternate();
let mosi = gpioa.pa7.into_alternate();
let spi = Spi::new(dp.SPI1, (sck, miso, mosi),
    Mode { polarity: Polarity::IdleLow, phase: Phase::CaptureOnFirstTransition },
    1.MHz(), &clocks);"#,
        category: SnippetCategory::Spi,
    },
    Snippet {
        trigger: "adc_stm",
        label: "ADC 読み取り (STM32)",
        description: "ADC1 の初期化と読み取り",
        code: r#"use stm32f4xx_hal::adc::{Adc, config::AdcConfig};
let gpioa = dp.GPIOA.split();
let pa0 = gpioa.pa0.into_analog();
let mut adc = Adc::adc1(dp.ADC1, true, AdcConfig::default());
let sample: u16 = adc.read(&mut pa0).unwrap();"#,
        category: SnippetCategory::Adc,
    },
];

// ─── Raspberry Pi Pico (rp2040-hal) ────────────────────

static RPI_PICO_SNIPPETS: &[Snippet] = &[
    Snippet {
        trigger: "pico_init",
        label: "Raspberry Pi Pico HAL 初期化",
        description: "rp2040-hal エントリーポイント",
        code: r#"#![no_std]
#![no_main]

use bsp::entry;
use bsp::hal::{clocks::init_clocks_and_plls, pac, watchdog::Watchdog};
use panic_halt as _;
use rp_pico as bsp;

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let clocks = init_clocks_and_plls(
        bsp::XOSC_CRYSTAL_FREQ, pac.XOSC, pac.CLOCKS,
        pac.PLL_SYS, pac.PLL_USB, &mut pac.RESETS, &mut watchdog,
    ).ok().unwrap();

    let sio = bsp::hal::Sio::new(pac.SIO);
    let pins = bsp::Pins::new(pac.IO_BANK0, pac.PADS_BANK0, sio.gpio_bank0, &mut pac.RESETS);

    let mut led_pin = pins.led.into_push_pull_output();
    loop {
        led_pin.toggle().unwrap();
        cortex_m::asm::delay(500_000);
    }
}"#,
        category: SnippetCategory::Main,
    },
    Snippet {
        trigger: "gpio_pico",
        label: "GPIO Output (Pico LED)",
        description: "GPIO ピン出力 (LED トグル)",
        code: r#"let mut led_pin = pins.led.into_push_pull_output();
led_pin.set_high().unwrap();"#,
        category: SnippetCategory::Gpio,
    },
    Snippet {
        trigger: "uart_pico",
        label: "UART (Pico)",
        description: "UART0 の初期化",
        code: r#"use bsp::hal::uart::{DataBits, StopBits, UartConfig, UartPeripheral};
let uart_pins = (pins.gpio0.into_function(), pins.gpio1.into_function());
let uart = UartPeripheral::new(pac.UART0, uart_pins, &mut pac.RESETS)
    .enable(UartConfig::new(115200.Hz(), DataBits::Eight, None, StopBits::One), clocks.peripheral_clock.freq())
    .unwrap();
uart.write_full_blocking(b"Hello Pico!\r\n");"#,
        category: SnippetCategory::Uart,
    },
    Snippet {
        trigger: "i2c_pico",
        label: "I2C (Pico)",
        description: "I2C0 の初期化",
        code: r#"use bsp::hal::i2c::I2C;
let i2c = I2C::i2c0(
    pac.I2C0,
    pins.gpio4.into_function(), // SDA
    pins.gpio5.into_function(), // SCL
    400.kHz(),
    &mut pac.RESETS,
    &clocks.system_clock,
);"#,
        category: SnippetCategory::I2c,
    },
];

// ─── nRF52 (nrf-hal) ────────────────────────────────────

static NRF_SNIPPETS: &[Snippet] = &[
    Snippet {
        trigger: "nrf_init",
        label: "nRF52840 初期化",
        description: "nrf52840-hal エントリーポイント",
        code: r#"#![no_std]
#![no_main]

use cortex_m_rt::entry;
use nrf52840_hal::{gpio::Level, pac, prelude::*};
use panic_halt as _;

#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();
    let port0 = nrf52840_hal::gpio::p0::Parts::new(p.P0);
    let mut led = port0.p0_13.into_push_pull_output(Level::High);
    loop {
        led.set_low().unwrap();
        cortex_m::asm::delay(1_000_000);
        led.set_high().unwrap();
        cortex_m::asm::delay(1_000_000);
    }
}"#,
        category: SnippetCategory::Main,
    },
    Snippet {
        trigger: "uart_nrf",
        label: "UART (nRF52)",
        description: "UARTE0 の初期化",
        code: r#"use nrf52840_hal::uarte::{Baudrate, Parity, Uarte, UarteRx, UarteTx};
let uart = Uarte::new(p.UARTE0, nrf52840_hal::uarte::Pins {
    txd: port0.p0_06.into_push_pull_output(Level::High).degrade(),
    rxd: port0.p0_08.into_floating_input().degrade(),
    cts: None, rts: None,
}, Parity::EXCLUDED, Baudrate::BAUD115200);"#,
        category: SnippetCategory::Uart,
    },
];

// ─── SAMD (atsamd-hal) ──────────────────────────────────

static SAMD_SNIPPETS: &[Snippet] = &[Snippet {
    trigger: "samd_init",
    label: "SAMD21 HAL 初期化",
    description: "atsamd-hal エントリーポイント",
    code: r#"#![no_std]
#![no_main]

use atsamd_hal as hal;
use hal::clock::GenericClockController;
use hal::pac::Peripherals;
use hal::prelude::*;
use panic_halt as _;

#[cortex_m_rt::entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let _clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK, &mut peripherals.PM,
        &mut peripherals.SYSCTRL, &mut peripherals.NVMCTRL,
    );
    loop {}
}"#,
    category: SnippetCategory::Main,
}];

// ─── 汎用 Cortex-M ──────────────────────────────────────

static CORTEX_M_GENERIC_SNIPPETS: &[Snippet] = &[
    Snippet {
        trigger: "cortex_init",
        label: "Cortex-M 汎用初期化",
        description: "cortex-m-rt エントリーポイント",
        code: r#"#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;

#[entry]
fn main() -> ! {
    loop {
        cortex_m::asm::nop();
    }
}"#,
        category: SnippetCategory::Main,
    },
    Snippet {
        trigger: "interrupt",
        label: "割り込みハンドラ定義",
        description: "cortex-m-rt 割り込みハンドラ",
        code: r#"#[interrupt]
fn INTERRUPT_NAME() {
    // 割り込み処理
}"#,
        category: SnippetCategory::Interrupt,
    },
];

// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright 2026 rust-embedded-ide contributors

use super::BlinkTemplate;

pub fn samd21() -> BlinkTemplate {
    BlinkTemplate {
        main_rs: r#"//! Arduino Zero / SAMD21 Lチカ
//! LED: PA17 (D13)
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
"#,
        cargo_toml: r#"[package]
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
"#,
        cargo_config: r#"[build]
target = "thumbv6m-none-eabi"
"#,
        rust_toolchain: r#"[toolchain]
channel = "stable"
targets = ["thumbv6m-none-eabi"]
"#,
        memory_x: Some("/*******************************************************************************
Memory layout for SAMD21
FLASH ORIGIN = 0x00000000 LENGTH = 256K
RAM   ORIGIN = 0x20000000 LENGTH = 32K
*******************************************************************************/"),
        build_rs: None,
        linker_ld: None,
        target_json: None,
    }
}

pub fn samd51() -> BlinkTemplate {
    BlinkTemplate {
        main_rs: r#"//! SAMD51 Lチカ
//! LED: PA23
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
"#,
        cargo_toml: r#"[package]
name = "blink"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "blink"
test = false
bench = false

[dependencies]
feather-m4 = "0.10"
cortex-m = "0.7"
cortex-m-rt = "0.7"
panic-halt = "0.2"
"#,
        cargo_config: r#"[build]
target = "thumbv7em-none-eabihf"
"#,
        rust_toolchain: r#"[toolchain]
channel = "stable"
targets = ["thumbv7em-none-eabihf"]
"#,
        memory_x: Some("/*******************************************************************************
Memory layout for SAMD51
FLASH ORIGIN = 0x00000000 LENGTH = 512K
RAM   ORIGIN = 0x20000000 LENGTH = 192K
*******************************************************************************/"),
        build_rs: None,
        linker_ld: None,
        target_json: None,
    }
}

pub fn arduino_due() -> BlinkTemplate {
    BlinkTemplate {
        main_rs: r#"//! Arduino Due (SAM3X8E) Lチカ
//! LED: PB27 / D13
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;

#[entry]
fn main() -> ! {
    // Minimal placeholder for Due; BSPs vary. User should adapt.
    loop {
        cortex_m::asm::nop();
    }
}
"#,
        cargo_toml: r#"[package]
name = "blink"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "blink"
test = false
bench = false

[dependencies]
# Platform-specific HALs for SAM3x are varied; leave minimal deps
cortex-m = "0.7"
cortex-m-rt = "0.7"
panic-halt = "0.2"
"#,
        cargo_config: r#"[build]
target = "thumbv7em-none-eabi"
"#,
        rust_toolchain: r#"[toolchain]
channel = "stable"
"#,
        memory_x: Some("/*******************************************************************************
Memory layout suggestion for Arduino Due (adjust as needed)
FLASH ORIGIN = 0x00000000 LENGTH = 512K
RAM   ORIGIN = 0x20070000 LENGTH = 96K
*******************************************************************************/"),
        build_rs: None,
        linker_ld: None,
        target_json: None,
    }
}

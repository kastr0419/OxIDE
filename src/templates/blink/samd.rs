// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright 2026 ALLoIDE contributors

use super::BlinkTemplate;

const BUILD_RS: &str = r#"use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("memory.x")).unwrap().write_all(include_bytes!("memory.x")).unwrap();
    println!("cargo:rustc-link-search={}", out.display());
    println!("cargo:rerun-if-changed=memory.x");
}
"#;

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
        memory_x: Some(
            r#"MEMORY
{
  FLASH : ORIGIN = 0x00000000, LENGTH = 256K
  RAM : ORIGIN = 0x20000000, LENGTH = 32K
}
"#,
        ),
        build_rs: Some(BUILD_RS),
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
        memory_x: Some(
            r#"MEMORY
{
  FLASH : ORIGIN = 0x00000000, LENGTH = 512K
  RAM : ORIGIN = 0x20000000, LENGTH = 192K
}
"#,
        ),
        build_rs: Some(BUILD_RS),
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
use cortex_m::asm;
use panic_halt as _;

#[entry]
fn main() -> ! {
    // Simple timing loop. Replace with board-specific GPIO toggling if desired.
    loop {
        asm::delay(8_000_000);
        asm::delay(8_000_000);
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
target = "thumbv7m-none-eabi"
"#,
        rust_toolchain: r#"[toolchain]
channel = "stable"
targets = ["thumbv7m-none-eabi"]
"#,
        memory_x: Some(
            r#"MEMORY
{
  FLASH : ORIGIN = 0x00080000, LENGTH = 512K
  RAM : ORIGIN = 0x20000000, LENGTH = 96K
}
"#,
        ),
        build_rs: Some(BUILD_RS),
        linker_ld: None,
        target_json: None,
    }
}

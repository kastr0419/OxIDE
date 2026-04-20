// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright 2026 rust-embedded-ide contributors

use super::BlinkTemplate;

// Common build.rs used to copy memory.x into OUT_DIR
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

pub fn nrf52840() -> BlinkTemplate {
    BlinkTemplate {
        main_rs: r#"//! nRF52840 DK Lチカ
//! LED: P0.13 (DK LED1)
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use nrf52840_hal::{gpio::{Level, Output, PushPull, p0}, pac, prelude::*};
use panic_halt as _;

#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();
    let port0 = p0::Parts::new(p.P0);
    let mut led1: Output<PushPull> = port0.p0_13.into_push_pull_output(Level::Low).degrade();
    loop {
        led1.set_low().unwrap();   // LED ON (active LOW)
        cortex_m::asm::delay(64_000_000);
        led1.set_high().unwrap();  // LED OFF
        cortex_m::asm::delay(64_000_000);
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
nrf52840-hal = "0.16"
cortex-m = "0.7"
cortex-m-rt = "0.7"
panic-halt = "0.2"

[profile.dev]
opt-level = "s"
"#,
        cargo_config: r#"[build]
target = "thumbv7em-none-eabihf"

[target.thumbv7em-none-eabihf]
runner = "probe-rs run --chip nRF52840_xxAA"
"#, 
        rust_toolchain: r#"[toolchain]
channel = "stable"
targets = ["thumbv7em-none-eabihf"]
"#,
        memory_x: Some(r#"MEMORY
{
  FLASH : ORIGIN = 0x00000000, LENGTH = 1024K
  RAM :   ORIGIN = 0x20000000, LENGTH = 256K
}
"#),
        build_rs: Some(BUILD_RS),
        linker_ld: None,
        target_json: None,
    }
}

pub fn nrf51822() -> BlinkTemplate {
    BlinkTemplate {
        main_rs: r#"//! nRF51822 Lチカ
//! LED: P0.21
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use nrf51_hal::{gpio::{p0, Level, Output, PushPull}, pac, prelude::*};
use panic_halt as _;

#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();
    let port0 = p0::Parts::new(p.P0);
    let mut led: Output<PushPull> = port0.p0_21.into_push_pull_output(Level::Low).degrade();
    loop {
        led.set_low().unwrap();   // LED ON
        cortex_m::asm::delay(8_000_000);
        led.set_high().unwrap();  // LED OFF
        cortex_m::asm::delay(8_000_000);
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
nrf51-hal = "0.14"
cortex-m = "0.7"
cortex-m-rt = "0.7"
panic-halt = "0.2"
"#,
        cargo_config: r#"[build]
target = "thumbv6m-none-eabi"

[target.thumbv6m-none-eabi]
runner = "probe-rs run --chip nRF51422_xxAC"
"#, 
        rust_toolchain: r#"[toolchain]
channel = "stable"
targets = ["thumbv6m-none-eabi"]
"#,
        memory_x: Some(r#"MEMORY
{
  FLASH : ORIGIN = 0x00000000, LENGTH = 256K
  RAM : ORIGIN = 0x20000000, LENGTH = 16K
}
"#),
        build_rs: Some(BUILD_RS),
        linker_ld: None,
        target_json: None,
    }
}

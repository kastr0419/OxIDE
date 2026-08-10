// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright 2026 rust-embedded-ide contributors

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

pub fn teensy4() -> BlinkTemplate {
    BlinkTemplate {
        main_rs: r#"//! Teensy 4.0 Lチカ
//! LED: pin 13 (GPIO_B0_03)
#![no_std]
#![no_main]

use teensy4_bsp as bsp;
use bsp::board;
use embedded_hal::digital::v2::OutputPin;
use panic_halt as _;

#[bsp::rt::entry]
fn main() -> ! {
    let instances = board::instances();
    let board::Resources { mut gpio2, pins, .. } = board::t40(instances);
    let mut led = bsp::board::led(&mut gpio2, pins.p13);
    
    loop {
        led.set_high().unwrap();
        cortex_m::asm::delay(600_000_000 / 2);
        led.set_low().unwrap();
        cortex_m::asm::delay(600_000_000 / 2);
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
teensy4-bsp = "0.5"
cortex-m = "0.7"
cortex-m-rt = "0.7"
embedded-hal = "0.2"
panic-halt = "0.2"
"#,
        cargo_config: r#"[build]
target = "thumbv7em-none-eabihf"

[target.thumbv7em-none-eabihf]
runner = "teensy_loader_cli --mcu=TEENSY40 -w"
"#,
        rust_toolchain: r#"[toolchain]
channel = "stable"
targets = ["thumbv7em-none-eabihf"]
"#,
        memory_x: Some(
            r#"MEMORY
{
  FLASH : ORIGIN = 0x60000000, LENGTH = 1984K
  RAM : ORIGIN = 0x20200000, LENGTH = 512K
}
"#,
        ),
        build_rs: Some(BUILD_RS),
        linker_ld: None,
        target_json: None,
    }
}

// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright 2026 rust-embedded-ide contributors

use super::BlinkTemplate;

pub fn teensy4() -> BlinkTemplate {
    BlinkTemplate {
        main_rs: r#"//! Teensy 4.0 Lチカ
//! LED: pin 13 (GPIO_B0_03)
#![no_std]
#![no_main]

use teensy4_bsp as bsp;
use bsp::board;
use embedded_hal::digital::OutputPin;
use panic_halt as _;

#[bsp::rt::entry]
fn main() -> ! {
    let instances = board::instances();
    let board::Resources { mut gpio2, pins, .. } = board::t40(instances);
    let mut led = bsp::board::led(&mut gpio2, pins.p13);
    
    loop {
        led.set_high().unwrap();
        bsp::ral::modify_reg!(bsp::ral::gpt, instances.GPT1, CR, EN: 1);
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
embedded-hal = "1.0"
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
Memory layout for Teensy 4.0 (IMXRT1062)
FLASH ORIGIN = 0x60000000 LENGTH = 2M
RAM   ORIGIN = 0x20200000 LENGTH = 512K
*******************************************************************************/"),
        build_rs: None,
        linker_ld: None,
        target_json: None,
    }
}

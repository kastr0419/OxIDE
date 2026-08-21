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

pub fn gd32vf103() -> BlinkTemplate {
    BlinkTemplate {
        main_rs: r#"//! GD32VF103 Lチカ
//! LED: PB0
#![no_std]
#![no_main]

use gd32vf103xx_hal::{pac, prelude::*, gpio::{GpioExt, Output, PushPull, gpiob::PB0}};
use riscv_rt::entry;
use panic_halt as _;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let mut rcu = dp.RCU.configure().freeze();
    let mut gpiob = dp.GPIOB.split(&mut rcu);
    let mut led: PB0<Output<PushPull>> = gpiob.pb0.into_push_pull_output(&mut gpiob.config);
    loop {
        led.set_low().unwrap();
        riscv::asm::delay(8_000_000);
        led.set_high().unwrap();
        riscv::asm::delay(8_000_000);
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
gd32vf103xx-hal = "0.4"
riscv = "0.10"
riscv-rt = "0.12"
panic-halt = "0.2"
"#,
        cargo_config: r#"[build]
target = "riscv32imac-unknown-none-elf"
"#,
        rust_toolchain: r#"[toolchain]
channel = "stable"
targets = ["riscv32imac-unknown-none-elf"]
"#,
        memory_x: Some(
            r#"MEMORY
{
  FLASH : ORIGIN = 0x08000000, LENGTH = 128K
  RAM : ORIGIN = 0x20000000, LENGTH = 32K
}
"#,
        ),
        build_rs: Some(BUILD_RS),
        linker_ld: None,
        target_json: None,
    }
}

pub fn ch32v003() -> BlinkTemplate {
    BlinkTemplate {
        main_rs: r#"//! CH32V003 Lチカ
//! LED: PD0 (CH32V003F4P6)
#![no_std]
#![no_main]

use ch32v_hal::{pac, prelude::*, gpio::GpioExt};
use panic_halt as _;

#[ch32v_hal::entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let rcc = dp.RCC.constrain();
    let _ = rcc.cfgr.freeze();
    let gpiod = dp.GPIOD.split();
    let mut led = gpiod.pd0.into_push_pull_output();
    loop {
        led.set_low().unwrap();  // LED ON
        unsafe { riscv::asm::delay(480_000) };
        led.set_high().unwrap(); // LED OFF
        unsafe { riscv::asm::delay(480_000) };
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
ch32v-hal = "0.1"
riscv = "0.10"
riscv-rt = "0.12"
panic-halt = "0.2"
"#,
        cargo_config: r#"[build]
target = "riscv32imc-unknown-none-elf"
"#,
        rust_toolchain: r#"[toolchain]
channel = "stable"
targets = ["riscv32imc-unknown-none-elf"]
"#,
        memory_x: Some(
            r#"MEMORY
{
  FLASH : ORIGIN = 0x08000000, LENGTH = 16K
  RAM : ORIGIN = 0x20000000, LENGTH = 2K
}
"#,
        ),
        build_rs: Some(BUILD_RS),
        linker_ld: None,
        target_json: None,
    }
}

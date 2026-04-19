// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright 2026 rust-embedded-ide contributors

use super::BlinkTemplate;

// Common build.rs used to copy memory.x into OUT_DIR
const BUILD_RS: &str = r#"use std::env; use std::fs::File; use std::io::Write; use std::path::PathBuf;
fn main() {
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("memory.x")).unwrap().write_all(include_bytes!("memory.x")).unwrap();
    println!("cargo:rustc-link-search={}", out.display());
    println!("cargo:rerun-if-changed=memory.x");
}"#;

pub fn stm32f1() -> BlinkTemplate {
    BlinkTemplate {
        main_rs: r#"//! STM32F1 (Blue Pill) Lチカ
//! LED: PC13 (アクティブLOW)
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;
use stm32f1xx_hal::{pac, prelude::*, timer::Timer};

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut gpioc = dp.GPIOC.split();
    // PC13はアクティブLOW（LOW=LED ON）
    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
    let mut delay = cp.SYST.delay(&clocks);
    loop {
        led.set_low();   // LED ON
        delay.delay_ms(500u32);
        led.set_high();  // LED OFF
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
stm32f1xx-hal = { version = "0.10", features = ["stm32f103", "medium"] }
cortex-m-rt = "0.7"
cortex-m = "0.7"
panic-halt = "0.2"

[profile.dev]
panic = "abort"
lto = true
opt-level = "s"

[profile.release]
lto = true
opt-level = "s"
"#,
        cargo_config: r#"[build]
target = "thumbv7m-none-eabi"

[target.thumbv7m-none-eabi]
runner = "probe-rs run --chip STM32F103C8"
rustflags = [
    "-C", "link-arg=-Tlink.x",
]
"#,
        rust_toolchain: r#"[toolchain]
channel = "stable"
targets = ["thumbv7m-none-eabi"]
"#,
        memory_x: Some(r#"MEMORY
{
  FLASH : ORIGIN = 0x08000000, LENGTH = 64K
  RAM   : ORIGIN = 0x20000000, LENGTH = 20K
}
"#),
        build_rs: Some(BUILD_RS),
        linker_ld: None,
        target_json: None,
    }
}

pub fn stm32f4() -> BlinkTemplate {
    BlinkTemplate {
        main_rs: r#"//! STM32F4 (Black Pill F401) Lチカ
//! LED: PC13 (アクティブLOW)
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;
use stm32f4xx_hal::{pac, prelude::*};

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(84.MHz()).freeze();
    let gpioc = dp.GPIOC.split();
    let mut led = gpioc.pc13.into_push_pull_output();
    let mut delay = dp.TIM2.delay_ms(&clocks);
    loop {
        led.set_low();
        delay.delay_ms(500u32);
        led.set_high();
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
stm32f4xx-hal = { version = "0.21", features = ["stm32f401"] }
cortex-m-rt = "0.7"
cortex-m = "0.7"
panic-halt = "0.2"

[profile.dev]
panic = "abort"
lto = true
opt-level = "s"

[profile.release]
lto = true
opt-level = "s"
"#,
        cargo_config: r#"[build]
target = "thumbv7em-none-eabihf"

[target.thumbv7em-none-eabihf]
runner = "probe-rs run --chip STM32F401RE"
rustflags = [
    "-C", "link-arg=-Tlink.x",
]
"#,
        rust_toolchain: r#"[toolchain]
channel = "stable"
targets = ["thumbv7em-none-eabihf"]
"#,
        memory_x: Some(r#"MEMORY
{
  FLASH : ORIGIN = 0x08000000, LENGTH = 512K
  RAM   : ORIGIN = 0x20000000, LENGTH = 96K
}
"#),
        build_rs: Some(BUILD_RS),
        linker_ld: None,
        target_json: None,
    }
}

pub fn stm32l4() -> BlinkTemplate {
    BlinkTemplate {
        main_rs: r#"//! STM32L4 Lチカ
//! LED: PB13
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;
use stm32l4xx_hal::{pac, prelude::*};

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut gpiob = dp.GPIOB.split();
    let mut led = gpiob.pb13.into_push_pull_output();
    let mut delay = cp.SYST.delay(&clocks);

    loop {
        led.set_high();
        delay.delay_ms(500u32);
        led.set_low();
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
stm32l4xx-hal = { version = "0.7", features = ["stm32l432kc"] }
cortex-m-rt = "0.7"
cortex-m = "0.7"
panic-halt = "0.2"

[profile.dev]
panic = "abort"
lto = true
opt-level = "s"

[profile.release]
lto = true
opt-level = "s"
"#,
        cargo_config: r#"[build]
target = "thumbv7em-none-eabihf"

[target.thumbv7em-none-eabihf]
runner = "probe-rs run --chip STM32L432KC"
rustflags = [
    "-C", "link-arg=-Tlink.x",
]
"#,
        rust_toolchain: r#"[toolchain]
channel = "stable"
targets = ["thumbv7em-none-eabihf"]
"#,
        memory_x: Some(r#"MEMORY
{
  FLASH : ORIGIN = 0x08000000, LENGTH = 256K
  RAM   : ORIGIN = 0x20000000, LENGTH = 64K
}
"#),
        build_rs: Some(BUILD_RS),
        linker_ld: None,
        target_json: None,
    }
}

pub fn stm32f7() -> BlinkTemplate {
    BlinkTemplate {
        main_rs: r#"//! STM32F7 (Nucleo-F767ZI) Lチカ
//! LED: PB7
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;
use stm32f7xx_hal::{pac, prelude::*};

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut gpiob = dp.GPIOB.split();
    let mut led = gpiob.pb7.into_push_pull_output();
    let mut delay = cp.SYST.delay(&clocks);

    loop {
        led.set_high();
        delay.delay_ms(500u32);
        led.set_low();
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
stm32f7xx-hal = { version = "0.7", features = ["stm32f767"] }
cortex-m-rt = "0.7"
cortex-m = "0.7"
panic-halt = "0.2"

[profile.dev]
panic = "abort"
lto = true
opt-level = "s"

[profile.release]
lto = true
opt-level = "s"
"#,
        cargo_config: r#"[build]
target = "thumbv7em-none-eabihf"

[target.thumbv7em-none-eabihf]
runner = "probe-rs run --chip STM32F767ZI"
rustflags = [
    "-C", "link-arg=-Tlink.x",
]
"#,
        rust_toolchain: r#"[toolchain]
channel = "stable"
targets = ["thumbv7em-none-eabihf"]
"#,
        memory_x: Some(r#"MEMORY
{
  FLASH : ORIGIN = 0x08000000, LENGTH = 2M
  RAM   : ORIGIN = 0x20020000, LENGTH = 512K
}
"#),
        build_rs: Some(BUILD_RS),
        linker_ld: None,
        target_json: None,
    }
}

pub fn stm32h7() -> BlinkTemplate {
    BlinkTemplate {
        main_rs: r#"//! STM32H7 (Nucleo-H743ZI2) Lチカ
//! LED: PB14
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;
use stm32h7xx_hal::{pac, prelude::*};

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut gpiob = dp.GPIOB.split();
    let mut led = gpiob.pb14.into_push_pull_output();
    let mut delay = cp.SYST.delay(&clocks);

    loop {
        led.set_high();
        delay.delay_ms(500u32);
        led.set_low();
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
stm32h7xx-hal = { version = "0.16", features = ["stm32h743"] }
cortex-m-rt = "0.7"
cortex-m = "0.7"
panic-halt = "0.2"

[profile.dev]
panic = "abort"
lto = true
opt-level = "s"

[profile.release]
lto = true
opt-level = "s"
"#,
        cargo_config: r#"[build]
target = "thumbv7em-none-eabihf"

[target.thumbv7em-none-eabihf]
runner = "probe-rs run --chip STM32H743ZI"
rustflags = [
    "-C", "link-arg=-Tlink.x",
]
"#,
        rust_toolchain: r#"[toolchain]
channel = "stable"
targets = ["thumbv7em-none-eabihf"]
"#,
        memory_x: Some(r#"MEMORY
{
  FLASH : ORIGIN = 0x08000000, LENGTH = 2M
  RAM   : ORIGIN = 0x24000000, LENGTH = 512K
}
"#),
        build_rs: Some(BUILD_RS),
        linker_ld: None,
        target_json: None,
    }
}

pub fn stm32g0() -> BlinkTemplate {
    BlinkTemplate {
        main_rs: r#"//! STM32G0 (Nucleo-G031K8) Lチカ
//! LED: PA5
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;
use stm32g0xx_hal::{pac, prelude::*};

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut gpioa = dp.GPIOA.split();
    let mut led = gpioa.pa5.into_push_pull_output();
    let mut delay = cp.SYST.delay(&clocks);

    loop {
        led.set_high();
        delay.delay_ms(500u32);
        led.set_low();
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
stm32g0xx-hal = { version = "0.2", features = ["stm32g031"] }
cortex-m-rt = "0.7"
cortex-m = "0.7"
panic-halt = "0.2"

[profile.dev]
panic = "abort"
lto = true
opt-level = "s"

[profile.release]
lto = true
opt-level = "s"
"#,
        cargo_config: r#"[build]
target = "thumbv6m-none-eabi"

[target.thumbv6m-none-eabi]
runner = "probe-rs run --chip STM32G031K8"
rustflags = [
    "-C", "link-arg=-Tlink.x",
]
"#,
        rust_toolchain: r#"[toolchain]
channel = "stable"
targets = ["thumbv6m-none-eabi"]
"#,
        memory_x: Some(r#"MEMORY
{
  FLASH : ORIGIN = 0x08000000, LENGTH = 64K
  RAM   : ORIGIN = 0x20000000, LENGTH = 8K
}
"#),
        build_rs: Some(BUILD_RS),
        linker_ld: None,
        target_json: None,
    }
}
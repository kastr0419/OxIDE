// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright 2026 rust-embedded-ide contributors

use super::BlinkTemplate;

pub fn rpi_pico() -> BlinkTemplate {
    BlinkTemplate {
        main_rs: r#"//! Raspberry Pi Pico Lチカ
//! LED: GPIO25 (オンボードLED)
#![no_std]
#![no_main]

use bsp::entry;
use bsp::hal::{clocks::init_clocks_and_plls, pac, sio::Sio, watchdog::Watchdog};
use embedded_hal::delay::DelayNs;
use panic_halt as _;
use rp_pico as bsp;

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);

    let clocks = init_clocks_and_plls(
        bsp::XOSC_CRYSTAL_FREQ,
        pac.XOSC, pac.CLOCKS, pac.PLL_SYS, pac.PLL_USB,
        &mut pac.RESETS, &mut watchdog,
    ).ok().unwrap();

    let mut delay = cortex_m::delay::Delay::new(
        core.SYST, clocks.system_clock.freq().to_Hz()
    );

    let pins = bsp::Pins::new(
        pac.IO_BANK0, pac.PADS_BANK0, sio.gpio_bank0, &mut pac.RESETS,
    );

    // GPIO25 = オンボードLED
    let mut led = pins.led.into_push_pull_output();

    loop {
        led.set_high().unwrap();
        delay.delay_ms(500);
        led.set_low().unwrap();
        delay.delay_ms(500);
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
rp-pico = "0.9"
cortex-m = { version = "0.7", features = ["critical-section-single-core"] }
cortex-m-rt = "0.7"
embedded-hal = "1.0"
panic-halt = "0.2"

[profile.release]
lto = true
opt-level = "s"
"#,
        cargo_config: r#"[build]
target = "thumbv6m-none-eabi"

[target.thumbv6m-none-eabi]
runner = "elf2uf2-rs -d"
rustflags = [
    "-C", "link-arg=-Tlink.x",
    "-C", "link-arg=--nmagic",
]
"#,
        rust_toolchain: r#"[toolchain]
channel = "stable"
targets = ["thumbv6m-none-eabi"]
"#,
        memory_x: None, // rp-pico BSP が内部で提供
        build_rs: None,
        linker_ld: None,
        target_json: None,
    }
}

pub fn rpi_pico2()-> BlinkTemplate {
    BlinkTemplate {
        main_rs: r#"//! Raspberry Pi Pico 2 Lチカ
//! LED: GPIO25 (オンボードLED, RP2350)
#![no_std]
#![no_main]

use bsp::entry;
use bsp::hal::{clocks::init_clocks_and_plls, pac, sio::Sio, watchdog::Watchdog};
use embedded_hal::delay::DelayNs;
use panic_halt as _;
use rp235x_hal as bsp;

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);

    let clocks = init_clocks_and_plls(
        12_000_000u32,
        pac.XOSC, pac.CLOCKS, pac.PLL_SYS, pac.PLL_USB,
        &mut pac.RESETS, &mut watchdog,
    ).ok().unwrap();

    let mut delay = cortex_m::delay::Delay::new(
        core.SYST, clocks.system_clock.freq().to_Hz()
    );

    let pins = bsp::gpio::Pins::new(
        pac.IO_BANK0, pac.PADS_BANK0, sio.gpio_bank0, &mut pac.RESETS,
    );

    // GPIO25 = オンボードLED
    let mut led = pins.gpio25.into_push_pull_output();

    loop {
        use embedded_hal::digital::OutputPin;
        led.set_high().unwrap();
        delay.delay_ms(500);
        led.set_low().unwrap();
        delay.delay_ms(500);
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
rp235x-hal = "0.2"
cortex-m = { version = "0.7", features = ["critical-section-single-core"] }
cortex-m-rt = "0.7"
embedded-hal = "1.0"
panic-halt = "0.2"

[profile.release]
lto = true
opt-level = "s"
"#,
        cargo_config: r#"[build]
target = "thumbv8m.main-none-eabihf"

[target.thumbv8m.main-none-eabihf]
runner = "elf2uf2-rs -d"
rustflags = [
    "-C", "link-arg=-Tlink.x",
]
"#,
        rust_toolchain: r#"[toolchain]
channel = "stable"
targets = ["thumbv8m.main-none-eabihf"]
"#,
        memory_x: Some(r#"MEMORY
{
    FLASH : ORIGIN = 0x10000000, LENGTH = 4M
    RAM   : ORIGIN = 0x20000000, LENGTH = 520K
}
"#),
        build_rs: None,
        linker_ld: None,
        target_json: None,
    }
}
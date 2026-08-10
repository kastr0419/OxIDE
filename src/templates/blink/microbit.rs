// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright 2026 rust-embedded-ide contributors

use super::BlinkTemplate;

pub fn microbit_v2() -> BlinkTemplate {
    BlinkTemplate {
        main_rs: r#"//! micro:bit v2 ボタン + LED ディスプレイ例
//! ボタンA: HEART パターン表示
//! ボタンB: SMILE パターン表示
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use microbit::{
    board::Board,
    display::blocking::Display,
    hal::Timer,
};
use embedded_hal::digital::InputPin;
use panic_halt as _;

const HEART: [[u8; 5]; 5] = [
    [0, 1, 0, 1, 0],
    [1, 1, 1, 1, 1],
    [1, 1, 1, 1, 1],
    [0, 1, 1, 1, 0],
    [0, 0, 1, 0, 0],
];

const SMILE: [[u8; 5]; 5] = [
    [0, 1, 0, 1, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
    [1, 0, 0, 0, 1],
    [0, 1, 1, 1, 0],
];

const ALL_OFF: [[u8; 5]; 5] = [[0; 5]; 5];

#[entry]
fn main() -> ! {
    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);
    let mut buttons = board.buttons;

    loop {
        if buttons.button_a.is_low().unwrap() {
            display.show(&mut timer, HEART, 1000);
        } else if buttons.button_b.is_low().unwrap() {
            display.show(&mut timer, SMILE, 1000);
        } else {
            display.show(&mut timer, ALL_OFF, 50);
        }
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
microbit-v2 = "0.15"
embedded-hal = "1"
cortex-m = "0.7"
cortex-m-rt = "0.7"
panic-halt = "0.2"

[profile.release]
lto = true
opt-level = "s"
"#,
        cargo_config: r#"[build]
target = "thumbv7em-none-eabihf"

[target.thumbv7em-none-eabihf]
runner = "probe-rs run --chip NRF52833"
rustflags = [
    "-C", "link-arg=-Tlink.x",
]
"#,
        rust_toolchain: r#"[toolchain]
channel = "stable"
targets = ["thumbv7em-none-eabihf"]
"#,
        memory_x: Some(
            r#"MEMORY
{
    FLASH : ORIGIN = 0x00000000, LENGTH = 512K
    RAM   : ORIGIN = 0x20000000, LENGTH = 128K
}
"#,
        ),
        build_rs: Some(
            r#"use std::env; use std::fs::File; use std::io::Write; use std::path::PathBuf;
fn main() {
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("memory.x")).unwrap().write_all(include_bytes!("memory.x")).unwrap();
    println!("cargo:rustc-link-search={}", out.display());
    println!("cargo:rerun-if-changed=memory.x");
}"#,
        ),
        linker_ld: None,
        target_json: None,
    }
}

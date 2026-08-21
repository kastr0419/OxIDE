// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright 2026 ALLoIDE contributors

use super::BlinkTemplate;

const ATMEGA328P_JSON: &str = r#"{
  "llvm-target": "avr-unknown-unknown",
  "cpu": "atmega328p",
  "data-layout": "e-P1-p:16:8-i8:8-i16:8-i32:8-i64:8-f32:8-f64:8-n8-a:8",
  "arch": "avr",
  "target-endian": "little",
  "target-pointer-width": "16",
  "target-c-int-width": "16",
  "os": "none",
  "env": "",
  "vendor": "unknown",
  "linker-flavor": "gcc",
  "linker": "avr-gcc",
  "pre-link-args": {
    "gcc": ["-mmcu=atmega328p", "-Os"]
  },
  "exe-suffix": ".elf",
  "executables": true,
  "max-atomic-width": 8,
  "atomic-cas": false,
  "panic-strategy": "abort",
  "relocation-model": "pic"
}"#;

const ATMEGA2560_JSON: &str = r#"{
  "llvm-target": "avr-unknown-unknown",
  "cpu": "atmega2560",
  "data-layout": "e-P1-p:16:8-i8:8-i16:8-i32:8-i64:8-f32:8-f64:8-n8-a:8",
  "arch": "avr",
  "target-endian": "little",
  "target-pointer-width": "16",
  "target-c-int-width": "16",
  "os": "none",
  "env": "",
  "vendor": "unknown",
  "linker-flavor": "gcc",
  "linker": "avr-gcc",
  "pre-link-args": {
    "gcc": ["-mmcu=atmega2560", "-Os"]
  },
  "exe-suffix": ".elf",
  "executables": true,
  "max-atomic-width": 8,
  "atomic-cas": false,
  "panic-strategy": "abort",
  "relocation-model": "pic"
}"#;

const ATMEGA32U4_JSON: &str = r#"{
  "llvm-target": "avr-unknown-unknown",
  "cpu": "atmega32u4",
  "data-layout": "e-P1-p:16:8-i8:8-i16:8-i32:8-i64:8-f32:8-f64:8-n8-a:8",
  "arch": "avr",
  "target-endian": "little",
  "target-pointer-width": "16",
  "target-c-int-width": "16",
  "os": "none",
  "env": "",
  "vendor": "unknown",
  "linker-flavor": "gcc",
  "linker": "avr-gcc",
  "pre-link-args": {
    "gcc": ["-mmcu=atmega32u4", "-Os"]
  },
  "exe-suffix": ".elf",
  "executables": true,
  "max-atomic-width": 8,
  "atomic-cas": false,
  "panic-strategy": "abort",
  "relocation-model": "pic"
}"#;

pub fn arduino_uno() -> BlinkTemplate {
    BlinkTemplate {
        main_rs: r#"//! Arduino Uno Lチカ
//! LED: D13 (PB5)
#![no_std]
#![no_main]

use panic_halt as _;
use arduino_hal::prelude::*;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    // D13 (PB5) をオンボードLEDとして設定
    let mut led = pins.d13.into_output();

    loop {
        led.toggle();
        arduino_hal::delay_ms(500);
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
arduino-hal = { git = "https://github.com/Rahix/avr-hal", features = ["arduino-uno"] }
panic-halt = "0.2"

[profile.dev]
panic = "abort"
lto = true
opt-level = "s"

[profile.release]
panic = "abort"
lto = true
opt-level = "s"
"#,
        cargo_config: r#"[build]
target = "avr-atmega328p.json"

[unstable]
build-std = ["core"]

[target.'cfg(target_arch = "avr")']
runner = "ravedude uno -cb 115200"
rustflags = ["-C", "opt-level=s", "-C", "target-cpu=atmega328p"]
"#,
        rust_toolchain: r#"[toolchain]
channel = "nightly"
components = ["rust-src"]
profile = "minimal"
"#,
        memory_x: None,
        build_rs: None,
        linker_ld: None,
        target_json: Some(("avr-atmega328p.json", ATMEGA328P_JSON)),
    }
}

pub fn arduino_nano() -> BlinkTemplate {
    BlinkTemplate {
        main_rs: r#"//! Arduino Nano Lチカ
//! LED: D13 (PB5)
#![no_std]
#![no_main]

use panic_halt as _;
use arduino_hal::prelude::*;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut led = pins.d13.into_output();

    loop {
        led.toggle();
        arduino_hal::delay_ms(500);
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
arduino-hal = { git = "https://github.com/Rahix/avr-hal", features = ["arduino-nano"] }
panic-halt = "0.2"

[profile.dev]
panic = "abort"
lto = true
opt-level = "s"

[profile.release]
panic = "abort"
lto = true
opt-level = "s"
"#,
        cargo_config: r#"[build]
target = "avr-atmega328p.json"

[unstable]
build-std = ["core"]

[target.'cfg(target_arch = "avr")']
runner = "ravedude nano -cb 115200"
rustflags = ["-C", "opt-level=s", "-C", "target-cpu=atmega328p"]
"#,
        rust_toolchain: r#"[toolchain]
channel = "nightly"
components = ["rust-src"]
profile = "minimal"
"#,
        memory_x: None,
        build_rs: None,
        linker_ld: None,
        target_json: Some(("avr-atmega328p.json", ATMEGA328P_JSON)),
    }
}

pub fn arduino_mega() -> BlinkTemplate {
    BlinkTemplate {
        main_rs: r#"//! Arduino Mega 2560 Lチカ
//! LED: D13 (PB7)
#![no_std]
#![no_main]

use panic_halt as _;
use arduino_hal::prelude::*;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut led = pins.d13.into_output();

    loop {
        led.toggle();
        arduino_hal::delay_ms(500);
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
arduino-hal = { git = "https://github.com/Rahix/avr-hal", features = ["arduino-mega2560"] }
panic-halt = "0.2"

[profile.dev]
panic = "abort"
lto = true
opt-level = "s"

[profile.release]
panic = "abort"
lto = true
opt-level = "s"
"#,
        cargo_config: r#"[build]
target = "avr-atmega2560.json"

[unstable]
build-std = ["core"]

[target.'cfg(target_arch = "avr")']
runner = "ravedude mega2560 -cb 115200"
rustflags = ["-C", "opt-level=s", "-C", "target-cpu=atmega2560"]
"#,
        rust_toolchain: r#"[toolchain]
channel = "nightly"
components = ["rust-src"]
profile = "minimal"
"#,
        memory_x: None,
        build_rs: None,
        linker_ld: None,
        target_json: Some(("avr-atmega2560.json", ATMEGA2560_JSON)),
    }
}

pub fn arduino_leonardo() -> BlinkTemplate {
    BlinkTemplate {
        main_rs: r#"//! Arduino Leonardo Lチカ
//! LED: D13 (PC7) — ATmega32U4
#![no_std]
#![no_main]

use panic_halt as _;
use arduino_hal::prelude::*;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut led = pins.d13.into_output();

    loop {
        led.toggle();
        arduino_hal::delay_ms(500);
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
arduino-hal = { git = "https://github.com/Rahix/avr-hal", features = ["arduino-leonardo"] }
panic-halt = "0.2"

[profile.dev]
panic = "abort"
lto = true
opt-level = "s"

[profile.release]
panic = "abort"
lto = true
opt-level = "s"
"#,
        cargo_config: r#"[build]
target = "avr-atmega32u4.json"

[unstable]
build-std = ["core"]

[target.'cfg(target_arch = "avr")']
runner = "ravedude leonardo -cb 115200"
rustflags = ["-C", "opt-level=s", "-C", "target-cpu=atmega32u4"]
"#,
        rust_toolchain: r#"[toolchain]
channel = "nightly"
components = ["rust-src"]
profile = "minimal"
"#,
        memory_x: None,
        build_rs: None,
        linker_ld: None,
        target_json: Some(("avr-atmega32u4.json", ATMEGA32U4_JSON)),
    }
}

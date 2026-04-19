// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright 2026 rust-embedded-ide contributors

use super::BlinkTemplate;

pub fn esp32() -> BlinkTemplate {
    BlinkTemplate {
        main_rs: r#"//! ESP32 Lチカ
//! LED: GPIO2 (多くのESP32ボードのオンボードLED)
#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    gpio::{Level, Output},
    main,
};

#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let delay = Delay::new();

    // GPIO2 = オンボードLED
    let mut led = Output::new(peripherals.GPIO2, Level::Low);

    loop {
        led.set_high();
        delay.delay_millis(500u32);
        led.set_low();
        delay.delay_millis(500u32);
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
esp-hal = { version = "0.22", features = ["esp32"] }
esp-backtrace = { version = "0.14", features = ["esp32", "panic-handler", "println"] }

[profile.dev]
opt-level = "s"

[profile.release]
codegen-units = 1
debug = 2
debug-assertions = false
incremental = false
lto = "fat"
opt-level = "s"
overflow-checks = false
"#,
        cargo_config: r#"[build]
target = "xtensa-esp32-none-elf"

[target.xtensa-esp32-none-elf]
runner = "espflash flash --monitor"
rustflags = [
    "-C", "link-arg=-nostartfiles",
]

[unstable]
build-std = ["core"]
"#,
        rust_toolchain: r#"[toolchain]
channel = "esp"
"#,
        memory_x: None,
        build_rs: None,
        linker_ld: None,
        target_json: None,
    }
}

pub fn esp32s2() -> BlinkTemplate {
    BlinkTemplate {
        main_rs: r#"//! ESP32-S2 Lチカ
//! LED: GPIO2
#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    gpio::{Level, Output},
    main,
};

#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let delay = Delay::new();
    let mut led = Output::new(peripherals.GPIO2, Level::Low);

    loop {
        led.set_high();
        delay.delay_millis(500u32);
        led.set_low();
        delay.delay_millis(500u32);
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
esp-hal = { version = "0.22", features = ["esp32s2"] }
esp-backtrace = { version = "0.14", features = ["esp32s2", "panic-handler", "println"] }

[profile.dev]
opt-level = "s"

[profile.release]
codegen-units = 1
debug = 2
debug-assertions = false
incremental = false
lto = "fat"
opt-level = "s"
overflow-checks = false
"#,
        cargo_config: r#"[build]
target = "xtensa-esp32s2-none-elf"

[target.xtensa-esp32s2-none-elf]
runner = "espflash flash --monitor"
rustflags = ["-C", "link-arg=-nostartfiles"]

[unstable]
build-std = ["core"]
"#,
        rust_toolchain: r#"[toolchain]
channel = "esp"
"#,
        memory_x: None,
        build_rs: None,
        linker_ld: None,
        target_json: None,
    }
}

pub fn esp32s3() -> BlinkTemplate {
    // esp32s3 は target = "xtensa-esp32s3-none-elf"
    // それ以外は esp32s2 とほぼ同一
    BlinkTemplate {
        main_rs: r#"//! ESP32-S3 Lチカ
//! LED: GPIO2
#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    gpio::{Level, Output},
    main,
};

#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let delay = Delay::new();
    let mut led = Output::new(peripherals.GPIO2, Level::Low);

    loop {
        led.set_high();
        delay.delay_millis(500u32);
        led.set_low();
        delay.delay_millis(500u32);
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
esp-hal = { version = "0.22", features = ["esp32s3"] }
esp-backtrace = { version = "0.14", features = ["esp32s3", "panic-handler", "println"] }

[profile.dev]
opt-level = "s"

[profile.release]
codegen-units = 1
debug = 2
debug-assertions = false
incremental = false
lto = "fat"
opt-level = "s"
overflow-checks = false
"#,
        cargo_config: r#"[build]
target = "xtensa-esp32s3-none-elf"

[target.xtensa-esp32s3-none-elf]
runner = "espflash flash --monitor"
rustflags = ["-C", "link-arg=-nostartfiles"]

[unstable]
build-std = ["core"]
"#,
        rust_toolchain: r#"[toolchain]
channel = "esp"
"#,
        memory_x: None,
        build_rs: None,
        linker_ld: None,
        target_json: None,
    }
}

pub fn esp32c3() -> BlinkTemplate {
    BlinkTemplate {
        main_rs: r#"//! ESP32-C3 Lチカ
//! LED: GPIO8 (RISC-V RV32IMC)
#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    gpio::{Level, Output},
    main,
};

#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let delay = Delay::new();
    // ESP32-C3 の多くのボードでは GPIO8 がLED
    let mut led = Output::new(peripherals.GPIO8, Level::Low);

    loop {
        led.set_high();
        delay.delay_millis(500u32);
        led.set_low();
        delay.delay_millis(500u32);
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
esp-hal = { version = "0.22", features = ["esp32c3"] }
esp-backtrace = { version = "0.14", features = ["esp32c3", "panic-handler", "println"] }

[profile.dev]
opt-level = "s"

[profile.release]
codegen-units = 1
debug = 2
debug-assertions = false
incremental = false
lto = "fat"
opt-level = "s"
overflow-checks = false
"#,
        cargo_config: r#"[build]
target = "riscv32imc-unknown-none-elf"

[target.riscv32imc-unknown-none-elf]
runner = "espflash flash --monitor"
rustflags = ["-C", "link-arg=-Tlinkall.x"]
"#,
        rust_toolchain: r#"[toolchain]
channel = "stable"
targets = ["riscv32imc-unknown-none-elf"]
"#,
        memory_x: None,
        build_rs: None,
        linker_ld: None,
        target_json: None,
    }
}

pub fn esp32c6() -> BlinkTemplate {
    BlinkTemplate {
        main_rs: r#"//! ESP32-C6 Lチカ
//! LED: GPIO8 (RISC-V RV32IMAC)
#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{delay::Delay, gpio::{Level, Output}, main};

#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let delay = Delay::new();
    let mut led = Output::new(peripherals.GPIO8, Level::Low);
    loop {
        led.set_high();
        delay.delay_millis(500u32);
        led.set_low();
        delay.delay_millis(500u32);
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
esp-hal = { version = "0.22", features = ["esp32c6"] }
esp-backtrace = { version = "0.14", features = ["esp32c6", "panic-handler", "println"] }

[profile.release]
codegen-units = 1
lto = "fat"
opt-level = "s"
"#,
        cargo_config: r#"[build]
target = "riscv32imac-unknown-none-elf"

[target.riscv32imac-unknown-none-elf]
runner = "espflash flash --monitor"
rustflags = ["-C", "link-arg=-Tlinkall.x"]
"#,
        rust_toolchain: r#"[toolchain]
channel = "stable"
targets = ["riscv32imac-unknown-none-elf"]
"#,
        memory_x: None,
        build_rs: None,
        linker_ld: None,
        target_json: None,
    }
}

pub fn esp32h2() -> BlinkTemplate {
    // H2 も riscv32imac、C6 とほぼ同一、feature だけ違う
    BlinkTemplate {
        main_rs: r#"//! ESP32-H2 Lチカ
//! LED: GPIO8 (RISC-V RV32IMAC)
#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{delay::Delay, gpio::{Level, Output}, main};

#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let delay = Delay::new();
    let mut led = Output::new(peripherals.GPIO8, Level::Low);
    loop {
        led.set_high();
        delay.delay_millis(500u32);
        led.set_low();
        delay.delay_millis(500u32);
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
esp-hal = { version = "0.22", features = ["esp32h2"] }
esp-backtrace = { version = "0.14", features = ["esp32h2", "panic-handler", "println"] }

[profile.release]
codegen-units = 1
lto = "fat"
opt-level = "s"
"#,
        cargo_config: r#"[build]
target = "riscv32imac-unknown-none-elf"

[target.riscv32imac-unknown-none-elf]
runner = "espflash flash --monitor"
rustflags = ["-C", "link-arg=-Tlinkall.x"]
"#,
        rust_toolchain: r#"[toolchain]
channel = "stable"
targets = ["riscv32imac-unknown-none-elf"]
"#,
        memory_x: None,
        build_rs: None,
        linker_ld: None,
        target_json: None,
    }
}

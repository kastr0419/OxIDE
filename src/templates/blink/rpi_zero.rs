// SPDX-License-Identifier: MIT OR Apache-2.0
use super::BlinkTemplate;

pub fn rpi_zero() -> BlinkTemplate {
    BlinkTemplate {
        main_rs: MAIN_RS,
        cargo_toml: CARGO_TOML,
        cargo_config: CARGO_CONFIG,
        rust_toolchain: RUST_TOOLCHAIN,
        memory_x: None,
        build_rs: None,
        linker_ld: Some(LINKER_LD),
        target_json: Some(("armv6-rpi-zero.json", TARGET_JSON)),
    }
}

const MAIN_RS: &str = r#"// Raspberry Pi Zero bare-metal blink (ACT LED = GPIO47)
// ビルド: cargo build --release --target ./armv6-rpi-zero.json
// 変換:   arm-none-eabi-objcopy -O binary target/.../release/blink kernel.img
// 書込:   kernel.img を SD カードの BOOT パーティションにコピー
#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::ptr::{read_volatile, write_volatile};

const GPIO_BASE: usize    = 0x2020_0000;
const GPFSEL4_OFFSET: usize = 0x10;
const GPSET1_OFFSET: usize  = 0x20;
const GPCLR1_OFFSET: usize  = 0x2C;
const GPIO47_BIT: u32 = 47 - 32; // bit 15 in GPSET1/GPCLR1

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    unsafe {
        let gpfsel4 = (GPIO_BASE + GPFSEL4_OFFSET) as *mut u32;
        let mut v = read_volatile(gpfsel4);
        // Each GPIO uses 3 bits in the GPFSEL registers; for GPIO47 (index 15) shift by 15*3 = 45
        v &= !(0b111 << (GPIO47_BIT * 3)); // clear bits for GPIO47
        v |=  0b001 << (GPIO47_BIT * 3);   // set as output
        write_volatile(gpfsel4, v);

        let gpset1 = (GPIO_BASE + GPSET1_OFFSET) as *mut u32;
        let gpclr1 = (GPIO_BASE + GPCLR1_OFFSET) as *mut u32;
        loop {
            write_volatile(gpset1, 1u32 << GPIO47_BIT); // LED ON
            delay(500_000);
            write_volatile(gpclr1, 1u32 << GPIO47_BIT); // LED OFF
            delay(500_000);
        }
    }
}

#[inline(never)]
fn delay(n: u32) {
    for _ in 0..n {
        unsafe { core::arch::asm!("nop") };
    }
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! { loop {} }
"#;

const CARGO_TOML: &str = r#"[package]
name = "blink"
version = "0.1.0"
edition = "2021"

[profile.release]
codegen-units = 1
lto = true
debug = false
opt-level = "z"
panic = "abort"
"#;

const CARGO_CONFIG: &str = r#"[build]
target = "armv6-rpi-zero.json"

[target.armv6-rpi-zero]
rustflags = ["-C", "link-arg=-Tlinker.ld"]
"#;

const RUST_TOOLCHAIN: &str = r#"[toolchain]
channel = "stable"
"#;

const TARGET_JSON: &str = r#"{
  "llvm-target": "armv6j-none-eabi",
  "data-layout": "e-m:e-p:32:32-i64:64-v128:128:128-a:0:32-n32-S64",
  "arch": "arm",
  "target-endian": "little",
  "target-pointer-width": "32",
  "target-c-int-width": "32",
  "os": "none",
  "env": "eabi",
  "vendor": "unknown",
  "cpu": "arm1176jzf-s",
  "features": "+v6,+vfp2,-thumb2,-neon",
  "max-atomic-width": 0,
  "linker-flavor": "gcc",
  "linker": "arm-none-eabi-gcc",
  "pre-link-args": {
    "gcc": [
      "-mcpu=arm1176jzf-s",
      "-march=armv6",
      "-mfpu=vfp",
      "-mfloat-abi=hard",
      "-Wl,--gc-sections"
    ]
  },
  "post-link-args": {
    "gcc": ["-nostartfiles"]
  },
  "panic-strategy": "abort",
  "disable-redzone": true,
  "executables": true,
  "relocation-model": "static",
  "dynamic-linking": false,
  "emit-debug-gdb-scripts": false
}"#;

const LINKER_LD: &str = r#"/* linker.ld - Raspberry Pi Zero (BCM2835) */
ENTRY(rust_main)

MEMORY
{
  RAM (rwx) : ORIGIN = 0x00008000, LENGTH = 0x07F8000
}

_estack = ORIGIN(RAM) + LENGTH(RAM);

SECTIONS
{
  . = ORIGIN(RAM);

  .text : {
    *(.text .text.*)
    *(.rodata .rodata.*)
    . = ALIGN(4);
  } > RAM

  .data : {
    __data_start = .;
    *(.data .data.*)
    . = ALIGN(4);
    __data_end = .;
  } > RAM

  .bss (NOLOAD) : {
    __bss_start = .;
    *(.bss .bss.*)
    *(COMMON)
    . = ALIGN(4);
    __bss_end = .;
  } > RAM

  /DISCARD/ : { *(.eh_frame*) }
}
"#;

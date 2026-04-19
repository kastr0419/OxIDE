// Raspberry Pi Zero bare-metal blink (ACT LED = GPIO47)
#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::ptr::{read_volatile, write_volatile};

const GPIO_BASE: usize    = 0x2020_0000;
const GPFSEL4_OFFSET: usize = 0x10;
const GPSET1_OFFSET: usize  = 0x20;
const GPCLR1_OFFSET: usize  = 0x2C;
const GPIO47_BIT: u32 = 47 - 32;

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    unsafe {
        let gpfsel4 = (GPIO_BASE + GPFSEL4_OFFSET) as *mut u32;
        let mut v = read_volatile(gpfsel4);
        v &= !(0b111 << (GPIO47_BIT * 3));
        v |=  0b001 << (GPIO47_BIT * 3);
        write_volatile(gpfsel4, v);

        let gpset1 = (GPIO_BASE + GPSET1_OFFSET) as *mut u32;
        let gpclr1 = (GPIO_BASE + GPCLR1_OFFSET) as *mut u32;
        loop {
            write_volatile(gpset1, 1u32 << GPIO47_BIT);
            delay(500_000);
            write_volatile(gpclr1, 1u32 << GPIO47_BIT);
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

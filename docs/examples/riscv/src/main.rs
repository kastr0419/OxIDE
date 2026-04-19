//! GD32VF103 Lチカ - LED: PB0
#![no_std]
#![no_main]

use gd32vf103xx_hal::{pac, prelude::*};
use embedded_hal::digital::v2::OutputPin;
use riscv_rt::entry;
use panic_halt as _;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let mut rcu = dp.RCU.configure().freeze();
    let mut gpiob = dp.GPIOB.split(&mut rcu);
    let mut led = gpiob.pb0.into_push_pull_output();
    loop {
        led.set_low().unwrap();
        unsafe { riscv::asm::delay(8_000_000) };
        led.set_high().unwrap();
        unsafe { riscv::asm::delay(8_000_000) };
    }
}

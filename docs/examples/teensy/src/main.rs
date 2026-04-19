//! Teensy 4.0 Lチカ - LED: pin 13
#![no_std]
#![no_main]

use teensy4_bsp as bsp;
use bsp::board;
use panic_halt as _;

#[bsp::rt::entry]
fn main() -> ! {
    let instances = board::instances();
    let board::Resources { mut gpio2, pins, .. } = board::t40(instances);
    let mut led = bsp::board::led(&mut gpio2, pins.p13);

    loop {
        led.set();
        cortex_m::asm::delay(600_000_000 / 2);
        led.clear();
        cortex_m::asm::delay(600_000_000 / 2);
    }
}

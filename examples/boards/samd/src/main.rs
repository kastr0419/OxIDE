//! Metro M0 / SAMD21 Lチカ - LED: D13
#![no_std]
#![no_main]

use metro_m0 as bsp;
use bsp::entry;
use bsp::hal;
use bsp::pac;
use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::prelude::*;
use panic_halt as _;

#[entry]
fn main() -> ! {
    let mut peripherals = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK, &mut peripherals.PM, &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );
    let pins = bsp::Pins::new(peripherals.PORT);
    let mut led = pins.d13.into_push_pull_output();
    let mut delay = Delay::new(core.SYST, &mut clocks);
    loop {
        led.set_high().unwrap();
        delay.delay_ms(500u32);
        led.set_low().unwrap();
        delay.delay_ms(500u32);
    }
}

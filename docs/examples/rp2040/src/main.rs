//! Raspberry Pi Pico Lチカ - GPIO25
#![no_std]
#![no_main]

use bsp::entry;
use bsp::hal::{clocks::init_clocks_and_plls, clocks::ClockSource, pac, sio::Sio, watchdog::Watchdog};
use embedded_hal::digital::OutputPin;
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
        core.SYST, clocks.system_clock.get_freq().to_Hz()
    );

    let pins = bsp::Pins::new(
        pac.IO_BANK0, pac.PADS_BANK0, sio.gpio_bank0, &mut pac.RESETS,
    );

    let mut led = pins.led.into_push_pull_output();

    loop {
        led.set_high().unwrap();
        delay.delay_ms(500);
        led.set_low().unwrap();
        delay.delay_ms(500);
    }
}

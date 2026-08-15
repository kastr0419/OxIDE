//! ESP32 Lチカ - GPIO2
#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{delay::Delay, gpio::{Level, Output, OutputConfig}, main};

#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let delay = Delay::new();
    let mut led = Output::new(peripherals.GPIO2, Level::Low, OutputConfig::default());
    loop {
        led.set_high();
        delay.delay_millis(500u32);
        led.set_low();
        delay.delay_millis(500u32);
    }
}

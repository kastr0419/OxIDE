//! STM32F401 Lチカ - LED: PC13
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;
use stm32f4xx_hal::{pac, prelude::*};

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(84.MHz()).freeze();
    let gpioc = dp.GPIOC.split();
    let mut led = gpioc.pc13.into_push_pull_output();
    let mut delay = dp.TIM2.delay_ms(&clocks);
    loop {
        led.set_low();
        delay.delay_ms(500u32);
        led.set_high();
        delay.delay_ms(500u32);
    }
}

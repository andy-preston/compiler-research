#![no_main]
#![no_std]

use cortex_m::peripheral::Peripherals;
use cortex_m_rt::entry;
use panic_semihosting as _;
use stm32f401_black_pill::{
    hal::{
        delay::Delay,
        prelude::*
    },
    pac,
    Led
};

#[entry]
fn main() -> ! {
    let device = pac::Peripherals::take().unwrap();
    let cortex_peripherals = Peripherals::take().unwrap();

    // (Re-)configure PC13 (User Led) as output
    let gpioc = device.GPIOC.split();
    let mut led = Led::new(gpioc.pc13);
    led.set(false);

    // Constrain clock registers
    let rcc = device.RCC.constrain();

    let clocks = rcc.cfgr.sysclk(84.mhz()).freeze();

    // Get delay provider
    let mut delay = Delay::new(
        cortex_peripherals.SYST,
        clocks
    );

    loop {
        delay.delay_ms(500_u16);
        led.toggle();
    }
}

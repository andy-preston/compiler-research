#![no_main]
#![no_std]

use cortex_m::peripheral::Peripherals;
use cortex_m_rt::entry;
use panic_rtt_target as _;
use rtt_target;
use stm32f401_black_pill::{
    hal::{
        delay::Delay,
        prelude::*
    },
    pac,
    Led,
};

#[entry]
fn main() -> ! {
    rtt_target::rtt_init_default!();

    let p = pac::Peripherals::take().unwrap();
    let cortex_peripherals = Peripherals::take().unwrap();

    // (Re-)configure PC13 (User Led) as output
    let gpioc = p.GPIOC.split();
    let mut led = Led::new(gpioc.pc13);
    led.set(false);

    // Constrain clock registers
    let rcc = p.RCC.constrain();

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

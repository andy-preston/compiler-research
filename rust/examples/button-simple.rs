#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_semihosting as _;
use stm32f401_black_pill::{
    hal::{
        prelude::*
    },
    pac,
    Button,
    Led
};

#[entry]
fn main() -> ! {
    let device = pac::Peripherals::take().unwrap();

    let gpioa = device.GPIOA.split();
    let gpioc = device.GPIOC.split();

    let mut led = Led::new(gpioc.pc13);
    let mut button = Button::new(gpioa.pa0);

    loop {
        led.set(button.is_high());
    }
}

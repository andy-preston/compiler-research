#![no_main]
#![no_std]

use core::cell::RefCell;
use core::sync::atomic::{AtomicBool, Ordering};
use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;
use panic_rtt_target as _;
use rtt_target;

use stm32f401_black_pill::{
    hal::{gpio::Edge, interrupt, prelude::*},
    pac,
    Button,
    Led
};

// Used to signal to the main loop that it should toggle the led
static SIGNAL: AtomicBool = AtomicBool::new(false);

static BUTTON: Mutex<RefCell<Option<Button>>> = Mutex::new(
    RefCell::new(None)
);

#[entry]
fn main() -> ! {
    rtt_target::rtt_init_default!();

    // The Stm32 peripherals
    let mut device = pac::Peripherals::take().unwrap();

    let gpioa = device.GPIOA.split();
    let gpioc = device.GPIOC.split();

    let mut led = Led::new(gpioc.pc13);

    let mut button = Button::new(gpioa.pa0);
    button.enable_interrupt(
        Edge::FALLING,
        &mut device.SYSCFG,
        &mut device.EXTI
    );

    cortex_m::interrupt::free(|cs| {
        BUTTON.borrow(cs).replace(Some(button));
    });

    // Enable the external interrupt
    unsafe {
        cortex_m::peripheral::NVIC::unmask(
            pac::Interrupt::EXTI0
        );
    }

    loop {
        let state_change = SIGNAL.load(Ordering::SeqCst);
        if state_change {
            led.toggle();
            SIGNAL.store(false, Ordering::SeqCst);
        }
    }
}

#[interrupt]
fn EXTI0() {
    // Clear the interrupt
    cortex_m::interrupt::free(|cs| {
        let mut button = BUTTON.borrow(cs).borrow_mut();
        button.as_mut().unwrap().clear_interrupt_pending_bit();
    });

    SIGNAL.store(true, Ordering::SeqCst);
}

#![no_main]
#![no_std]

use core::cell::RefCell;
use core::sync::atomic::{AtomicUsize, Ordering};

use cortex_m::interrupt::Mutex;
use cortex_m::peripheral::Peripherals;
use cortex_m_rt::entry;
use panic_rtt_target as _;

use stm32f401_black_pill::{
    hal::{
        delay::Delay,
        gpio::Edge,
        interrupt,
        prelude::*,
        spi::{self, Spi},
    },
    pac, Button,
};

use embedded_hal::digital::v1_compat::OldOutputPin;

use segment_display::SegmentDisplay;

static SIGNAL: AtomicUsize = AtomicUsize::new(0);
static BUTTON: Mutex<RefCell<Option<Button>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    rtt_target::rtt_init_print!();

    // The Stm32 peripherals
    let mut dp = pac::Peripherals::take().unwrap();
    // The Cortex-m peripherals
    let p = Peripherals::take().unwrap();

    // Enable the clock for the SYSCFG
    dp.RCC.apb2enr.modify(|_, w| w.syscfgen().enabled());
    // Constrain clock registers
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(84.mhz()).freeze();

    let gpioa = dp.GPIOA.split();
    let gpiob = dp.GPIOB.split();

    // Setup button
    let mut button = Button::new(gpioa.pa0);
    button.enable_interrupt(Edge::FALLING, &mut dp.SYSCFG, &mut dp.EXTI);

    let mut delay = Delay::new(p.SYST, clocks);

    cortex_m::interrupt::free(|cs| {
        BUTTON.borrow(cs).replace(Some(button));
    });

    // Enable the external interrupt
    unsafe {
        cortex_m::peripheral::NVIC::unmask(pac::Interrupt::EXTI15_10);
    }

    let sck = gpiob.pb3.into_alternate_af5();
    let miso = spi::NoMiso;
    let mosi = gpiob.pb5.into_alternate_af5();

    // rclk
    let latch = gpiob.pb4.into_push_pull_output();

    let mode = spi::Mode {
        polarity: spi::Polarity::IdleHigh,
        phase: spi::Phase::CaptureOnFirstTransition,
    };

    let spi = Spi::spi1(dp.SPI1, (sck, miso, mosi), mode, 10_000_000.hz(), clocks);

    let mut segment_display = SegmentDisplay::new(spi, OldOutputPin::from(latch));

    loop {
        let v = SIGNAL.load(Ordering::SeqCst);
        segment_display.write_number(v);
        segment_display.refresh().unwrap();

        delay.delay_us(200_u16);
    }
}

#[interrupt]
fn EXTI15_10() {
    // Clear the interrupt
    cortex_m::interrupt::free(|cs| {
        let mut button = BUTTON.borrow(cs).borrow_mut();
        button.as_mut().unwrap().clear_interrupt_pending_bit();
    });
    // Update display number
    SIGNAL.fetch_add(1, Ordering::SeqCst);
}

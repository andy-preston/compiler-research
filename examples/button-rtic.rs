#![no_main]
#![no_std]

use panic_semihosting as _;
use cortex_m_semihosting::hprintln;
use stm32f401_black_pill::{
    hal::{
        gpio::Edge,
        prelude::*
    },
    Button,
    Led
};
use rtic::app;

#[app(device = stm32f401_black_pill::pac, peripherals = true)]
const APP: () = {
    struct Resources {
        button: Button,
        led: Led
    }

    #[init]
    fn init(ctx: init::Context) -> init::LateResources {
        // Device specific peripherals
        let mut device = ctx.device;

        // Enable the clock for the SYSCFG
        device.RCC.apb2enr.modify(|_, w| w.syscfgen().enabled());

        // Setup the system clock
        let rcc = device.RCC.constrain();
        let _clocks = rcc.cfgr.sysclk(84.mhz()).freeze();

        let gpioa = device.GPIOA.split();
        let gpioc = device.GPIOC.split();

        // Setup Button and enable external interrupt
        let mut button = Button::new(gpioa.pa0);
        button.enable_interrupt(
            Edge::RISING,
            &mut device.SYSCFG,
            &mut device.EXTI
        );

        // Setup the led
        let led = Led::new(gpioc.pc13);

        hprintln!("init done").unwrap();

        init::LateResources { led, button }
    }

    #[idle]
    fn idle(_ctx: idle::Context) -> ! {
        hprintln!("idle").unwrap();

        // The idle loop
        loop {}
    }

    #[task(binds = EXTI0, resources = [led, button])]
    fn on_button_press(ctx: on_button_press::Context) {
        let on_button_press::Resources { led, button } = ctx.resources;

        // Clear the interrupt
        button.clear_interrupt_pending_bit();

        // Toggle the led
        led.toggle();
    }
};

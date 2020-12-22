use stm32f4xx_hal::gpio::{gpioa::PA0, Edge, ExtiPin, Input, PullUp};

use stm32f4xx_hal::stm32::{EXTI, SYSCFG};

pub struct Button {
    pin: PA0<Input<PullUp>>,
}

impl Button {
    pub fn new<M>(
        pa0: PA0<M>
    ) -> Self {
        let pin = pa0.into_pull_up_input();
        Self { pin }
    }

    pub fn enable_interrupt(
        &mut self,
        edge: Edge,
        syscfg: &mut SYSCFG,
        exti: &mut EXTI
    ) {
        // Enable external interrupt on PA0
        self.pin.make_interrupt_source(syscfg);
        self.pin.enable_interrupt(exti);
        self.pin.trigger_on_edge(exti, edge);
    }

    pub fn clear_interrupt_pending_bit(
        &mut self
    ) {
        self.pin.clear_interrupt_pending_bit();
    }
}

use stm32f4xx_hal::gpio::{gpioc::PC13, Output, PushPull};

use embedded_hal::digital::v2::{OutputPin, ToggleableOutputPin};

pub struct Led {
    pin: PC13<Output<PushPull>>,
}

impl Led {
    pub fn new<M>(
        pc13: PC13<M>
    ) -> Self {
        let pin = pc13.into_push_pull_output();
        Self { pin }
    }

    pub fn set(
        &mut self,
        enable: bool
    ) {
        if enable {
            self.pin.set_high().ok();
        } else {
            self.pin.set_low().ok();
        }
    }

    pub fn toggle(
        &mut self
    ) {
        self.pin.toggle().ok();
    }
}

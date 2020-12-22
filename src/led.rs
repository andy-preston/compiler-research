use stm32f4xx_hal::gpio::{gpioc::PC13, Output, PushPull};

use embedded_hal::digital::v2::{OutputPin, ToggleableOutputPin};

pub struct Led {
    pc13: PC13<Output<PushPull>>,
}

impl Led {
    pub fn new<M>(
        pin: PC13<M>
    ) -> Self {
        let pc13 = pin.into_push_pull_output();
        Self { pc13 }
    }

    pub fn set(
        &mut self,
        enable: bool
    ) {
        if enable {
            self.pc13.set_high().ok();
        } else {
            self.pc13.set_low().ok();
        }
    }

    pub fn toggle(
        &mut self
    ) {
        self.pc13.toggle().ok();
    }
}

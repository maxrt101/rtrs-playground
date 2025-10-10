#![allow(dead_code)]

use embedded_hal::digital::v2::OutputPin;
use core::fmt::Write;
use rtrs::println;

pub(crate) struct MockPin {
    state: bool,
    last_state_change_time: u32,
}

impl MockPin {
    pub(crate) const fn new() -> Self {
        Self { state: false, last_state_change_time: 0 }
    }

    fn state_str(&self) -> &'static str {
        if self.state {
            "HIGH"
        } else {
            "LOW"
        }
    }
}

impl OutputPin for MockPin {
    type Error = void::Void;

    fn set_low(&mut self) -> Result<(), Self::Error> {
        println!("MockPin(0): Was {} for {} ticks", self.state_str(), rtrs::time::global_tick() - self.last_state_change_time);
        self.state = false;
        self.last_state_change_time = rtrs::time::global_tick();
        Ok(())
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        println!("MockPin(1): Was {} for {} ticks", self.state_str(), rtrs::time::global_tick() - self.last_state_change_time);
        self.state = true;
        self.last_state_change_time = rtrs::time::global_tick();
        Ok(())
    }
}

impl rtrs::object::Object for MockPin {}

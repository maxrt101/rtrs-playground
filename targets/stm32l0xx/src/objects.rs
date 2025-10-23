use crate::hal::gpio::gpioa::{PA5, PA14, PA15};
use crate::hal::gpio::{Output, Input, PushPull, PullDown};
use crate::hal::pac::USART1;
use crate::hal::serial::Serial;

use rtrs::{object_insert, output_pin_wrapper, input_pin_wrapper};
use rtrs::time::{TimeProvider, TIME_OBJECT_NAME};
use rtrs::log::console::CONSOLE_OBJECT_NAME;
use rtrs_drivers::radio::sx1278::SX1278RadioDriver;

output_pin_wrapper!(LedPin,    PA5<Output<PushPull>>);
output_pin_wrapper!(BuzzerPin, PA15<Output<PushPull>>);
input_pin_wrapper!(ButtonPin,  PA14<Input<PullDown>>);

pub(crate) fn init_serial(log_serial: Serial<USART1>) {
    object_insert!(CONSOLE_OBJECT_NAME, rtrs::tty::Tty::new(super::tty::TtyUSART1Backend::new(log_serial)));
}

pub(crate) fn init_led(green_led: PA5<Output<PushPull>>) {
    object_insert!(crate::GREEN_LED_NAME, rtrs::gpio::Output::new(LedPin::new(green_led)));
}

pub(crate) fn init_btn(btn: PA14<Input<PullDown>>) {
    object_insert!(crate::BTN_PIN_NAME, rtrs::gpio::Input::new(ButtonPin::new(btn)));
}

pub(crate) fn init_buzz(pin: PA15<Output<PushPull>>) {
    object_insert!(crate::BUZZER_PIN_NAME, rtrs::gpio::Output::new(BuzzerPin::new(pin)));
}

pub(crate) fn init_time() {
    object_insert!(TIME_OBJECT_NAME, TimeProvider::new());
}

pub(crate) fn init_radio(bus: super::spi::Spi1Bus) {
    let radio = SX1278RadioDriver::create_radio(bus);
    object_insert!("radio", radio);
}

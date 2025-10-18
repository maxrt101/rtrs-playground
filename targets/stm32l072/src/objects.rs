use core::fmt::Write;
use embedded_hal::serial::Read;

use stm32l0xx_hal::gpio::gpioa::{PA5, PA14, PA15};
use stm32l0xx_hal::gpio::{Output, Input, PushPull, PullDown};
use stm32l0xx_hal::pac::USART1;
use stm32l0xx_hal::serial::Serial;

use rtrs::object_insert;
use rtrs::sync::{Mutex, RaceAction};
use rtrs::time::{TimeProvider, TIME_OBJECT_NAME};
use rtrs::log::console::CONSOLE_OBJECT_NAME;

use alloc::boxed::Box;

static SERIAL: Mutex<Serial<USART1>> = Mutex::uninit(RaceAction::Crash);

fn init_serial(log_serial: Serial<USART1>) {
    SERIAL.replace(log_serial);

    object_insert!(CONSOLE_OBJECT_NAME, rtrs::tty::Tty::new(
        |ch: u8| {
            let mut serial = SERIAL.lock_mut();
            serial.write_char(ch as char).unwrap();
        },
        || -> Option<u8> {
            let mut serial = SERIAL.lock_mut();
            serial.read().ok()
        }
    ));
}

fn init_led(green_led: PA5<Output<PushPull>>) {
    object_insert!(crate::GREEN_LED_NAME, rtrs::gpio::Output::new(Box::new(green_led)));
}

fn init_btn(btn: PA14<Input<PullDown>>) {
    object_insert!(crate::BTN_NAME, rtrs::gpio::Input::new(Box::new(btn)));
}

fn init_buzz(pin: PA15<Output<PushPull>>) {
    object_insert!(crate::BUZZER_PIN_NAME, rtrs::gpio::Output::new(Box::new(pin)));
}

fn init_time() {
    object_insert!(TIME_OBJECT_NAME, TimeProvider::new());
    // object_insert!(crate::US_TIMER_NAME, TimeProvider::new());
}

pub(crate) fn init_objects(
    log_serial: Serial<USART1>,
    green_led: PA5<Output<PushPull>>,
    btn: PA14<Input<PullDown>>,
    buzzer_pin: PA15<Output<PushPull>>
) {
    init_serial(log_serial);
    init_led(green_led);
    init_btn(btn);
    init_buzz(buzzer_pin);
    init_time();
}

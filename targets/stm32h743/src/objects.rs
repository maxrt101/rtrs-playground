use core::fmt::Write;
use embedded_hal::serial::Read;

use stm32h7xx_hal::gpio::gpioa::PA15;
use stm32h7xx_hal::gpio::{Output, PushPull};
use stm32h7xx_hal::pac::USART3;
use stm32h7xx_hal::serial::Serial;

use rtrs::object_insert;
use rtrs::sync::{Mutex, RaceAction};
use rtrs::time::{TimeProvider, TIME_OBJECT_NAME};
use rtrs::log::console::CONSOLE_OBJECT_NAME;

static SERIAL: Mutex<Serial<USART3>> = Mutex::uninit(RaceAction::Crash);

fn init_serial(log_serial: Serial<USART3>) {
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

fn init_time() {
    object_insert!(TIME_OBJECT_NAME, TimeProvider::new());
}

pub(crate) fn init_objects(log_serial: Serial<USART3>) {
    init_serial(log_serial);
    init_time();
}

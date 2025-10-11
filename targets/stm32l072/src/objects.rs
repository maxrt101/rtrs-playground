use core::fmt::Write;
use embedded_hal::serial::Read;

use stm32l0xx_hal::gpio::gpioa::PA15;
use stm32l0xx_hal::gpio::{Output, PushPull};
use stm32l0xx_hal::pac::USART1;
use stm32l0xx_hal::serial::Serial;

use rtrs::object_insert;
use rtrs::util::{LazyTakeOnce};
use rtrs::sync::{Mutex, RaceAction};
use rtrs::time::{TimeProvider, TIME_OBJECT_NAME};
use rtrs::log::console::CONSOLE_OBJECT_NAME;

static SERIAL: Mutex<Serial<USART1>> = Mutex::uninit(RaceAction::Crash);
static LED_GREEN_GPIO: LazyTakeOnce<PA15<Output<PushPull>>> = LazyTakeOnce::uninit();

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

fn init_led(green_led: PA15<Output<PushPull>>) {
    LED_GREEN_GPIO.init(green_led);

    object_insert!(crate::GREEN_LED_NAME, rtrs::Led::new(LED_GREEN_GPIO.take_mut()));
}

fn init_time() {
    object_insert!(TIME_OBJECT_NAME, TimeProvider::new());
}

pub(crate) fn init_objects(log_serial: Serial<USART1>, green_led: PA15<Output<PushPull>>) {
    init_serial(log_serial);
    init_led(green_led);
    init_time();
}

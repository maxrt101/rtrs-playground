#![no_std]
#![no_main]

mod exc;
mod util;
mod time;
mod objects;

use cortex_m_rt::entry;

use stm32l0xx_hal as hal;
use hal::prelude::*;

extern crate alloc;
use alloc::boxed::Box;

pub const US_TIMER_NAME: &str = "us_time";
pub const GREEN_LED_NAME: &str = "led_green";
pub const BTN_NAME: &str = "btn";
pub const BUZZER_PIN_NAME: &str = "buzzer";

#[unsafe(no_mangle)]
fn rtrs_critical_section_acquire() {
    cortex_m::interrupt::disable();
}

#[unsafe(no_mangle)]
fn rtrs_critical_section_release() {
    unsafe { cortex_m::interrupt::enable() };
}

#[entry]
unsafe fn main() -> ! {
    let peripherals = hal::pac::Peripherals::take().unwrap();
    let mut rcc = peripherals.RCC.freeze(hal::rcc::Config::hsi16());
    // let pwr = hal::pwr::PWR::new(peripherals.PWR, &mut rcc);

    let mut core_peripherals = cortex_m::Peripherals::take().unwrap();

    time::setup_systick(&mut core_peripherals.SYST, rcc.clocks.sys_clk().0, 1_000);

    time::setup_tim2();

    let gpioa = peripherals.GPIOA.split(&mut rcc);

    let usart_tx_pin = gpioa.pa9;
    let usart_rx_pin = gpioa.pa10;

    let log_serial = peripherals.USART1.usart(
        usart_tx_pin,
        usart_rx_pin,
        hal::serial::Config::default().baudrate(115_200.Bd()),
        &mut rcc
    ).unwrap();

    let green_led_pin = gpioa.pa5.into_push_pull_output();
    let btn_pin = gpioa.pa14.into_pull_down_input();

    let buzzer_pin = gpioa.pa15.into_push_pull_output();

    objects::init_objects(log_serial, green_led_pin, btn_pin, buzzer_pin);

    {
        let mut r = time::SYSCLK.lock_mut();
        *r = rcc.clocks.sys_clk().0;
    }

    app::board::BoardInterface::register_callback(app::board::CallbackType::TriggerCrash(|| {
        unsafe {
            core::arch::asm!("udf #0");
        }
    }));

    app::board::BoardInterface::register_callback(app::board::CallbackType::MicrosecondDelay(|us| {
        let r = time::SYSCLK.lock();
        time::delay_us(us, *r, 3);
    }));

    app::board::BoardInterface::register_callback(app::board::CallbackType::MicrosecondTickProvider(|provider| {
        *provider = Box::new(time::MicrosecondTickProvider::new());
    }));

    app::main();
}


#![no_std]
#![no_main]

mod exc;
mod util;
mod objects;
mod pins;

use cortex_m::peripheral::SYST;
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::entry;

use stm32l0xx_hal as hal;
use hal::prelude::*;

pub const GREEN_LED_NAME: &str = "led_green";

#[unsafe(no_mangle)]
fn rtrs_critical_section_acquire() {
    cortex_m::interrupt::disable();
}

#[unsafe(no_mangle)]
fn rtrs_critical_section_release() {
    unsafe { cortex_m::interrupt::enable() };
}

fn setup_systick(syst: &mut SYST, core_freq: u32, hz: u32) {
    syst.set_clock_source(SystClkSource::Core);
    syst.set_reload(core_freq / hz - 1);
    syst.clear_current();
    syst.enable_counter();
    syst.enable_interrupt();
}

#[entry]
unsafe fn main() -> ! {
    let peripherals = hal::pac::Peripherals::take().unwrap();
    let mut rcc = peripherals.RCC.freeze(hal::rcc::Config::hsi16());
    // let pwr = hal::pwr::PWR::new(peripherals.PWR, &mut rcc);

    let mut core_peripherals = cortex_m::Peripherals::take().unwrap();

    setup_systick(&mut core_peripherals.SYST, rcc.clocks.sys_clk().0, 1_000);

    let gpioa = peripherals.GPIOA.split(&mut rcc);

    let usart_tx_pin = gpioa.pa9;
    let usart_rx_pin = gpioa.pa10;

    let log_serial = peripherals.USART1.usart(
        usart_tx_pin,
        usart_rx_pin,
        hal::serial::Config::default().baudrate(115_200.Bd()),
        &mut rcc
    ).unwrap();

    objects::init_objects(log_serial, gpioa.pa15.into_push_pull_output());

    app::board::BoardInterface::register_callback(app::board::CallbackType::TriggerCrash, || {
        unsafe {
            core::arch::asm!("udf #0");
        }
    });

    app::main();
}


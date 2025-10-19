#![no_std]
#![no_main]

mod exc;
mod util;
mod time;
mod objects;
mod tty;
mod spi;

use cortex_m_rt::entry;

use stm32l0xx_hal as hal;
use hal::prelude::*;

extern crate alloc;
use alloc::boxed::Box;

use rtrs::{ignore, println};

pub const GREEN_LED_NAME: &str = "led_green";
pub const BTN_PIN_NAME: &str = "btn";
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
    let mut core_peripherals = cortex_m::Peripherals::take().unwrap();

    time::setup_systick(&mut core_peripherals.SYST, rcc.clocks.sys_clk().0, 1_000);
    time::setup_tim2();

    let gpioa = peripherals.GPIOA.split(&mut rcc);
    let gpiob = peripherals.GPIOB.split(&mut rcc);

    objects::init_serial(
        peripherals.USART1.usart(
            gpioa.pa9,  // tx
            gpioa.pa10, // rx
            hal::serial::Config::default().baudrate(115_200.Bd()),
            &mut rcc
        ).unwrap()
    );
    // FIXME: Can't use green led on nucleo, because SPI1_CLK is wired there also
    // objects::init_led(gpioa.pa5.into_push_pull_output());
    objects::init_btn(gpioa.pa14.into_pull_down_input());
    objects::init_buzz(gpioa.pa15.into_push_pull_output());
    objects::init_time();

    let mut bus = spi::Spi1Bus::new(
        peripherals.SPI1.spi(
            (
                gpioa.pa5, // clk
                gpioa.pa6, // miso
                gpioa.pa7  // mosi
            ),
            hal::spi::Mode {
                polarity: hal::spi::Polarity::IdleLow,
                phase:    hal::spi::Phase::CaptureOnFirstTransition,
            },
            1000.Hz(),
            &mut rcc
        ),
        gpiob.pb6.into_push_pull_output() // cs
    );

    let tx = [0x42, 0];
    let mut rx = [0, 0];

    use rtrs::bus::Bus;
    ignore!(bus.lock());
    ignore!(bus.transfer(&tx, &mut rx));
    ignore!(bus.unlock());

    use core::fmt::Write;
    println!("{:?}", rx);

    app::board::BoardInterface::register_callback(
        app::board::CallbackType::TriggerCrash(|| {
            unsafe { core::arch::asm!("udf #0"); }
        })
    );

    app::board::BoardInterface::register_callback(
        app::board::CallbackType::MicrosecondDelay(|us| {
            let r = time::SYSCLK.lock();
            time::delay_us(us, *r, 3);
        })
    );

    app::board::BoardInterface::register_callback(
        app::board::CallbackType::MicrosecondTickProvider(|provider| {
            *provider = Box::new(time::MicrosecondTickProvider::new());
        })
    );

    app::main();
}


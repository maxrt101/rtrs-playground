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

use rtrs::sync::{Mutex, RaceAction};

pub const GREEN_LED_NAME: &str = "led_green";

#[allow(dead_code)]
pub struct Board {
    // peripherals: hal::pac::Peripherals,
    core_peripherals: cortex_m::Peripherals,
    rcc: hal::rcc::Rcc,
    // pwr: hal::pwr::PWR
}

pub enum CallbackType {
    Systick
}

struct Callbacks {
    systick: Option<fn()>
}

impl Callbacks {
    const fn empty() -> Self {
        Self { systick: None }
    }
}

static CALLBACKS: Mutex<Callbacks> = Mutex::new(Callbacks::empty(), RaceAction::Crash);

impl Board {
    pub fn register_callback(t: CallbackType, f: fn()) {
        let mut cbs = CALLBACKS.lock_mut();

        match t {
            CallbackType::Systick => {
                (*cbs).systick = Some(f);
            }
        }
    }

    pub fn callback(t: CallbackType) {
        let cbs = CALLBACKS.lock_mut();

        match t {
            CallbackType::Systick => {
                if let Some(f) = (*cbs).systick {
                    f()
                }
            }
        }
    }

    #[inline(never)]
    pub fn crash() -> ! {
        unsafe {
            core::arch::asm!("udf #0");
        }
        unreachable!()
    }
}

fn setup_systick(syst: &mut SYST, core_freq: u32, hz: u32) {
    syst.set_clock_source(SystClkSource::Core);
    syst.set_reload(core_freq / hz - 1);
    syst.clear_current();
    syst.enable_counter();
    syst.enable_interrupt();
}

pub fn init() -> Board {
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

    // let mut lptim = hal::lptim::LpTimer::init_periodic(peripherals.LPTIM, &mut pwr, &mut rcc, hal::lptim::ClockSrc::Lse);
    // lptim.start(1_000_000.Hz());

    objects::init_objects(log_serial, gpioa.pa15.into_push_pull_output());

    // use core::fmt::Write;
    // use rtrs::println;
    // println!("core_freq: {}", rcc.clocks.sys_clk().0);

    Board {
        // peripherals,
        core_peripherals,
        rcc,
        // pwr
    }
}

#[unsafe(no_mangle)]
fn rtrs_lock_acquire() {
    cortex_m::interrupt::disable();
}

#[unsafe(no_mangle)]
fn rtrs_lock_release() {
    unsafe { cortex_m::interrupt::enable() };
}

#[entry]
unsafe fn main() -> ! {
    let _ = init();

    app::main();
}


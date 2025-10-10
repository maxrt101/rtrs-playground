#![no_std]
#![no_main]

mod exc;
mod util;
mod objects;
mod pins;

use core::fmt::Write;
use cortex_m::peripheral::SYST;
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::entry;

use stm32h7xx_hal as hal;
use hal::prelude::*;

use rtrs::sync::{Mutex, RaceAction};

pub const GREEN_LED_NAME: &str = "led_green";

#[allow(dead_code)]
pub struct Board {
    // peripherals: hal::pac::Peripherals,
    // core_peripherals: cortex_m::Peripherals,
    // rcc: hal::pac::RCC,
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

    let rcc = peripherals.RCC.constrain();
    let pwr = peripherals.PWR.constrain();
    let pwrcfg = pwr.freeze();

    // let ccdr = rcc
    //     .sys_ck(100.MHz())   // core clock
    //     // .hclk(200.MHz())     // AHB bus clock
    //     // .pclk1(100.MHz())    // APB1
    //     // .pclk2(100.MHz())    // APB2
    //     .freeze(pwrcfg, &peripherals.SYSCFG);

    // let clocks = ccdr.clocks;

    // let mut core_peripherals = cortex_m::Peripherals::take().unwrap();

    // setup_systick(&mut core_peripherals.SYST, clocks.sysclk().to_Hz(), 1_000);

    // let gpiod = peripherals.GPIOD.split(ccdr.peripheral.GPIOD);
    //
    // let tx = gpiod.pd8.into_alternate();
    // let rx = gpiod.pd9.into_alternate();

    // let gpioc = peripherals.GPIOC.split(ccdr.peripheral.GPIOC);
    //
    // let tx = gpioc.pc10.into_alternate();
    // let rx = gpioc.pc11.into_alternate();

    // USART3
    // let mut log_serial = peripherals
    //     .USART3
    //     .serial((tx, rx), 115_200.bps(), ccdr.peripheral.USART3, &clocks)
    //     .unwrap();

    // log_serial.write_str("Test");

    // let mut lptim = hal::lptim::LpTimer::init_periodic(peripherals.LPTIM, &mut pwr, &mut rcc, hal::lptim::ClockSrc::Lse);
    // lptim.start(1_000_000.Hz());

    loop {}

    // objects::init_objects(log_serial);

    // use core::fmt::Write;
    // use rtrs::println;
    // println!("core_freq: {}", rcc.clocks.sys_clk().0);

    Board {
        // peripherals,
        // core_peripherals,
        // rcc,
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

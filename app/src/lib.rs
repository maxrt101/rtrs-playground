#![no_std]
#![no_main]

mod cmd;
mod led;
mod logs;
mod tests;
pub mod board;

extern crate alloc;

pub(crate) use tests::*;

use core::fmt::Write; // For println!
use core::sync::atomic::{self, Ordering};
use core::panic::PanicInfo;

use rtrs::log::console::CONSOLE_OBJECT_NAME;
use rtrs::object::STORAGE;
use rtrs::task; // For task_yield!
use rtrs::heap_allocator;
use rtrs::println;
use crate::cmd::create_shell;

heap_allocator!(global, pub GLOBAL_HEAP, 2048);

pub fn main() -> ! {
    board::BoardInterface::register_callback(board::CallbackType::Systick, || {
        SYSTICK_EVENT.trigger();
    });

    logs::init_logs();

    println!(
        "\r\n{}----- rtrs-playground v{} -----{}\r\n",
        rtrs::ANSI_COLOR_FG_YELLOW,
        env!("CARGO_PKG_VERSION"),
        rtrs::ANSI_TEXT_RESET
    );

    let mut shell = create_shell();

    // bsp::Board::crash();
    // panic!("Test panic");

    // Test led performance
    // const TIMES: u32 = 1000;
    // println!("led on/off {}: {}", TIMES, timeit!({
    //     for _ in 0..TIMES {
    //         object_with_mut!(bsp::GREEN_LED, rtrs::Led, led, {
    //             led.on();
    //         });
    //         object_with_mut!(bsp::GREEN_LED, rtrs::Led, led, {
    //             led.off();
    //         });
    //     }
    // }));

    loop {
        // object_with_mut!(bsp::GREEN_LED, rtrs::Led, led, {
        //     if !led.is_running() {
        //         led.set_pattern(&led::BLINK_PATTERN);
        //     }
        //
        //     led.cycle();
        // });

        // print!("tick: {}\r", rtrs::time::global_tick());

        shell.cycle();
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // If panic happened during printing, try to allow console to be accessed, disregarding safety.
    // Safety can be ignored because at this stage of execution nothing matters except for trying
    // to deliver panic report - code that panicked won't be executed again
    unsafe {
        // Reset storage borrows - storage can now be accessed, even if active refs still exist
        STORAGE.reset_borrows();

        // Force unlock console mutex
        // STORAGE.with(&|storage| {
        //     storage.unlock(CONSOLE_OBJECT_NAME);
        // })
        let storage = STORAGE.lock();
        storage.unlock(CONSOLE_OBJECT_NAME);
    }

    println!(
        "{}{}        PANIC        {}",
        rtrs::ANSI_COLOR_BG_RED,
        rtrs::ANSI_TEXT_BOLD,
        rtrs::ANSI_TEXT_RESET
    );

    println!(
        "{}Message:{}  {}{}{}",
        rtrs::ANSI_COLOR_FG_CYAN,
        rtrs::ANSI_TEXT_RESET,
        rtrs::ANSI_TEXT_BOLD,
        info.message(),
        rtrs::ANSI_TEXT_RESET
    );

    if let Some(location) = info.location() {
        println!(
            "{}Location:{} {}{}{}",
            rtrs::ANSI_COLOR_FG_CYAN,
            rtrs::ANSI_TEXT_RESET,
            rtrs::ANSI_TEXT_BOLD,
            location,
            rtrs::ANSI_TEXT_RESET
        );
    }

    // TODO: Reboot
    loop {
        atomic::compiler_fence(Ordering::SeqCst);
    }
}


use cortex_m_rt::exception;
use core::sync::atomic::Ordering;
use core::sync::atomic;
use core::fmt::Write;
use rtrs::{object_with, println};

use crate::{print_regs, print_reg};

unsafe fn stack_trace() {
    println!("Stack trace:");

    let mut sp = cortex_m::register::msp::read() as *const u32;
    let mut found = 0;

    const SRAM_BASE: usize = 0x20000000;
    const SRAM_SIZE: usize = 0x00002000; // l051

    const FLASH_BASE: u32 = super::hal::flash::FLASH_START as u32;
    let FLASH_SIZE: u32 = (super::hal::flash::flash_size_in_kb() * 1024) as u32;

    const DEPTH: usize = 16;

    while ((sp as usize) > SRAM_BASE) && ((sp as usize) < (SRAM_BASE + SRAM_SIZE)) && found < DEPTH {
        let value = unsafe { *sp };

        if value >= FLASH_BASE && value < (FLASH_BASE + FLASH_SIZE) {
            println!("#{}:\t0x{:08x}", found, value);
            found += 1;
        }

        sp = unsafe { sp.add(1) };
    }
}

#[exception]
unsafe fn HardFault(ef: &cortex_m_rt::ExceptionFrame) -> ! {
    println!("{}{}        HARD FAULT        {}", rtrs::ANSI_COLOR_BG_RED, rtrs::ANSI_TEXT_BOLD, rtrs::ANSI_TEXT_RESET);

    print_regs!(
        {"R0",   ef.r0()},
        {"R1",   ef.r1()},
        {"R2",   ef.r2()},
        {"R3",   ef.r3()},
        {"R12",  ef.r12()},
        {"PC",   ef.pc()},
        {"LR",   ef.lr()},
        {"xPSR", ef.xpsr()}
    );
    
    // TODO: Minimal stack trace?
    stack_trace();

    // TODO: Reboot
    loop {
        atomic::compiler_fence(Ordering::SeqCst);
    }
}

#[exception]
unsafe fn DefaultHandler(irqn: i16) {
    println!("Unhandled exception: {}", irqn);

    // TODO: Reboot
    loop {
        atomic::compiler_fence(Ordering::SeqCst);
    }
}

#[exception]
fn SysTick() {
    object_with!(rtrs::time::TIME_OBJECT_NAME, rtrs::time::TimeProvider, time, {
        time.increment()
    });

    app::board::BoardInterface::callback(app::board::Callback::Systick)
}


use cortex_m_rt::exception;
use core::sync::atomic::Ordering;
use core::sync::atomic;
use core::fmt::Write;
use rtrs::{object_with, println};

use crate::{print_regs, print_reg};

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

    app::board::BoardInterface::callback(app::board::CallbackType::Systick)
}


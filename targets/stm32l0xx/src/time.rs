use cortex_m::peripheral::SYST;
use cortex_m::peripheral::syst::SystClkSource;
use rtrs::sync::RwLock;

pub(crate) static SYSCLK: RwLock<u32> = RwLock::new(0);

pub(crate) fn setup_systick(syst: &mut SYST, core_freq: u32, hz: u32) {
    syst.set_clock_source(SystClkSource::Core);
    syst.set_reload(core_freq / hz - 1);
    syst.clear_current();
    syst.enable_counter();
    syst.enable_interrupt();

    let mut r = SYSCLK.lock_mut();
    *r = core_freq;
}

pub(crate) fn setup_tim2() {
    let rcc_reg = unsafe { &*crate::hal::pac::RCC::ptr() };
    rcc_reg.apb1enr.modify(|_, w| w.tim2en().set_bit());

    let tim2 = unsafe { &*crate::hal::pac::TIM2::ptr() };

    tim2.psc.write(|w| w.psc().bits(16 - 1));
    tim2.arr.write(|w| w.arr().bits(u16::MAX));
    tim2.cr1.modify(|_, w| w.cen().set_bit());
}

#[inline(never)]
fn delay_cycles(cycles: u32) {
    unsafe {
        core::arch::asm!(
            "1:",
            "   subs {0}, #1",
            "   bne 1b",
            inout(reg) cycles => _,
            options(nomem, nostack, preserves_flags),
        );
    }
}

pub(crate) fn delay_us(us: u32, cpu_hz: u32, cycles_per_loop: u32) {
    let cycles_needed = (us as u64 * cpu_hz as u64) / 1_000_000u64;
    let loops = (cycles_needed + (cycles_per_loop as u64 - 1)) / cycles_per_loop as u64;
    delay_cycles(loops as u32);
}

#[derive(Copy, Clone)]
pub struct MicrosecondTickProvider {
    tick: u32,
    last: u16,
}

impl MicrosecondTickProvider {
    pub fn new() -> Self {
        MicrosecondTickProvider {
            tick: Self::get_tim_cnt() as u32,
            last: Self::get_tim_cnt(),
        }
    }
    
    fn get_tim_cnt() -> u16 {
        let tim2 = unsafe { &*crate::hal::pac::TIM2::ptr() };
        tim2.cnt.read().cnt().bits()
    }
}

impl rtrs::time::TickProvider for MicrosecondTickProvider {
    type Tick = u32;

    fn get_tick(&mut self) -> Self::Tick {
        let tick = Self::get_tim_cnt();
        
        let diff = if tick < self.last {
            u16::MAX - self.last + tick
        } else {
            tick - self.last
        };

        self.tick += diff as u32;
        self.last = tick;
        self.tick
    }
}

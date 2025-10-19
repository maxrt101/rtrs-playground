use core::fmt::Write;
use embedded_hal::serial::Read;

use crate::hal::pac::USART1;
use crate::hal::serial::Serial;

pub struct TtyUSART1Backend {
    serial: Serial<USART1>
}

impl TtyUSART1Backend {
    pub fn new(serial: Serial<USART1>) -> Self {
        Self { serial }
    }
}

impl rtrs::tty::TtyBackend for TtyUSART1Backend {
    fn read(&mut self) -> Option<u8> {
        self.serial.read().ok()
    }

    fn write(&mut self, byte: u8) {
        self.serial.write_char(byte as char).unwrap();
    }
}

unsafe impl Sync for TtyUSART1Backend {}

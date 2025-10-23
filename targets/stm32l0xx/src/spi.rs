use embedded_hal::digital::v2::OutputPin;
use embedded_hal::spi::FullDuplex;
use nb::block;

use crate::hal::spi::Spi;
use crate::hal::pac::SPI1;
use crate::hal::gpio::{Analog, Output, PushPull};
use crate::hal::gpio::gpioa::{PA4, PA5, PA6, PA7};
use crate::hal::gpio::gpiob::{PB6};

use rtrs::bus::Bus;
use rtrs::ok;

use core::fmt::Write;
use rtrs::println;


type Spi1 = Spi<SPI1, (PA5<Analog>, PA6<Analog>, PA7<Analog>)>;

#[cfg(feature = "mcu-stm32l073")]
type CsPin = PB6<Output<PushPull>>;
#[cfg(feature = "mcu-stm32l051")]
type CsPin = PA4<Output<PushPull>>;

pub struct Spi1Bus {
    spi: Spi1,
    cs:  CsPin,
}

impl Spi1Bus {
    pub fn new(spi: Spi1, cs: CsPin) -> Self {
        Self { spi, cs }
    }

    fn clear(&mut self) {
        unsafe {
            (*SPI1::ptr()).sr.read();
            (*SPI1::ptr()).dr.read();
        }
    }
}

impl Bus for Spi1Bus {
    type Error = ();

    fn lock(&mut self) -> Result<(), Self::Error> {
        ok!(self.cs.set_low())
    }

    fn unlock(&mut self) -> Result<(), Self::Error> {
        ok!(self.cs.set_high())
    }

    fn send(&mut self, data: u8) -> Result<(), Self::Error> {
        let res = block!(self.spi.send(data));

        self.clear();

        match res {
            Ok(_) => Ok(()),
            Err(err) => {
                println!("spi1send error {:?}", err);
                Err(())
            }
        }
    }

    fn recv(&mut self) -> Result<u8, Self::Error> {
        let res = block!(self.spi.read());

        self.clear();

        match res {
            Ok(b) => Ok(b),
            Err(err) => {
                println!("spi1recv error {:?}", err);
                Err(())
            }
        }
    }
}

unsafe impl Sync for Spi1Bus {}

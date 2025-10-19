use embedded_hal::digital::v2::OutputPin;
use embedded_hal::spi::FullDuplex;
use nb::block;

use crate::hal::spi::Spi;
use crate::hal::pac::SPI1;
use crate::hal::gpio::{Analog, Output, PushPull};
use crate::hal::gpio::gpioa::{PA5, PA6, PA7};
use crate::hal::gpio::gpiob::{PB6};

use rtrs::bus::Bus;
use rtrs::ok;

type Spi1 = Spi<SPI1, (PA5<Analog>, PA6<Analog>, PA7<Analog>)>;
type CsPin = PB6<Output<PushPull>>;

pub struct Spi1Bus {
    spi: Spi1,
    cs:  CsPin,
}

impl Spi1Bus {
    pub fn new(spi: Spi1, cs: CsPin) -> Self {
        Self { spi, cs }
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
        ok!(block!(self.spi.send(data)))
    }

    fn recv(&mut self) -> Result<u8, Self::Error> {
        ok!(block!(self.spi.read()))
    }
}

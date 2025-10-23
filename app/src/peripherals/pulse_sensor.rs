
extern crate alloc;
use alloc::boxed::Box;

pub trait PulseSensorInterface {
    fn read(&mut self) -> u16;
}

pub struct PulseSensor {
    ifc: Box<dyn PulseSensorInterface + Send + Sync + 'static>,
}

impl PulseSensor {
    pub fn new(ifc: impl PulseSensorInterface + Send + Sync + 'static) -> Self {
        Self { ifc: Box::new(ifc) }
    }

    pub fn read(&mut self) -> u16 {
        (*self.ifc).read()
    }
}

impl rtrs::object::Object for PulseSensor {}

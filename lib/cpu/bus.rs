use std::{cell::RefCell, rc::Rc};

use crate::{Data, io::Device};

#[derive(Debug)]
pub struct Bus<'a> {
    devices: Vec<Rc<RefCell<dyn Device<()> + 'a>>>,
}

impl Default for Bus<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Bus<'a> {
    pub fn new() -> Self {
        Bus {
            devices: vec![],
        }
    }

    pub fn attach_device(&mut self, device: Rc<RefCell<dyn Device<()> + 'a>>) {
        self.devices.push(device);
    }

    /// If an interrupt signal is pending, returns the cause
    pub fn should_interrupt(&self) -> Option<u32> {
        for device in &self.devices {
            let device = device.borrow();
            if let Some(cause) = device.interrupt() {
                return Some(cause);
            }
        }

        None
    }

    pub fn load_at<K: Into<u8>, T: IntoIterator<Item = K>>(&mut self, offset: u32, bin: T)  {
        for (i, byte) in bin.into_iter().enumerate() {
            self.store_byte(offset + i as u32, byte.into()).unwrap();
        }
    }


    pub fn clock(&mut self) {
        for device in &mut self.devices {
            let mut device = device.borrow_mut();
            device.clock();
        }
    }
}

impl<'a> Data<()> for Bus<'_> {
    fn load_byte(&self, addr: u32) -> Result<u8, ()> {
        for device in &self.devices {
            let device = device.borrow();
            let (lower, upper) = device.addr_range();

            if addr >= lower && addr <= upper {
                return device.load_byte(addr);
            }
        }

        Err(())
    }

    fn store_byte(&mut self, addr: u32, byte: u8) -> Result<(), ()> {
        for device in &mut self.devices {
            let mut device = device.borrow_mut();
            let (lower, upper) = device.addr_range();

            if addr >= lower && addr <= upper {
                return device.store_byte(addr, byte);
            }
        }

        Err(())
    }
}

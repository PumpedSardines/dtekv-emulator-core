use std::{cell::RefCell, rc::Rc};

use crate::{
    io::{self, Device},
    Data,
};

#[derive(Debug)]
pub struct Bus {
    devices: Vec<Rc<RefCell<dyn Device<()>>>>,
}

impl Default for Bus {
    fn default() -> Self {
        Self::new()
    }
}

impl Bus {
    pub fn new() -> Self {
        Bus { devices: vec![] }
    }

    pub fn attach_device(&mut self, device: Rc<RefCell<dyn Device<()>>>) {
        self.devices.push(device);
    }
}

impl io::Device<()> for Bus {
    fn addr_range(&self) -> (u32, u32) {
        (0, 0xFFFF_FFFF)
    }

    fn clock(&mut self) {
        for device in &mut self.devices {
            let mut device = device.borrow_mut();
            device.clock();
        }
    }
}

impl io::Interruptable for Bus {
    fn interrupt(&self) -> Option<u32> {
        for device in &self.devices {
            let device = device.borrow();
            if let Some(cause) = device.interrupt() {
                return Some(cause);
            }
        }

        None
    }
}

macro_rules! data_func_on_all_devices {
    ($self:expr, $addr:expr, $func:ident, $($arg:expr),*) => {
        {
            for device in &$self.devices {
                let mut device = device.borrow_mut();
                let (lower, upper) = device.addr_range();

                if $addr >= lower && $addr <= upper {
                    return Ok(device
                        .$func($($arg),*)
                        .expect("A device can't return error in it's address range"));
                }
            }

            Err(())
        }
    };
}

impl Data<()> for Bus {
    fn load_byte(&self, addr: u32) -> Result<u8, ()> {
        data_func_on_all_devices!(self, addr, load_byte, addr)
    }

    fn store_byte(&mut self, addr: u32, byte: u8) -> Result<(), ()> {
        data_func_on_all_devices!(self, addr, store_byte, addr, byte)
    }

    fn load_halfword(&self, addr: u32) -> Result<u16, ()> {
        data_func_on_all_devices!(self, addr, load_halfword, addr)
    }

    fn store_halfword(&mut self, addr: u32, halfword: u16) -> Result<(), ()> {
        data_func_on_all_devices!(self, addr, store_halfword, addr, halfword)
    }

    fn load_word(&self, addr: u32) -> Result<u32, ()> {
        data_func_on_all_devices!(self, addr, load_word, addr)
    }

    fn store_word(&mut self, addr: u32, word: u32) -> Result<(), ()> {
        data_func_on_all_devices!(self, addr, store_word, addr, word)
    }
}

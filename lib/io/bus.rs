//! You should probably not use the default bus implementation. It is very general purpose and is
//! quite slow. You should implement your own bus for your specific needs. This is mostly here for
//! completeness and for testing purposes.

use crate::io::{self, Data, Device};

#[derive(Debug)]
pub struct Bus {
    devices: Vec<((u32, u32), Box<dyn Device<()>>)>,
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

    pub fn attach_device(&mut self, range: (u32, u32), device: Box<dyn Device<()>>) {
        self.devices.push((range, device));
    }
}

impl io::Device<()> for Bus {
    fn clock(&mut self) {
        for (_, device) in &mut self.devices {
            device.clock();
        }
    }
}

impl io::Interruptable for Bus {
    fn interrupt(&self) -> Option<u32> {
        for (_, device) in &self.devices {
            if let Some(cause) = device.interrupt() {
                return Some(cause);
            }
        }

        None
    }
}

impl Data<()> for Bus {
    fn load_byte(&self, addr: u32) -> Result<u8, ()> {
        for ((lower, higher), device) in &self.devices {
            if addr >= *lower && addr <= *higher {
                return Ok(device.load_byte(addr).unwrap_or_else(|_| {
                    panic!("Device failed to load byte at address {:#010x}", addr)
                }));
            }
        }

        Err(())
    }

    fn store_byte(&mut self, addr: u32, byte: u8) -> Result<(), ()> {
        for ((lower, higher), device) in &mut self.devices {
            if addr >= *lower && addr <= *higher {
                return Ok(device.store_byte(addr, byte).unwrap_or_else(|_| {
                    panic!("Device failed to store byte at address {:#010x}", addr)
                }));
            }
        }

        Err(())
    }
}

use std::fmt;

use crate::{
    interrupt::InterruptSignal, memory_mapped::MemoryMapped, peripheral::Peripheral
};

/// You should probably not use the default bus implementation. It is very general purpose and is
/// quite slow. You should implement your own bus for your specific needs. This is mostly here for
/// completeness and for testing purposes.
pub struct Bus {
    devices: Vec<((u32, u32), Box<dyn Peripheral<()>>)>,
}

impl Default for Bus {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for Bus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Bus {{ ... }}")
    }
}

impl Bus {
    pub fn new() -> Self {
        Bus { devices: vec![] }
    }

    pub fn attach_device(&mut self, range: (u32, u32), device: Box<dyn Peripheral<()>>) {
        self.devices.push((range, device));
    }
}

impl Peripheral<()> for Bus {
    fn poll_interrupt(&self) -> Option<InterruptSignal> {
        for (_, device) in &self.devices {
            if let Some(signal) = device.poll_interrupt() {
                return Some(signal);
            }
        }

        None
    }
}

impl MemoryMapped<()> for Bus {
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

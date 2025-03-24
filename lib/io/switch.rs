use crate::{exception, io, utils};

#[derive(Clone)]
pub struct Switch {
    state: u32,
    interrupt_mask: u32,
    edge_cap: u32,
}

pub const SWITCH_LOWER_ADDR: u32 = 0x04000010;
pub const SWITCH_HIGHER_ADDR: u32 = 0x400001f;

impl Switch {
    /// Returns a new Memory object with a given size all set to 0
    pub fn new() -> Self {
        Switch {
            state: 0,
            interrupt_mask: 0,
            edge_cap: 0,
        }
    }

    pub fn set(&mut self, index: u32, high: bool) {
        if high {
            self.state |= 1 << index;
        } else {
            self.state &= !(1 << index);
        }

        self.edge_cap |= 1 << index;
    }

    pub fn get(&self, index: u32) -> bool {
        (self.state & (1 << index)) != 0
    }
}

impl io::Device<()> for Switch {}

impl io::Interruptable for Switch {
    fn interrupt(&self) -> Option<u32> {
        let interrupt_condition = (self.edge_cap & self.interrupt_mask) != 0;

        if interrupt_condition {
            Some(exception::SWITCH_INTERRUPT)
        } else {
            None
        }
    }
}

impl io::Data<()> for Switch {
    fn load_byte(&self, addr: u32) -> Result<u8, ()> {
        let addr = addr - SWITCH_LOWER_ADDR;

        // TODO: Edge capture
        Ok(match addr {
            0 => (self.state & 0xFF) as u8,
            1 => ((self.state >> 8) & 0x3) as u8,
            _ => 0,
        })
    }

    fn store_byte(&mut self, addr: u32, byte: u8) -> Result<(), ()> {
        let addr = addr - SWITCH_LOWER_ADDR;

        let part = addr / 4;
        // Do nothing, hardwired to nothing
        match part {
            0 => {} // Data address, can store here
            1 => {} // Direction address, can store here, but changes nothing
            2 => {
                // Interrupt mask
                self.interrupt_mask = utils::set_in_u32(self.interrupt_mask, byte, addr);
            }
            3 => {
                // Edge capture
                self.edge_cap = utils::set_in_u32(self.edge_cap, byte, addr);
            }
            _ => unreachable!("The switch address space is only 4 words long, if this error happens, update the bus module"),
        };

        Ok(())
    }
}

impl std::fmt::Debug for Switch {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Switch {{ ... }}")
    }
}

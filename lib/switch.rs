use crate::LoadStore;

#[derive(Clone)]
pub struct Switch {
    pub state: u32,
    pub interrupt_mask: u32,
    pub edge_cap: u32,
}

impl Switch {
    /// Returns a new Memory object with a given size all set to 0
    pub fn new() -> Self {
        Switch {
            state: 0,
            interrupt_mask: 0,
            edge_cap: 0,
        }
    }

    pub fn set_switch(&mut self, index: u32, high: bool) {
        if high {
            self.state |= 1 << index;
        } else {
            self.state &= !(1 << index);
        }

        self.edge_cap |= 1 << index;
    }

    pub fn get_switch(&self, index: u32) -> bool {
        (self.state & (1 << index)) != 0
    }

    pub fn should_interrupt(&self) -> bool {
        (self.edge_cap & self.interrupt_mask) != 0
    }
}

impl LoadStore for Switch {
    fn load_byte(&self, addr: u32) -> u8 {
        // TODO: Edge capture
        match addr {
            0 => (self.state & 0xFF) as u8,
            1 => ((self.state >> 8) & 0x3) as u8,
            _ => 0,
        }
    }

    fn store_byte(&mut self, addr: u32, byte: u8) {
        let part = addr / 4;
        // Do nothing, hardwired to nothing
        match part {
            0 => {} // Data address, can store here
            1 => {} // Direction address, can store here, but changes nothing
            2 => { // Interrupt mask
                let i = addr % 4;
                self.interrupt_mask =
                    (self.interrupt_mask & !(0xFF << (i * 8))) | (byte as u32) << (i * 8);
            }
            3 => { // Edge capture
                let i = addr % 4;
                self.edge_cap =
                    (self.edge_cap & !(0xFF << (i * 8))) | (byte as u32) << (i * 8);
            }
            _ => unreachable!(),
        };
    }
}

impl std::fmt::Debug for Switch {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Switch {{ ... }}")
    }
}

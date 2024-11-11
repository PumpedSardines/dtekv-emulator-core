use crate::{utils, Data};

#[derive(Clone)]
pub struct Button {
    pressed: bool,
    interrupt_mask: u32,
    edge_cap: u32,
}

impl Button {
    /// Returns a new Memory object with a given size all set to 0
    pub fn new() -> Self {
        Button {
            pressed: false,
            interrupt_mask: 0,
            edge_cap: 0,
        }
    }

    pub fn set(&mut self, pressed: bool) {
        self.pressed = pressed;
        self.edge_cap |= 1;
    }

    pub fn get(&self) -> bool {
        self.pressed
    }

    pub fn should_interrupt(&self) -> bool {
        (self.edge_cap & self.interrupt_mask) != 0
    }
}

impl Data<()> for Button {
    fn load_byte(&self, addr: u32) -> Result<u8, ()> {
        Ok(if addr == 0 {
            match self.pressed {
                true => 1,
                false => 0,
            }
        } else {
            // hard wired to 0
            0
        })
    }

    fn store_byte(&mut self, addr: u32, byte: u8) -> Result<(), ()> {
        let part = addr / 4;
        // Hard wired to nothing
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
            _ => unreachable!("The button address space is only 4 words long, if this error happens, update the bus module"),
        };

        Ok(())
    }
}

impl std::fmt::Debug for Button {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Button {{ ... }}")
    }
}

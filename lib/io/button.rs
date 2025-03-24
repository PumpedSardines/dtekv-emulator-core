use crate::{exception, io, io::Data, utils};

#[derive(Clone)]
pub struct Button {
    pressed: bool,
    interrupt_mask: u32,
    edge_cap: u32,
}

pub const BUTTON_LOWER_ADDR: u32 = 0x040000d0;
pub const BUTTON_HIGHER_ADDR: u32 = 0x040000df;

impl Button {
    /// Returns a new Memory object with a given size all set to 0
    pub fn new() -> Self {
        Button {
            pressed: false,
            interrupt_mask: 0,
            edge_cap: 0,
        }
    }

    pub const fn bounds() -> [(u32, u32); 1] {
        [(BUTTON_LOWER_ADDR, BUTTON_HIGHER_ADDR)]
    }

    pub fn set(&mut self, pressed: bool) {
        self.pressed = pressed;
        self.edge_cap |= 1;
    }

    pub fn get(&self) -> bool {
        self.pressed
    }

    fn should_interrupt(&self) -> bool {
        (self.edge_cap & self.interrupt_mask) != 0
    }
}

impl io::Device<()> for Button {}

impl io::Interruptable for Button {
    fn interrupt(&self) -> Option<u32> {
        if self.should_interrupt() {
            Some(exception::BUTTON_INTERRUPT)
        } else {
            None
        }
    }
}

impl Data<()> for Button {
    fn load_byte(&self, addr: u32) -> Result<u8, ()> {
        let addr = addr - BUTTON_LOWER_ADDR;
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
        let addr = addr - BUTTON_LOWER_ADDR;
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

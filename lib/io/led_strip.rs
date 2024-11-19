use crate::{io, utils, Data};

#[derive(Clone)]
pub struct LEDStrip {
    leds: u32,
}

pub const LED_STRIP_LOWER_ADDR: u32 = 0x04000000;
pub const LED_STRIP_HIGHER_ADDR: u32 = 0x0400000F;

impl LEDStrip {
    /// Returns a new Memory object with a given size all set to 0
    pub fn new() -> Self {
        LEDStrip { leds: 0 }
    }

    pub fn get(&self, index: u32) -> bool {
        ((self.leds >> index) & 0x1) == 1
    }
}

impl Default for LEDStrip {
    fn default() -> Self {
        Self::new()
    }
}

impl io::Device<()> for LEDStrip {
    fn addr_range(&self) -> (u32, u32) {
        (LED_STRIP_LOWER_ADDR, LED_STRIP_HIGHER_ADDR)
    }

    fn clock(&mut self) {}
}

impl io::Interruptable for LEDStrip {
    fn interrupt(&self) -> Option<u32> {
        None
    }
}

impl Data<()> for LEDStrip {
    fn load_byte(&self, _addr: u32) -> Result<u8, ()> {
        // hard wired to 0
        Ok(0)
    }

    fn store_byte(&mut self, addr: u32, byte: u8) -> Result<(), ()> {
        let addr = addr - LED_STRIP_LOWER_ADDR;
        let index = addr % 4;

        let byte = match index { // Ignore the upper bits, only let through the lower 10 bits
            0 => byte,
            1 => byte & 0x3,
            2 => 0,
            3 => 0,
            _ => unreachable!(),
        };

        self.leds = utils::set_in_u32(self.leds, byte, addr);
    

        Ok(())
    }
}

impl std::fmt::Debug for LEDStrip {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Leds {{ ... }}")
    }
}

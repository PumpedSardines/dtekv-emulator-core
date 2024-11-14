use crate::{io, Data};

pub const VGA_DMA_LOWER_ADDR: u32 = 0x08000000;
pub const VGA_DMA_HIGHER_ADDR: u32 = 0x80257ff;

pub struct VgaDma {}

impl Default for VgaDma {
    fn default() -> Self {
        Self::new()
    }
}

impl VgaDma {
    /// Returns a new Memory object with a given size all set to 0
    pub fn new() -> Self {
        VgaDma {}
    }
}

impl io::Device<()> for VgaDma {
    fn addr_range(&self) -> (u32, u32) {
        (VGA_DMA_LOWER_ADDR, VGA_DMA_HIGHER_ADDR)
    }

    fn clock(&mut self) {}
}

impl io::Interruptable for VgaDma {
    fn interrupt(&self) -> Option<u32> {
        None
    }
}

impl Data<()> for VgaDma {
    fn load_byte(&self, _addr: u32) -> Result<u8, ()> {
        // Hard wired to 0
        Ok(0)
    }

    fn store_byte(&mut self, _addr: u32, _byte: u8) -> Result<(), ()> {
        // Hard wired to 0
        Ok(())
    }
}

impl std::fmt::Debug for VgaDma {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Vga {{ ... }}")
    }
}

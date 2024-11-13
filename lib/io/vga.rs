use crate::{Data, io};


pub const VGA_LOWER_ADDR: u32 = 0x08000000;
pub const VGA_HIGHER_ADDR: u32 = 0x80257ff;

pub struct Vga {
    pixels: [u8; 320 * 240],
    has_changed: bool,
}

impl Default for Vga {
    fn default() -> Self {
        Self::new()
    }
}

impl Vga {
    /// Returns a new Memory object with a given size all set to 0
    pub fn new() -> Self {
        Vga {
            has_changed: false,
            pixels: [0; 320 * 240],
        }
    }

    pub fn has_changed(&self) -> bool {
        self.has_changed
    }

    pub fn reset_has_changed(&mut self) {
        self.has_changed = false;
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> (u8, u8, u8) {
        let pixel = self.pixels[(y * 320 + x) as usize];
        let red = pixel & 0b11100000;
        let green = pixel & 0b00011100;
        let blue = pixel & 0b00000011;

        ((red >> 5) * 32, (green >> 2) * 32, blue * 85)
    }
}

impl io::Device<()> for Vga {
    fn addr_range(&self) -> (u32, u32) {
        (VGA_LOWER_ADDR, VGA_HIGHER_ADDR)
    }

    fn clock(&mut self) {}
}

impl io::Interruptable for Vga {
    fn interrupt(&self) -> Option<u32> {
        None
    }
}

impl Data<()> for Vga {
    fn load_byte(&self, _addr: u32) -> Result<u8, ()> {
        // Hard wired to 0
        Ok(0)
    }

    fn store_byte(&mut self, addr: u32, byte: u8) -> Result<(), ()> {
        let addr = addr - VGA_LOWER_ADDR;
        if addr >= self.pixels.len() as u32 {
            return Err(());
        }
        let addr = addr as usize;
        let last_pixel = self.pixels[addr];
        if last_pixel != byte {
            self.has_changed = true;
        }
        self.pixels[addr] = byte;
        Ok(())
    }
}

impl std::fmt::Debug for Vga {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Vga {{ ... }}")
    }
}

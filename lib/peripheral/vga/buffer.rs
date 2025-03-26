pub const VGA_BUFFER_LOWER_ADDR: u32 = 0x08000000;
pub const VGA_BUFFER_HIGHER_ADDR: u32 = 0x80257ff;

use std::{cell::RefCell, rc::Rc};

use crate::{debug_console::DebugConsole, memory_mapped::MemoryMapped, peripheral::{self, Peripheral}};

pub trait VgaBufferRenderer {
    fn set_pixel(&mut self, index: u32, color: (u8, u8, u8));
    fn get_pixel(&self, index: u32) -> (u8, u8, u8);
    fn set_buffer(&mut self, buffer: u32);
}

pub struct VgaBuffer<T: VgaBufferRenderer> {
    pub renderer: T,
    pub last_buffer_index: u32,
    #[cfg(feature = "debug-console")]
    debug_console: Option<Rc<RefCell<DebugConsole>>>,
}

impl<T: VgaBufferRenderer> VgaBuffer<T> {
    /// Returns a new Memory object with a given size all set to 0
    pub fn new() -> Self {
        VgaBuffer {
            // One for each color channel
            // Framebuffer
            // Backbuffer
            // Extra space
            // buffer: Uint8Array::new_with_length(320 * 240 * 3 * 3),
            //
            last_buffer_index: io::VGA_BUFFER_LOWER_ADDR,
            #[cfg(feature = "debug-console")]
            debug_console: None,
        }
    }

    pub fn get(&mut self) -> Uint8Array {
        // let buffer = match self.dma.borrow().get_buffer() {
        //     // If less than 0x8000000, it's the first buffer
        //     buffer if buffer < io::VGA_BUFFER_LOWER_ADDR => io::VGA_BUFFER_LOWER_ADDR,
        //     buffer if buffer > io::VGA_BUFFER_HIGHER_ADDR => io::VGA_BUFFER_HIGHER_ADDR,
        //     buffer => buffer,
        // };

        // self.last_buffer_index = buffer;
        //
        // let start = (buffer - io::VGA_BUFFER_LOWER_ADDR) * 3;
        // let end = start + 320 * 240 * 3;
        //
        // self.buffer.subarray(start, end)
    }

    pub fn should_update(&self) -> bool {
        self.dma.borrow().get_buffer() != self.last_buffer_index
    }

    pub fn to_color(&self, pixel: u8) -> (u8, u8, u8) {
        let red = pixel & 0b11100000;
        let green = pixel & 0b00011100;
        let blue = pixel & 0b00000011;

        ((red >> 5) * 32, (green >> 2) * 32, blue * 85)
    }
}

impl<T: VgaBufferRenderer> Peripheral<()> for VgaBuffer<T> {}

impl<T: VgaBufferRenderer> MemoryMapped<()> for VgaBuffer<T> {
    fn load_byte(&self, addr: u32) -> Result<u8, ()> {
        let addr = addr - VGA_BUFFER_LOWER_ADDR;
        let addr = addr * 3;
        let (red, green, blue) = self.renderer.get_pixel(addr);
        let color = (red / 32) << 5 | (green / 32) << 2 | (blue / 85);
        Ok(color)
    }

    fn store_byte(&mut self, addr: u32, byte: u8) -> Result<(), ()> {
        let addr = addr - VGA_BUFFER_LOWER_ADDR;
        let addr = addr * 3;
        let (red, green, blue) = self.to_color(byte);
        self.renderer.set_pixel(addr, (red, green, blue));
        Ok(())
    }
}

impl std::fmt::Debug for VgaBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Vga {{ ... }}")
    }
}

pub const VGA_BUFFER_LOWER_ADDR: u32 = 0x08000000;
pub const VGA_BUFFER_HIGHER_ADDR: u32 = 0x80257ff;

use std::{cell::RefCell, rc::Rc};

use super::{channel::Channel, Renderer};
use crate::{debug_console::DebugConsole, memory_mapped::MemoryMapped, peripheral::Peripheral};

pub struct Buffer<'a, T: Renderer> {
    channel: &'a Channel<T>,
    buffer: Vec<u8>,
    #[cfg(feature = "debug-console")]
    debug_console: Option<Rc<RefCell<DebugConsole>>>,
}

impl<'a, T: Renderer> Buffer<'a, T> {
    pub fn new(channel: &'a Channel<T>) -> Self {
        Buffer {
            channel,
            buffer: vec![0; 320 * 240 * 3],
            #[cfg(feature = "debug-console")]
            debug_console: None,
        }
    }

    fn to_color(&self, pixel: u8) -> (u8, u8, u8) {
        let red = pixel & 0b11100000;
        let green = pixel & 0b00011100;
        let blue = pixel & 0b00000011;

        ((red >> 5) * 32, (green >> 2) * 32, blue * 85)
    }
}

impl<'a, T: Renderer> Peripheral<()> for Buffer<'a, T> {}
impl<'a, T: Renderer> MemoryMapped<()> for Buffer<'a, T> {
    fn load_byte(&self, addr: u32) -> Result<u8, ()> {
        let addr = addr - VGA_BUFFER_LOWER_ADDR;
        if addr > VGA_BUFFER_HIGHER_ADDR {
            return Err(());
        }
        Ok(self.buffer[addr as usize])
    }

    fn store_byte(&mut self, addr: u32, byte: u8) -> Result<(), ()> {
        let addr = addr - VGA_BUFFER_LOWER_ADDR;
        if addr > VGA_BUFFER_HIGHER_ADDR {
            return Err(());
        }

        #[cfg(feature = "debug-console")]
        if self.channel.is_swapping() {
            if let Some(debug_console) = &self.debug_console {
                debug_console.borrow_mut().render_while_swapping();
            }
        }

        self.buffer[addr as usize] = byte;
        self.channel.set_pixel(addr, self.to_color(byte));

        Ok(())
    }
}

impl<'a, T: Renderer> std::fmt::Debug for Buffer<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Vga {{ ... }}")
    }
}

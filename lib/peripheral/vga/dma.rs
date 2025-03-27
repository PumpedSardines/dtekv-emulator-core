use crate::{memory_mapped::MemoryMapped, peripheral::Peripheral, utils};

use super::{buffer::VGA_BUFFER_LOWER_ADDR, channel::Channel, Renderer };

pub const VGA_DMA_LOWER_ADDR: u32 = 0x4000100;
pub const VGA_DMA_HIGHER_ADDR: u32 = 0x400010f;

pub struct Dma<'a, T: Renderer> {
    channel: &'a Channel<T>,
    buffer_offset: u32,
    back_buffer: u32,
    enable: bool,
}

enum VgaDmaPart {
    Buffer,
    BackBuffer,
    Resolution,
    StatusControl,
}

impl From<u32> for VgaDmaPart {
    fn from(value: u32) -> Self {
        match value / 4 {
            0 => VgaDmaPart::Buffer,
            1 => VgaDmaPart::BackBuffer,
            2 => VgaDmaPart::Resolution,
            3 => VgaDmaPart::StatusControl,
            _ => panic!("VGA DMA only has 4 sections, {}", value),
        }
    }
}

impl<'a, T: Renderer> Dma<'a, T> {
    /// Returns a new Memory object with a given size all set to 0
    pub fn new(channel: &'a Channel<T>) -> Self {
        Dma {
            channel,
            buffer_offset: VGA_BUFFER_LOWER_ADDR,
            back_buffer: VGA_BUFFER_LOWER_ADDR,
            enable: false,
        }
    }

    pub fn get_buffer(&self) -> u32 {
        self.buffer_offset
    }

    pub fn handle_swap(&mut self) {
        // Swap the buffers if needed
        if self.channel.is_swapping() {
            let temp = self.buffer_offset;

            self.buffer_offset = self.back_buffer;
            self.channel.set_buffer_offset(self.buffer_offset);
            self.back_buffer = temp;
        }
        self.channel.finish_swap();
    }
}

impl<'a, T: Renderer> Peripheral<()> for Dma<'a, T> {}

impl<'a, T: Renderer> MemoryMapped<()> for Dma<'a, T> {
    fn load_byte(&self, addr: u32) -> Result<u8, ()> {
        let addr = addr - VGA_DMA_LOWER_ADDR;
        let part: VgaDmaPart = addr.into();
        let index = addr & 0b11;

        match part {
            VgaDmaPart::Buffer => Ok(utils::get_in_u32(self.buffer_offset, addr)),
            VgaDmaPart::BackBuffer => Ok(utils::get_in_u32(self.back_buffer, addr)),
            VgaDmaPart::Resolution => {
                const RESOLUTION: u32 = (240 << 16) | 320;
                Ok(utils::get_in_u32(RESOLUTION, index))
            }
            VgaDmaPart::StatusControl => {
                let mut value = 0;
                if self.channel.is_swapping() {
                    value |= 0b1;
                }
                value = value | (1 << 1); // The Addressing mode is always 1
                if self.enable {
                    value |= 0b100;
                }
                // 5..3 reserved
                // 7..6 always 0
                // 11..8 always 0
                // 15.12 reserved
                // 23..16 always 0
                value |= 17 << 24;

                Ok(utils::get_in_u32(value, index))
            }
        }
    }

    fn store_byte(&mut self, addr: u32, byte: u8) -> Result<(), ()> {
        let addr = addr - VGA_DMA_LOWER_ADDR;
        let part: VgaDmaPart = addr.into();
        let index = addr & 0b11;

        match part {
            VgaDmaPart::Buffer => {
                self.channel.start_swap();
            }
            VgaDmaPart::BackBuffer => {
                self.back_buffer = utils::set_in_u32(self.back_buffer, byte, index);
            }
            VgaDmaPart::Resolution => {
                // Do nothing
            }
            VgaDmaPart::StatusControl => {
                if index == 0 {
                    self.enable = byte & 0b100 == 1;
                }
            }
        };

        Ok(())
    }
}

impl<'a, T: Renderer> std::fmt::Debug for Dma<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Vga {{ ... }}")
    }
}

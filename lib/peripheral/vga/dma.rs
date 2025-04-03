use crate::{memory_mapped::MemoryMapped, peripheral::Peripheral, utils};

use super::{
    buffer::{VGA_BUFFER_HIGHER_ADDR, VGA_BUFFER_LOWER_ADDR},
    channel::Channel,
    Renderer,
};

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
    /// Create a new DMA peripheral
    pub fn new(channel: &'a Channel<T>) -> Self {
        Dma {
            channel,
            buffer_offset: VGA_BUFFER_LOWER_ADDR,
            back_buffer: VGA_BUFFER_LOWER_ADDR,
            enable: false,
        }
    }

    /// **TL;DR: Call this function 60 times a second to handle scheduled swaps**
    ///
    /// The DMA swapping works by the code writing to a specific memory region to signal that it
    /// wants to swap the buffers. However a swap takes time on the chip. Therefore writing to the
    /// swap bit will only schedule a swap, not trigger one.
    ///
    /// When emulating we don't really have to worry about this delay so we can just swap
    /// instantly, however this meant that a lot of users had code that worked on the emulator but
    /// not on the real hardware since checking the swap bit is optional on the emulator but not on
    /// the hardware.
    ///
    /// Therefore i've also implemented schedule functionality like this that will schedule a swap
    /// and execute the swap at a later time. Calling this function will handle a scheduled swap.
    /// Preferably call this function 60 times a second or something like that.
    pub fn handle_swap(&mut self) {
        // Swap the buffers if needed
        if self.channel.is_swapping() {
            let temp = self.buffer_offset;

            self.buffer_offset = self.back_buffer;
            self.channel.set_buffer_offset(u32::clamp(
                self.buffer_offset - VGA_BUFFER_LOWER_ADDR,
                0,
                VGA_BUFFER_HIGHER_ADDR - VGA_BUFFER_LOWER_ADDR,
            ));
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

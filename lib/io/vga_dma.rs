use crate::{io, utils};

pub const VGA_DMA_LOWER_ADDR: u32 = 0x4000100;
pub const VGA_DMA_HIGHER_ADDR: u32 = 0x400010f;

pub struct VgaDma {
    buffer: u32,
    back_buffer: u32,
    enable: bool,
    is_swapping: bool,
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

impl Default for VgaDma {
    fn default() -> Self {
        Self::new()
    }
}

impl VgaDma {
    /// Returns a new Memory object with a given size all set to 0
    pub fn new() -> Self {
        VgaDma {
            buffer: 0x08000000,
            back_buffer: 0x08000000,
            enable: false,
            is_swapping: false,
        }
    }

    pub fn get_buffer(&self) -> u32 {
        self.buffer
    }
}

impl io::Device<()> for VgaDma {
    fn clock(&mut self) {
        // Swap the buffers if needed
        if self.is_swapping {
            let temp = self.buffer;
            self.buffer = self.back_buffer;
            self.back_buffer = temp;
        }
        self.is_swapping = false;
    }
}

impl io::Interruptable for VgaDma {}

impl io::Data<()> for VgaDma {
    fn load_byte(&self, addr: u32) -> Result<u8, ()> {
        let addr = addr - VGA_DMA_LOWER_ADDR;
        let part: VgaDmaPart = addr.into();
        let index = addr & 0b11;

        match part {
            VgaDmaPart::Buffer => Ok(utils::get_in_u32(self.buffer, addr)),
            VgaDmaPart::BackBuffer => Ok(utils::get_in_u32(self.back_buffer, addr)),
            VgaDmaPart::Resolution => {
                const RESOLUTION: u32 = (240 << 16) | 320;
                Ok(utils::get_in_u32(RESOLUTION, index))
            }
            VgaDmaPart::StatusControl => {
                let mut value = 0;
                if self.is_swapping {
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
                self.is_swapping = true; // Schedule a swap
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

impl std::fmt::Debug for VgaDma {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Vga {{ ... }}")
    }
}

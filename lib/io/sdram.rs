use crate::{io, Data};

pub const SDRAM_SIZE: usize = 0x4000000;
pub const SDRAM_LOWER_ADDR: u32 = 0;
pub const SDRAM_HIGHER_ADDR: u32 = SDRAM_LOWER_ADDR + SDRAM_SIZE as u32 - 1;

#[derive(Clone)]
#[cfg(target_endian = "little")]
pub struct SDRam {
    mem: Vec<u32>,
}

#[derive(Clone)]
#[cfg(target_endian = "big")]
pub struct SDRam {
    mem: Vec<u8>,
}

impl Default for SDRam {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(target_endian = "little")]
impl SDRam {
    pub fn new() -> SDRam {
        SDRam {
            mem: vec![0; SDRAM_SIZE / 4],
        }
    }
}

#[cfg(target_endian = "big")]
impl SDRam {
    pub fn new() -> SDRam {
        SDRam {
            mem: vec![0; SDRAM_SIZE],
        }
    }
}

impl io::Device<()> for SDRam {}
impl io::Interruptable for SDRam {
    fn interrupt(&self) -> Option<u32> {
        None
    }
}

#[cfg(target_endian = "big")]
impl Data<()> for SDRam {
    fn load_byte(&self, addr: u32) -> Result<u8, ()> {
        let addr = addr - SDRAM_LOWER_ADDR;
        if addr > SDRAM_HIGHER_ADDR {
            return Err(());
        }
        Ok(self.mem[addr as usize])
    }

    fn store_byte(&mut self, addr: u32, byte: u8) -> Result<(), ()> {
        let addr = addr - SDRAM_LOWER_ADDR;
        if addr as usize > self.mem.len() {
            return Err(());
        }

        self.mem[addr as usize] = byte;
        Ok(())
    }
}

#[cfg(target_endian = "little")]
impl Data<()> for SDRam {
    // A lot of unsafe code here, here's an explanation:
    //
    // Since the DTEK-V memory is in essence just a large 32 bit array layed out in little endian
    // we can emulate that pretty easily on little endian targets. We use a Vec<u32> to store the
    // bytes and then we can access the data as a u8 slice when we need to load or store bytes.
    //
    // So this is pretty safe, unsafe code really and it speeds up the sdram pretty significantly
    // when targeting web-asm :)

    fn load_byte(&self, addr: u32) -> Result<u8, ()> {
        let addr = addr - SDRAM_LOWER_ADDR;
        if addr > SDRAM_HIGHER_ADDR {
            return Err(());
        }
        Ok(unsafe { *(self.mem.as_ptr() as *const u8).add(addr as usize) })
    }

    fn store_byte(&mut self, addr: u32, byte: u8) -> Result<(), ()> {
        let addr = addr - SDRAM_LOWER_ADDR;
        if addr >= SDRAM_HIGHER_ADDR {
            return Err(());
        }

        unsafe {
            *(self.mem.as_mut_ptr() as *mut u8).add(addr as usize) = byte;
        }
        Ok(())
    }

    fn load_halfword(&self, addr: u32) -> Result<u16, ()> {
        let addr = addr - SDRAM_LOWER_ADDR;
        if addr + 1 > SDRAM_HIGHER_ADDR {
            return Err(());
        }
        Ok(unsafe { *((self.mem.as_ptr() as *const u8).add(addr as usize) as *const u16) })
    }

    fn store_halfword(&mut self, addr: u32, halfword: u16) -> Result<(), ()> {
        let addr = addr - SDRAM_LOWER_ADDR;
        if addr + 1 > SDRAM_HIGHER_ADDR {
            return Err(());
        }

        unsafe {
            *((self.mem.as_mut_ptr() as *mut u8).add(addr as usize) as *mut u16) = halfword;
        }
        Ok(())
    }

    fn load_word(&self, addr: u32) -> Result<u32, ()> {
        let addr = addr - SDRAM_LOWER_ADDR;
        if addr + 3 > SDRAM_HIGHER_ADDR {
            return Err(());
        }
        Ok(unsafe { *((self.mem.as_ptr() as *const u8).add(addr as usize) as *const u32) })
    }

    fn store_word(&mut self, addr: u32, word: u32) -> Result<(), ()> {
        let addr = addr - SDRAM_LOWER_ADDR;
        if addr + 3 > SDRAM_HIGHER_ADDR {
            return Err(());
        }

        unsafe {
            *((self.mem.as_mut_ptr() as *mut u8).add(addr as usize) as *mut u32) = word;
        }
        Ok(())
    }
}

impl std::fmt::Debug for SDRam {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Memory {{ ... }}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Testing the unsafe code in the SDRam implementation
    #[test]
    fn test_error_on_out_of_bounds() {
        let mut sdram = SDRam::new();
        assert_eq!(sdram.load_byte(SDRAM_HIGHER_ADDR + 1), Err(()));
        assert_eq!(sdram.store_halfword(SDRAM_HIGHER_ADDR, 0), Err(()));
        assert_eq!(sdram.store_word(SDRAM_HIGHER_ADDR - 2, 0), Err(()));
    }
}

use crate::{io, Data};

pub const SDRAM_SIZE: usize = 0x4000000;
pub const SDRAM_LOWER_ADDR: u32 = 0;
pub const SDRAM_HIGHER_ADDR: u32 = SDRAM_LOWER_ADDR + SDRAM_SIZE as u32 - 1;

#[derive(Clone)]
#[cfg(target_endian = "little")]
pub struct SDRam {
    mem: Vec<u32>,
}

#[cfg(target_endian = "little")]
impl Default for SDRam {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(target_endian = "little")]
impl SDRam {
    pub fn new() -> SDRam {
        SDRam {
            mem: vec![0; SDRAM_SIZE],
        }
    }
}

#[cfg(target_endian = "little")]
impl io::Device<()> for SDRam {
    fn addr_range(&self) -> (u32, u32) {
        (SDRAM_LOWER_ADDR, SDRAM_HIGHER_ADDR)
    }
}

#[cfg(target_endian = "little")]
impl io::Interruptable for SDRam {
    fn interrupt(&self) -> Option<u32> {
        None
    }
}

#[cfg(target_endian = "little")]
impl Data<()> for SDRam {
    fn load_byte(&self, addr: u32) -> Result<u8, ()> {
        let addr = addr - SDRAM_LOWER_ADDR;
        if addr >= SDRAM_HIGHER_ADDR {
            return Err(());
        }
        Ok(unsafe { *(self.mem.as_ptr() as *const u8).add(addr as usize) })
    }

    fn store_byte(&mut self, addr: u32, byte: u8) -> Result<(), ()> {
        let addr = addr - SDRAM_LOWER_ADDR;
        if addr as usize >= self.mem.len() {
            return Err(());
        }

        unsafe {
            *(self.mem.as_mut_ptr() as *mut u8).add(addr as usize) = byte;
        }
        Ok(())
    }

    fn load_halfword(&self, addr: u32) -> Result<u16, ()> {
        let addr = addr - SDRAM_LOWER_ADDR;
        if addr + 1 >= SDRAM_HIGHER_ADDR {
            return Err(());
        }
        Ok(unsafe { *((self.mem.as_ptr() as *const u8).add(addr as usize) as *const u16) })
    }

    fn store_halfword(&mut self, addr: u32, halfword: u16) -> Result<(), ()> {
        let addr = addr - SDRAM_LOWER_ADDR;
        if addr as usize + 1 >= self.mem.len() {
            return Err(());
        }

        unsafe {
            *((self.mem.as_mut_ptr() as *mut u8).add(addr as usize) as *mut u16) = halfword;
        }
        Ok(())
    }

    fn load_word(&self, addr: u32) -> Result<u32, ()> {
        let addr = addr - SDRAM_LOWER_ADDR;
        if addr + 3 >= SDRAM_HIGHER_ADDR {
            return Err(());
        }
        Ok(unsafe { *((self.mem.as_ptr() as *const u8).add(addr as usize) as *const u32) })
    }

    fn store_word(&mut self, addr: u32, word: u32) -> Result<(), ()> {
        let addr = addr - SDRAM_LOWER_ADDR;
        if addr as usize + 3 >= self.mem.len() {
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

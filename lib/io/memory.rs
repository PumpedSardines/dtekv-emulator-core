use crate::{Data, io};

const SDRAM_SIZE: usize = 0x3ffffff;
const SDRAM_LOWER_ADDR: u32 = 0;
const SDRAM_HIGHER_ADDR: u32 = SDRAM_LOWER_ADDR + SDRAM_SIZE as u32;

#[derive(Clone)]
pub struct SDRam {
    mem: Vec<u8>,
}

impl Default for SDRam {
    fn default() -> Self {
        Self::new()
    }
}

impl SDRam {
    pub fn new() -> SDRam {
        SDRam { mem: vec![0; SDRAM_SIZE + 1] }
    }

    pub fn load_data_at(&mut self, addr: u32, data: Vec<u8>) {
        for (i, byte) in data.iter().enumerate() {
            self.mem[addr as usize + i] = *byte;
        }
    }
}

impl io::Device<()> for SDRam {
    fn addr_range(&self) -> (u32, u32) {
        (SDRAM_LOWER_ADDR, SDRAM_HIGHER_ADDR)
    }

    fn clock(&mut self) {
    }
}

impl io::Interruptable for SDRam {
    fn interrupt(&self) -> Option<u32> {
        None
    }
}

impl Data<()> for SDRam {
    fn load_byte(&self, addr: u32) -> Result<u8, ()> {
        let addr = addr - SDRAM_LOWER_ADDR;
        if addr as usize >= self.mem.len() {
            return Err(());
        }
        Ok(self.mem[addr as usize])
    }

    fn store_byte(&mut self, addr: u32, byte: u8) -> Result<(), ()> {
        let addr = addr - SDRAM_LOWER_ADDR;
        if addr as usize >= self.mem.len() {
            return Err(());
        }
        self.mem[addr as usize] = byte;
        Ok(())
    }
}

impl std::fmt::Debug for SDRam {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Memory {{ ... }}")
    }
}

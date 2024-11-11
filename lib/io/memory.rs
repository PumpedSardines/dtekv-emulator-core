use crate::{Data, io};

const SDRAM_SIZE: usize = 0x3ffffff;
const SDRAM_LOWER_ADDR: u32 = 0;
const SDRAM_HIGHER_ADDR: u32 = SDRAM_SIZE as u32;

#[derive(Clone)]
pub struct Memory {
    mem: Vec<u8>,
}

impl Memory {
    pub fn new() -> Memory {
        Memory { mem: vec![0; SDRAM_SIZE] }
    }

    pub fn load_data_at(&mut self, addr: u32, data: Vec<u8>) {
        for (i, byte) in data.iter().enumerate() {
            self.mem[addr as usize + i] = *byte;
        }
    }
}

impl io::Device<()> for Memory {
    fn bounds(&self) -> Vec<(u32, u32)> {
        vec![(SDRAM_LOWER_ADDR, SDRAM_HIGHER_ADDR)]
    }
}

impl Data<()> for Memory {
    fn load_byte(&self, addr: u32) -> Result<u8, ()> {
        Ok(self.mem[addr as usize])
    }

    fn store_byte(&mut self, addr: u32, byte: u8) -> Result<(), ()> {
        self.mem[addr as usize] = byte;
        Ok(())
    }
}

impl std::fmt::Debug for Memory {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Memory {{ ... }}")
    }
}

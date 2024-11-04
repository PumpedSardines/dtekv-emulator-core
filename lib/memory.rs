use crate::LoadStore;

#[derive(Clone)]
pub struct Memory {
    mem: Vec<u8>,
}

impl Memory {
    /// Returns a new Memory object with a given size all set to 0
    pub fn empty(size: usize) -> Memory {
        Memory { mem: vec![0; size] }
    }

    pub fn load_data_at(&mut self, addr: u32, data: Vec<u8>) {
        for (i, byte) in data.iter().enumerate() {
            self.mem[addr as usize + i] = *byte;
        }
    }
}

impl LoadStore for Memory {
    fn load_byte(&self, addr: u32) -> u8 {
        self.mem[addr as usize]
    }

    fn store_byte(&mut self, addr: u32, byte: u8) {
        self.mem[addr as usize] = byte;
    }
}

impl std::fmt::Debug for Memory {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Memory {{ ... }}")
    }
}

use crate::LoadStore;

#[derive(Clone)]
pub struct HexDisplay {
    pub displays: [u8; 6],
}

impl HexDisplay {
    /// Returns a new Memory object with a given size all set to 0
    pub fn new() -> Self {
        HexDisplay { displays: [0; 6] }
    }

    pub fn get_display(&self, index: u32) -> u8 {
        self.displays[index as usize]
    }

    pub fn set_display(&mut self, index: u32, value: u8) {
        self.displays[index as usize] = value;
    }
}

impl LoadStore for HexDisplay {
    fn load_byte(&self, _addr: u32) -> u8 {
        // ahrd wired to 0
        0
    }

    fn store_byte(&mut self, addr: u32, byte: u8) {
        let lower_addr = addr % 4;
        if lower_addr != 0 {
            // Doesn't matter, hard wired to 0
            return;
        }
        let addr = addr / 16;
        self.set_display(addr, byte);
    }
}

impl std::fmt::Debug for HexDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "HexDisplay {{ ... }}")
    }
}

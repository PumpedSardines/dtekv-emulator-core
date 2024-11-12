use crate::Data;

#[derive(Clone)]
pub struct HexDisplay {
    pub displays: [u8; 6],
}

impl HexDisplay {
    /// Returns a new Memory object with a given size all set to 0
    pub fn new() -> Self {
        HexDisplay { displays: [0; 6] }
    }

    pub fn get(&self, index: u32) -> u8 {
        self.displays[index as usize]
    }

    pub fn set(&mut self, index: u32, value: u8) {
        self.displays[index as usize] = value;
    }
}

impl Default for HexDisplay {
    fn default() -> Self {
        Self::new()
    }
}

impl Data<()> for HexDisplay {
    fn load_byte(&self, _addr: u32) -> Result<u8, ()> {
        // hard wired to 0
        Ok(0)
    }

    fn store_byte(&mut self, addr: u32, byte: u8) -> Result<(), ()> {
        let lower_addr = addr % 4;
        if lower_addr != 0 {
            // Doesn't matter
            return Ok(());
        }
        let addr = addr / 16;
        if addr >= 6 {
            panic!("Invalid hex display address: {}", addr);
        }
        self.set(addr, byte);

        Ok(())
    }
}

impl std::fmt::Debug for HexDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "HexDisplay {{ ... }}")
    }
}

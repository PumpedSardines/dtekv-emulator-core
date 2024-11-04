use std::io::{self, Write};
use crate::LoadStore;

#[derive(Clone)]
pub struct Uart;

impl Uart {
    /// Returns a new Memory object with a given size all set to 0
    pub fn new() -> Self {
        Uart {}
    }
}

impl LoadStore for Uart {
    fn load_byte(&self, addr: u32) -> u8 {
        if addr >= 4 {
            // CTRL signal, always send high, aka ready
            return u8::MAX;
        } else {
            0
        }
    }

    fn store_byte(&mut self, addr: u32, byte: u8) {
        if addr >= 4 {
            return;
        }

        if addr == 0 {
            // Send the byte
            print!("{}", byte as char);
            io::stdout().flush().unwrap();
        }
    }
}

impl std::fmt::Debug for Uart {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Uart {{ ... }}")
    }
}

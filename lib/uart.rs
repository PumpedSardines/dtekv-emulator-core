use crate::LoadStore;
use std::{io::{self, Write}, sync::mpsc::Sender};

#[derive(Clone)]
pub struct Uart {
    tx: Option<Sender<char>>
}

impl Uart {
    /// Returns a new Memory object with a given size all set to 0
    pub fn new() -> Self {
        Uart { tx: None }
    }

    pub fn set_tx(&mut self, tx: Sender<char>) {
        self.tx = Some(tx);
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
            if let Some(tx) = &self.tx {
                tx.send(byte as char).unwrap();
            } else {
                print!("{}", byte as char);
                io::stdout().flush().unwrap();
            }
        }
    }
}

impl std::fmt::Debug for Uart {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Uart {{ ... }}")
    }
}

use crate::Data;
use std::collections::LinkedList;

#[derive(Clone)]
pub struct Uart {
    values: LinkedList<char>,
}

impl Uart {
    pub fn new() -> Self {
        Uart {
            values: LinkedList::new(),
        }
    }

    fn push(&mut self, value: char) {
        self.values.push_back(value);
    }

    fn pop(&mut self) -> Option<char> {
        self.values.pop_front()
    }
}

impl Default for Uart {
    fn default() -> Self {
        Self::new()
    }
}

impl Iterator for Uart {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.pop()
    }
}

impl Data<()> for Uart {
    fn load_byte(&self, addr: u32) -> Result<u8, ()> {
        Ok(if addr >= 4 {
            // CTRL signal, always send high, aka ready
            u8::MAX
        } else {
            0
        })
    }

    fn store_byte(&mut self, addr: u32, byte: u8) -> Result<(), ()> {
        if addr >= 4 {
            return Ok(());
        }

        if addr == 0 {
            self.push(byte as char);
        }

        Ok(())
    }
}

impl std::fmt::Debug for Uart {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Uart {{ ... }}")
    }
}

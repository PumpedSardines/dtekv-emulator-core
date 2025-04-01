use std::collections::LinkedList;

use crate::memory_mapped::MemoryMapped;

use super::Peripheral;

pub const UART_LOWER_ADDR: u32 = 0x04000040;
pub const UART_HIGHER_ADDR: u32 = 0x04000047;

#[derive(Clone)]
pub struct UART {
    values: LinkedList<char>,
}

impl UART {
    pub fn new() -> Self {
        UART {
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

impl Peripheral<()> for UART {}

impl Default for UART {
    fn default() -> Self {
        Self::new()
    }
}

impl Iterator for UART {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.pop()
    }
}

impl MemoryMapped<()> for UART {
    fn load_byte(&self, addr: u32) -> Result<u8, ()> {
        let addr = addr - UART_LOWER_ADDR;
        Ok(if addr >= 4 {
            // CTRL signal, always send high, aka ready
            u8::MAX
        } else {
            0
        })
    }

    fn store_byte(&mut self, addr: u32, byte: u8) -> Result<(), ()> {
        let addr = addr - UART_LOWER_ADDR;

        if addr >= 4 {
            return Ok(());
        }

        if addr == 0 {
            self.push(byte as char);
        }

        Ok(())
    }
}

impl std::fmt::Debug for UART {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Uart {{ ... }}")
    }
}

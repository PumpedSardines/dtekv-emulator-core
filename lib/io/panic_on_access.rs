use crate::{io, Data};

/// An io device that panics if accessed
/// Useful for debugging
#[derive(Clone)]
pub struct PanicOnAccess {}

impl PanicOnAccess {
    /// Returns a new Memory object with a given size all set to 0
    pub fn new() -> Self {
        PanicOnAccess {}
    }
}

impl io::Device<()> for PanicOnAccess {}

impl io::Interruptable for PanicOnAccess {
    fn interrupt(&self) -> Option<u32> {
        None
    }
}

impl Data<()> for PanicOnAccess {
    fn load_byte(&self, addr: u32) -> Result<u8, ()> {
        panic!("PanicOnAccess device accessed at address {:#010x}", addr);
    }

    fn store_byte(&mut self, addr: u32, byte: u8) -> Result<(), ()> {
        panic!("PanicOnAccess device store at address {:#010x}, byte {:#04x}", addr, byte);
    }
}

impl std::fmt::Debug for PanicOnAccess {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "PanicOnAccess {{ ... }}")
    }
}

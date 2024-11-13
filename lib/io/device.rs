use crate::{io, Data};

pub trait Device<T>: io::Interruptable + Data<T> + std::fmt::Debug {
    // Static function to get the address range
    fn addr_range(&self) -> (u32, u32);

    // Clock the device
    fn clock(&mut self) {}
}

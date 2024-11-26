use crate::{io, Data};

pub trait Device<T>: io::Interruptable + Data<T> + std::fmt::Debug {
    // Clock the device
    fn clock(&mut self) {}
}

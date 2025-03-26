use crate::interrupt::InterruptSignal;
use crate::memory_mapped::MemoryMapped;
use std::cell::RefCell;
use std::rc::Rc;

pub trait Peripheral<T>: MemoryMapped<T> {
    /// If an interrupt signal is present, for peripherals that can't generate interrupts this
    /// should simply always return None
    fn poll_interrupt(&self) -> Option<InterruptSignal> {
        None
    }
}

/// Default implementation since it is a common use case
impl<K, T> Peripheral<T> for Rc<RefCell<K>> where K: Peripheral<T> {}

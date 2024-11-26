use crate::{io, Data};
use std::cell::RefCell;
use std::rc::Rc;

pub trait Device<T>: io::Interruptable + Data<T> + std::fmt::Debug {
    // Static function to get the address range
    fn addr_range(&self) -> (u32, u32);

    // Clock the device
    fn clock(&mut self) {}
}

impl<K> io::Interruptable for Rc<RefCell<K>>
where
    K: io::Interruptable
{
    fn interrupt(&self) -> Option<u32> {
        self.borrow_mut().interrupt()
    }
}

impl<K, T> Device<T> for Rc<RefCell<K>>
where
    K: Device<T>
{
    fn addr_range(&self) -> (u32, u32) {
        self.borrow().addr_range()
    }

    fn clock(&mut self) {
        self.borrow_mut().clock();
    }
}

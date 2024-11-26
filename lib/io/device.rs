use crate::{io, Data};
use std::cell::RefCell;
use std::rc::Rc;

pub trait Device<T>: io::Interruptable + Data<T> + std::fmt::Debug {
    // Clock the device
    fn clock(&mut self) {}
}

impl<K, T> Device<T> for Rc<RefCell<K>>
where
    K: Device<T>
{
    fn clock(&mut self) {
        self.borrow_mut().clock();
    }
}

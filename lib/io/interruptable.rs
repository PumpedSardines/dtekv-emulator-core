use crate::io;
use std::cell::RefCell;
use std::rc::Rc;

pub trait Interruptable {
    fn interrupt(&self) -> Option<u32>;
}

impl<K> io::Interruptable for Rc<RefCell<K>>
where
    K: io::Interruptable,
{
    fn interrupt(&self) -> Option<u32> {
        self.borrow_mut().interrupt()
    }
}

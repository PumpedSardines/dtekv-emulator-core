use crate::exception::Exception;
use crate::io;
use std::cell::RefCell;
use std::rc::Rc;

pub trait Interruptable {
    fn interrupt(&self) -> Option<Exception> {
        None
    }
}

impl<K> io::Interruptable for Rc<RefCell<K>>
where
    K: io::Interruptable,
{
    fn interrupt(&self) -> Option<Exception> {
        self.borrow_mut().interrupt()
    }
}

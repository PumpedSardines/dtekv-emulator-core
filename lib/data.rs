use std::cell::RefCell;
use std::rc::Rc;

pub trait Data<T> {
    fn load_byte(&self, addr: u32) -> Result<u8, T>;
    fn store_byte(&mut self, addr: u32, byte: u8) -> Result<(), T>;

    fn load_halfword(&self, addr: u32) -> Result<u16, T> {
        Ok(u16::from_be_bytes([
            self.load_byte(addr + 1)?,
            self.load_byte(addr + 0)?,
        ]))
    }

    fn load_word(&self, addr: u32) -> Result<u32, T> {
        Ok(u32::from_be_bytes([
            self.load_byte(addr + 3)?,
            self.load_byte(addr + 2)?,
            self.load_byte(addr + 1)?,
            self.load_byte(addr + 0)?,
        ]))
    }

    fn store_halfword(&mut self, addr: u32, halfword: u16) -> Result<(), T> {
        let bytes = halfword.to_le_bytes();
        self.store_byte(addr + 0, bytes[0])?;
        self.store_byte(addr + 1, bytes[1])?;
        Ok(())
    }

    fn store_word(&mut self, addr: u32, word: u32) -> Result<(), T> {
        let bytes = word.to_le_bytes();
        self.store_byte(addr + 0, bytes[0])?;
        self.store_byte(addr + 1, bytes[1])?;
        self.store_byte(addr + 2, bytes[2])?;
        self.store_byte(addr + 3, bytes[3])?;
        Ok(())
    }

    fn store_at<K: Into<u8>, R: IntoIterator<Item = K>>(
        &mut self,
        offset: u32,
        bin: R,
    ) -> Result<(), T>
    where
        Self: Sized,
    {
        for (i, byte) in bin.into_iter().enumerate() {
            self.store_byte(offset + i as u32, byte.into())?;
        }

        Ok(())
    }

    fn load_at(&self, offset: u32, size: usize) -> Result<Vec<u8>, T> {
        let mut buf = Vec::with_capacity(size);
        for i in 0..size {
            buf.push(self.load_byte(offset + i as u32)?);
        }

        Ok(buf)
    }
}

impl<K, T> Data<T> for Rc<RefCell<K>>
where
    K: Data<T>,
{
    fn load_byte(&self, addr: u32) -> Result<u8, T> {
        self.borrow().load_byte(addr)
    }

    fn store_byte(&mut self, addr: u32, byte: u8) -> Result<(), T> {
        self.borrow_mut().store_byte(addr, byte)
    }

    fn load_halfword(&self, addr: u32) -> Result<u16, T> {
        self.borrow().load_halfword(addr)
    }

    fn load_word(&self, addr: u32) -> Result<u32, T> {
        self.borrow().load_word(addr)
    }

    fn store_halfword(&mut self, addr: u32, halfword: u16) -> Result<(), T> {
        self.borrow_mut().store_halfword(addr, halfword)
    }

    fn store_word(&mut self, addr: u32, word: u32) -> Result<(), T> {
        self.borrow_mut().store_word(addr, word)
    }
}

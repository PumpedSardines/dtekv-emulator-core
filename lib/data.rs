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
        let bytes = halfword.to_be_bytes();
        self.store_byte(addr + 0, bytes[1])?;
        self.store_byte(addr + 1, bytes[0])?;
        Ok(())
    }

    fn store_word(&mut self, addr: u32, word: u32) -> Result<(), T> {
        let bytes = word.to_be_bytes();
        self.store_byte(addr + 0, bytes[3])?;
        self.store_byte(addr + 1, bytes[2])?;
        self.store_byte(addr + 2, bytes[1])?;
        self.store_byte(addr + 3, bytes[0])?;
        Ok(())
    }

    fn load_at<K: Into<u8>, R: IntoIterator<Item = K>>(
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

    fn read_at(&self, offset: u32, size: usize) -> Result<Vec<u8>, T> {
        let mut buf = Vec::with_capacity(size);
        for i in 0..size {
            buf.push(self.load_byte(offset + i as u32)?);
        }

        Ok(buf)
    }
}

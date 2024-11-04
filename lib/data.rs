pub trait LoadStore {
    fn load_byte(&self, addr: u32) -> u8;
    fn store_byte(&mut self, addr: u32, byte: u8);
}

pub trait Data {
    fn load_halfword(&self, addr: u32) -> u16;
    fn load_word(&self, addr: u32) -> u32;

    fn store_halfword(&mut self, addr: u32, halfword: u16);
    fn store_word(&mut self, addr: u32, word: u32);
}

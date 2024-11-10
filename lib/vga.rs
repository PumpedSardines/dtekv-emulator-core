use crate::LoadStore;
use image::{Rgb, RgbImage};

#[derive(Clone)]
pub struct Vga {
    pub pixels: [u8; 320 * 240],
}

impl Vga {
    /// Returns a new Memory object with a given size all set to 0
    pub fn new() -> Self {
        Vga {
            pixels: [0; 320 * 240],
        }
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> (u8, u8, u8) {
        let pixel = self.pixels[(y * 320 + x) as usize];
        let red = pixel & 0b11100000;
        let green = pixel & 0b00011100;
        let blue = pixel & 0b00000011;

        ((red >> 5) * 32, (green >> 2) * 32, blue * 85)
    }

    pub fn to_rbg_image(&self) -> RgbImage {
        let mut img = RgbImage::new(320, 240);

        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let (r, g, b) = self.get_pixel(x, y);
            *pixel = Rgb([r, g, b]);
        }

        img
    }
}

impl LoadStore for Vga {
    fn load_byte(&self, _addr: u32) -> u8 {
        // Hard wired to 0
        0
    }

    fn store_byte(&mut self, addr: u32, byte: u8) {
        self.pixels[addr as usize] = byte;
    }
}

impl std::fmt::Debug for Vga {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Vga {{ ... }}")
    }
}

use crate::Data;
use image::{ImageFormat, Rgb, RgbImage};
use std::io::Cursor;
use std::sync::atomic::AtomicU8;

pub struct Vga {
    pub pixels: [AtomicU8; 320 * 240],
}

impl Default for Vga {
    fn default() -> Self {
        Self::new()
    }
}

impl Vga {
    /// Returns a new Memory object with a given size all set to 0
    pub fn new() -> Self {
        Vga {
            pixels: [const { AtomicU8::new(0) }; 320 * 240],
        }
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> (u8, u8, u8) {
        let pixel = self.pixels[(y * 320 + x) as usize].load(std::sync::atomic::Ordering::Relaxed);
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

    pub fn to_png(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        let img = self.to_rbg_image();
        img.write_to(&mut Cursor::new(&mut buffer), ImageFormat::Png)
            .unwrap();
        buffer
    }
}

impl Data<()> for Vga {
    fn load_byte(&self, _addr: u32) -> Result<u8, ()> {
        // Hard wired to 0
        Ok(0)
    }

    fn store_byte(&mut self, addr: u32, byte: u8) -> Result<(), ()> {
        self.pixels[addr as usize].store(byte, std::sync::atomic::Ordering::Relaxed);
        Ok(())
    }
}

impl std::fmt::Debug for Vga {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Vga {{ ... }}")
    }
}

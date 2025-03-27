//! VGA Buffer and DMA peripherals along with a generic rendering trait for multiple
//! implementations
//!
//! Minimal working example:
//! ```rust
//! # use dtekv_emulator_core::*;
//! # use dtekv_emulator_core::peripheral::*;
//! #
//! struct TestRenderer {}
//! impl vga::Renderer for TestRenderer {
//!     // implement rendering code here
//!     fn set_pixel(&mut self, index: u32, color: (u8, u8, u8)) { }
//!     fn set_buffer_offset(&mut self, offset: u32) { }
//! }

//! let channel = vga::Channel::new(TestRenderer {});
//! let dma = vga::Dma::new(&channel);
//! let buffer = vga::Buffer::new(&channel);
//! ```
//!
//! In hindsight it might've been easiest to just have the buffer and dma as one single
//! peripheral but oh well `¯\_(ツ)_/¯`

mod buffer;
pub use buffer::*;
mod dma;
pub use dma::*;
mod channel;
pub use channel::*;

pub trait Renderer {
    /// Set's a pixel color at a given index into the buffer, since it's two 320*240 buffers, index
    /// can be between 0 and 153 600
    fn set_pixel(&mut self, index: u32, color: (u8, u8, u8));
    /// Move buffer the vga buffer renders from
    fn set_buffer_offset(&mut self, buffer: u32);
}

fn test() {
    struct TestRenderer {}
    impl Renderer for TestRenderer {
        // implement rendering code here
        fn set_pixel(&mut self, index: u32, color: (u8, u8, u8)) {}
        fn set_buffer_offset(&mut self, offset: u32) {}
    }

    struct Vga<'a> {
        channel: Channel<TestRenderer>,
        objects: Option<VgaObjects<'a>>,
    }

    struct VgaObjects<'a> {
        dma: Dma<'a, TestRenderer>,
        buffer: Buffer<'a, TestRenderer>,
    }

    impl<'a> VgaObjects<'a> {
        pub fn new(channel: &'a Channel<TestRenderer>) -> Self {
             VgaObjects {
                    dma: Dma::new(&channel),
                    buffer: Buffer::new(&channel),
            }
        }
    }

    impl<'a> Vga<'a> {
        pub fn new() -> Self {
            let channel = Channel::new(TestRenderer {});

            let mut vga = Vga {
                channel,
                objects: None
            };

            vga.create_dma_buffer();

            vga
        }

        fn create_dma_buffer(&'a mut self) {
            self.objects = Some(VgaObjects::new(&self.channel));
        }
    }
}

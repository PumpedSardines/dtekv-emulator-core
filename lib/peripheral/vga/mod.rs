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

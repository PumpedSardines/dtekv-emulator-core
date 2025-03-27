use std::cell::UnsafeCell;

use super::Renderer;

struct ChannelData<T: Renderer> {
    is_swapping: bool,
    renderer: T,
}

/// Abstraction for the connection between the DMA and the Buffer peripherals
pub struct Channel<T: Renderer> {
    data: UnsafeCell<ChannelData<T>>,
}

impl<T: Renderer> Channel<T> {
    pub fn new(renderer: T) -> Self {
        Channel {
            data: UnsafeCell::new(ChannelData {
                renderer,
                is_swapping: false,
            }),
        }
    }

    /// Get the swapping state of the channel
    pub fn is_swapping(&self) -> bool {
        unsafe { (*self.data.get()).is_swapping }
    }

    pub fn start_swap(&self) {
        unsafe { (*self.data.get()).is_swapping = true }
    }

    pub fn finish_swap(&self) {
        unsafe { (*self.data.get()).is_swapping = false }
    }

    pub fn set_pixel(&self, index: u32, color: (u8, u8, u8)) {
        unsafe { (*self.data.get()).renderer.set_pixel(index, color) }
    }

    pub fn set_buffer_offset(&self, buffer: u32) {
        unsafe { (*self.data.get()).renderer.set_buffer_offset(buffer) }
    }
}

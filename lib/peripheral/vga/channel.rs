#[cfg(debug_assertions)]
use std::cell::RefCell;
#[cfg(not(debug_assertions))]
use std::cell::UnsafeCell;

use super::Renderer;

// Use `cfg` here to find UB during dev builds

struct ChannelData<T: Renderer> {
    is_swapping: bool,
    renderer: T,
}

/// Abstraction for the connection between the DMA and the Buffer peripherals
pub struct Channel<T: Renderer> {
    #[cfg(not(debug_assertions))]
    data: UnsafeCell<ChannelData<T>>,
    #[cfg(debug_assertions)]
    data: RefCell<ChannelData<T>>,
}

macro_rules! get_mut {
    ($self:expr) => {{
        #[cfg(not(debug_assertions))]
        unsafe {
            &mut *$self.data.get()
        }
        #[cfg(debug_assertions)]
        &mut $self.data.borrow_mut()
    }};
}

impl<T: Renderer> Channel<T> {
    pub fn new(renderer: T) -> Self {
        Channel {
            #[cfg(not(debug_assertions))]
            data: UnsafeCell::new(ChannelData {
                renderer,
                is_swapping: false,
            }),
            #[cfg(debug_assertions)]
            data: RefCell::new(ChannelData {
                renderer,
                is_swapping: false,
            }),
        }
    }

    pub fn with_renderer_borrow<K>(&self, f: impl FnOnce(&T) -> K) -> K {
        let data = get_mut!(self);
        f(&data.renderer)
    }

    pub fn with_renderer_borrow_mut<K>(&self, f: impl FnOnce(&mut T) -> K) -> K {
        let data = get_mut!(self);
        f(&mut data.renderer)
    }

    // SAFETY: All of these functions only have a mutable borrow while this function is called.
    // Therefore we can't have multiple mutable borrows at the same time since the mutable borrow
    // doesn't escape the function.

    /// Get the swapping state of the channel
    pub fn is_swapping(&self) -> bool {
        let data = get_mut!(self);
        data.is_swapping
    }

    pub fn start_swap(&self) {
        let data = get_mut!(self);
        data.is_swapping = true;
    }

    pub fn finish_swap(&self) {
        let data = get_mut!(self);
        data.is_swapping = false;
    }

    pub fn set_pixel(&self, index: u32, color: (u8, u8, u8)) {
        let data = get_mut!(self);
        data.renderer.set_pixel(index, color);
    }

    pub fn set_buffer_offset(&self, buffer: u32) {
        let data = get_mut!(self);
        data.renderer.set_buffer_offset(buffer);
    }
}

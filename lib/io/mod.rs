//! Default implementation of different io devices
//!
//! You can make your own implementation if you need to, especially the vga buffer, timer and uart
//! can be pretty platform specific

mod device;
pub use device::Device;

mod panic_on_access;
pub use panic_on_access::PanicOnAccess;

mod bus;
pub use bus::Bus;

mod interruptable;
pub use interruptable::Interruptable;

mod button;
pub use button::*;

mod hex_display;
pub use hex_display::*;

mod led_strip;
pub use led_strip::*;

mod sdram;
pub use sdram::*;

mod switch;
pub use switch::*;

mod timer;
pub use timer::*;

mod uart;
pub use uart::*;

mod vga_buffer;
pub use vga_buffer::*;

mod vga_dma;
pub use vga_dma::*;

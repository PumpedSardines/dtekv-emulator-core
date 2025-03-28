//! Peripherals

mod peripheral;
pub use peripheral::Peripheral;

mod bus;
pub use bus::Bus;

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

pub mod vga;

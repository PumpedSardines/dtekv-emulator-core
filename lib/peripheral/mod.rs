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

fn test() {
    struct TestRenderer {}
    impl vga::Renderer for TestRenderer {
        fn set_pixel(&mut self, _0: u32, _1: (u8, u8, u8)) { }
        fn set_buffer_offset(&mut self, _: u32) { }
    }

    let channel = vga::Channel::new(TestRenderer {});
    let dma = vga::Dma::new(&channel);
    let buffer = vga::Buffer::new(&channel);

}

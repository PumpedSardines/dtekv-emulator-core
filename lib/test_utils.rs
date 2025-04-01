//! Some utils that are used throughout testing

use std::{cell::RefCell, rc::Rc};

use crate::{
    cpu::Cpu,
    memory_mapped::MemoryMapped,
    peripheral::{self, Peripheral},
};

pub struct TestCpuData {
    pub cpu: Cpu<peripheral::Bus>,
    pub sdram: Rc<RefCell<peripheral::SDRam>>,
    pub button: Rc<RefCell<peripheral::Button>>,
    pub hex_display: Rc<RefCell<peripheral::HexDisplay>>,
    pub led_strip: Rc<RefCell<peripheral::LEDStrip>>,
    pub switch: Rc<RefCell<peripheral::Switch>>,
    pub uart: Rc<RefCell<peripheral::UART>>,
}

macro_rules! add_bus {
    ($bus:expr, $device:expr, $range:expr) => {{
        let device = Rc::new(RefCell::new($device));
        $bus.attach_device($range, Box::new(device.clone()));
        device
    }};
}

/// Generates a new CPU with a panic on access device
pub fn new_panic_io_cpu() -> Cpu<PanicOnAccess> {
    Cpu::new_with_bus(PanicOnAccess::new())
}

pub fn new_io_cpu() -> TestCpuData {
    let mut bus = peripheral::Bus::new();
    let sdram = add_bus!(
        bus,
        peripheral::SDRam::new(),
        (peripheral::SDRAM_LOWER_ADDR, peripheral::SDRAM_HIGHER_ADDR)
    );
    let button = add_bus!(
        bus,
        peripheral::Button::new(),
        (
            peripheral::BUTTON_LOWER_ADDR,
            peripheral::BUTTON_HIGHER_ADDR
        )
    );
    let hex_display = add_bus!(
        bus,
        peripheral::HexDisplay::new(),
        (
            peripheral::HEX_DISPLAY_LOWER_ADDR,
            peripheral::HEX_DISPLAY_HIGHER_ADDR
        )
    );
    let led_strip = add_bus!(
        bus,
        peripheral::LEDStrip::new(),
        (
            peripheral::LED_STRIP_LOWER_ADDR,
            peripheral::LED_STRIP_HIGHER_ADDR
        )
    );
    let switch = add_bus!(
        bus,
        peripheral::Switch::new(),
        (
            peripheral::SWITCH_LOWER_ADDR,
            peripheral::SWITCH_HIGHER_ADDR
        )
    );
    let uart = add_bus!(
        bus,
        peripheral::UART::new(),
        (peripheral::UART_LOWER_ADDR, peripheral::UART_HIGHER_ADDR)
    );

    // Final, if we're out of bounds we panic
    add_bus!(bus, PanicOnAccess::new(), (0x0, 0xFFFFFFFF));

    TestCpuData {
        cpu: Cpu::new_with_bus(bus),
        sdram,
        button,
        hex_display,
        led_strip,
        switch,
        uart,
    }
}

#[derive(Clone)]
pub struct PanicOnAccess {}

impl PanicOnAccess {
    /// Returns a new Memory object with a given size all set to 0
    pub fn new() -> Self {
        PanicOnAccess {}
    }
}

impl Peripheral<()> for PanicOnAccess {}

impl MemoryMapped<()> for PanicOnAccess {
    fn load_byte(&self, addr: u32) -> Result<u8, ()> {
        panic!("PanicOnAccess device accessed at address {:#010x}", addr);
    }

    fn store_byte(&mut self, addr: u32, byte: u8) -> Result<(), ()> {
        panic!(
            "PanicOnAccess device store at address {:#010x}, byte {:#04x}",
            addr, byte
        );
    }
}

impl std::fmt::Debug for PanicOnAccess {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "PanicOnAccess {{ ... }}")
    }
}

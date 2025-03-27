//! # DTEK-V Emulator Core
//!
//! The core module of the emulator. This rust package is a general purpose DTEK-V emulator that
//! can have any frontend. The emulator is designed to be as modular as possible
//!
//! This package is centered around the Cpu struct and the IO devices. The Cpu contains logic about
//! running the emulator, updating the registers and interrupts. The Cpu has a bus field that connects it
//! to memory. The bus can be any type that implements the Device trait, for simple use cases of
//! this emulator you might only need a memory block
//!
//! # Example
//! Minimal runnable example using the default bus implementation with only a SDRAM device. You
//! should probably not do this if you want to create your own frontend. The default bus is pretty
//! slow, you should implement your own bus for your specific needs.
//!
//! ```rust,no_run
//! # use dtekv_emulator_core::*;
//! # use dtekv_emulator_core::peripheral::*;
//! # use std::fs::File;
//! # use std::io::{BufReader, Read};
//! # use dtekv_emulator_core::memory_mapped::*;
//! #
//! let mut bus = peripheral::Bus::new();
//! // Use RC if you want to access io device after attaching
//! let mut sdram = peripheral::SDRam::new();
//! bus.attach_device(
//!     (peripheral::SDRAM_LOWER_ADDR, peripheral::SDRAM_HIGHER_ADDR),
//!     Box::new(sdram)
//! );
//!
//! let mut cpu = cpu::Cpu::new_with_bus(bus);
//!
//! let file = File::open("path/to/binary-file").unwrap();
//! let bytes = BufReader::new(file).bytes().map(Result::unwrap);
//! cpu.store_at(0, bytes).unwrap();
//!
//! loop {
//!     cpu.clock();
//!     if let Some(interrupt) = cpu.bus.poll_interrupt() {
//!         cpu.handle_interrupt(interrupt);
//!     }
//! }
//!
//! ```

pub mod cpu;
pub mod csr;
pub mod register;

pub mod interrupt;
pub mod memory_mapped;

pub mod instruction;

pub mod peripheral;

#[cfg(feature = "debug-console")]
pub mod debug_console;

pub(crate) mod utils;

#[cfg(test)]
pub mod test_utils;

//! The core module of the emulator. This rust package is a general purpose DTEK-V emulator that
//! can have any frontend. The emulator is designed to be as modular as possible
//!
//! This package is centered around the Cpu struct and the IO devices. The Cpu contains logic about
//! running the emulator, updating the registers and interrupts. The Cpu has a bus field that connects it
//! to memory. The bus can be any type that implements the Device trait, for simple use cases of
//! this emulator you might only need a memory block
//!
//! IMPORTANT: After creating the Cpu you should access memory through the Cpu's implementation of
//! Data. This is because the Cpu does some extra caching logic that speeds up the emulator.
//! Otherwise the cache might get out of sync with the memory

pub mod exception;
pub mod instruction;

pub mod io;
pub mod cpu;

#[cfg(feature = "debug-console")]
pub mod debug_console;

pub(crate) mod utils;

#[cfg(test)]
pub mod test_utils;

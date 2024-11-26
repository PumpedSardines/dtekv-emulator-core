//! The core module of the emulator. This rust package is a general purpose DTEK-V emulator that
//! can have any frontend. The emulator is designed to be as modular as possible

mod data;
pub use data::Data;

pub mod exception;
pub mod instruction;

pub mod io;
pub mod cpu;

pub(crate) mod utils;

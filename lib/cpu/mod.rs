//! The code for the Cpu, it runs code!

mod cpu;
pub use cpu::{Cpu, CLOCK_FEQ};

mod csr;
pub use csr::*;

mod regs;
pub use regs::Regs;

mod instructions;

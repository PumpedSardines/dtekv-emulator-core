//! The code for the Cpu, it runs code!

mod cpu;
pub use cpu::{Cpu, CLOCK_FEQ};

pub mod csr;
pub use csr::CSR;

mod regs;
pub use regs::Regs;

mod instructions;

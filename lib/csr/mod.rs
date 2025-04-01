//! Control and Status Registers (CSRs) for the RISC-V architecture.

mod csr;
pub use csr::Csr;

mod csr_block;
pub use csr_block::CsrBlock;

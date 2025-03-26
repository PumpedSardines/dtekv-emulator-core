//! RISC-V instruction parsing

mod instruction;
pub use instruction::Instruction;

mod newtype;
pub use newtype::*;

/// The RISC-V instruction types
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum InstructionType {
    R,
    I,
    S,
    B,
    U,
    J,
}

mod parse;

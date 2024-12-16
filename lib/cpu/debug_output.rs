use std::collections::LinkedList;

#[derive(Debug)]
pub enum DebugOutputLine {
    /// When a CSR is accessed that is not used anywhere in the emulator
    AccessUselessCsr { csr: u32, instr_addr: u32 },
    /// When an instruction is not implemented
    InstructionNotImplemented { instr: &'static str, instr_addr: u32 },
    /// When an division by zero occurs
    DivisionByZero { instr_addr: u32 },
    /// When an remainder by zero occurs
    RemainderByZero { instr_addr: u32 },
    /// When an illegal instruction is executed
    IllegalInstruction { instr: u32, instr_addr: u32 },
    /// When an instruction is executed that is not aligned to 4 bytes
    InstructionMisaligned { instr_addr: u32 },
    /// When a load instruction is out of bounds
    LoadOutOfBounds { addr: u32, instr_addr: u32 },
    /// When a store instruction is out of bounds
    StoreOutOfBounds { addr: u32, instr_addr: u32 },
}

pub struct DebugOutput {
    pub lines: LinkedList<DebugOutputLine>,
}

impl DebugOutput {
    pub fn new() -> Self {
        return DebugOutput {
            lines: LinkedList::new(),
        };
    }

    pub fn push(&mut self, line: DebugOutputLine) {
        self.lines.push_back(line);
    }

    pub fn pop(&mut self) -> Option<DebugOutputLine> {
        return self.lines.pop_front();
    }

    pub fn is_empty(&self) -> bool {
        return self.lines.is_empty();
    }
}

impl std::fmt::Debug for DebugOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "DebugOutput {{ ... }}")
    }
}

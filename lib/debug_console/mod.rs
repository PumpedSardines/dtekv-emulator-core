//! Debug Console, stores warnings and errors that might occur during the execution of the emulator

use std::collections::LinkedList;

use crate::csr::Csr;

#[derive(Debug)]
pub enum Entry {
    Warning(Warning),
}

impl Entry {}

impl Into<Entry> for Warning {
    fn into(self) -> Entry {
        return Entry::Warning(self);
    }
}

#[derive(Debug)]
pub enum Warning {
    /// When a CSR is accessed that is not used anywhere in the emulator
    AccessUselessCsr { csr: Csr, instr_addr: u32 },
    /// When an instruction is not implemented
    InstructionNotImplemented {
        instr: &'static str,
        instr_addr: u32,
    },
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

pub struct DebugConsole {
    pub lines: LinkedList<Entry>,
}

impl DebugConsole {
    pub fn new() -> Self {
        return Self {
            lines: LinkedList::new(),
        };
    }

    pub fn push(&mut self, line: Entry) {
        self.lines.push_back(line);
    }

    pub fn pop(&mut self) -> Option<Entry> {
        return self.lines.pop_front();
    }

    pub fn is_empty(&self) -> bool {
        return self.lines.is_empty();
    }

    pub fn access_useless_csr(&mut self, csr: Csr, instr_addr: u32) {
        self.push(Warning::AccessUselessCsr { csr, instr_addr }.into());
    }
    pub fn illegal_instruction(&mut self, instr: u32, instr_addr: u32) {
        self.push(Warning::IllegalInstruction { instr, instr_addr }.into());
    }

    pub fn instruction_not_implemented(&mut self, instr: &'static str, instr_addr: u32) {
        self.push(Warning::InstructionNotImplemented { instr, instr_addr }.into());
    }

    pub fn division_by_zero(&mut self, instr_addr: u32) {
        self.push(Warning::DivisionByZero { instr_addr }.into());
    }

    pub fn remainder_by_zero(&mut self, instr_addr: u32) {
        self.push(Warning::RemainderByZero { instr_addr }.into());
    }

    pub fn instruction_misaligned(&mut self, instr_addr: u32) {
        self.push(Warning::InstructionMisaligned { instr_addr }.into());
    }

    pub fn load_out_of_bounds(&mut self, addr: u32, instr_addr: u32) {
        self.push(Warning::LoadOutOfBounds { addr, instr_addr }.into());
    }

    pub fn store_out_of_bounds(&mut self, addr: u32, instr_addr: u32) {
        self.push(Warning::StoreOutOfBounds { addr, instr_addr }.into());
    }
}

impl std::fmt::Debug for DebugConsole {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "DebugOutput {{ ... }}")
    }
}

//! Debug Console, stores warnings and errors that might occur during the execution of the emulator

use std::collections::LinkedList;

use crate::csr::Csr;

#[derive(Debug)]
pub enum Entry {
    Warning(Warning),
    Error(Error),
}

impl Entry {}

impl Into<Entry> for Warning {
    fn into(self) -> Entry {
        return Entry::Warning(self);
    }
}

impl Into<Entry> for Error {
    fn into(self) -> Entry {
        return Entry::Error(self);
    }
}

#[derive(Debug)]
pub enum Warning {
    /// When a CSR is accessed that is not used anywhere in the emulator
    AccessUselessCsr { csr: Csr, instr_addr: u32 },
}

#[derive(Debug)]
pub enum Error {
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

    pub(crate) fn access_useless_csr(&mut self, csr: Csr, instr_addr: u32) {
        self.push(Warning::AccessUselessCsr { csr, instr_addr }.into());
    }
    pub(crate) fn illegal_instruction(&mut self, instr: u32, instr_addr: u32) {
        self.push(Error::IllegalInstruction { instr, instr_addr }.into());
    }

    pub(crate) fn instruction_not_implemented(&mut self, instr: &'static str, instr_addr: u32) {
        self.push(Error::InstructionNotImplemented { instr, instr_addr }.into());
    }

    pub(crate) fn division_by_zero(&mut self, instr_addr: u32) {
        self.push(Error::DivisionByZero { instr_addr }.into());
    }

    pub(crate) fn remainder_by_zero(&mut self, instr_addr: u32) {
        self.push(Error::RemainderByZero { instr_addr }.into());
    }

    pub(crate) fn instruction_misaligned(&mut self, instr_addr: u32) {
        self.push(Error::InstructionMisaligned { instr_addr }.into());
    }

    pub(crate) fn load_out_of_bounds(&mut self, addr: u32, instr_addr: u32) {
        self.push(Error::LoadOutOfBounds { addr, instr_addr }.into());
    }

    pub(crate) fn store_out_of_bounds(&mut self, addr: u32, instr_addr: u32) {
        self.push(Error::StoreOutOfBounds { addr, instr_addr }.into());
    }
}

impl std::fmt::Debug for DebugConsole {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "DebugOutput {{ ... }}")
    }
}

//! RISC-V instructions parsing and representation

// This module is quite ugly, with a lot of boilerplate and reimplementation. I think this is the
// best way to accomplish this functionality, and in a way the risc-v instructions are just a long
// list of hardcoded values.

use crate::{csr::Csr, register::Register};

use super::{BTypeImm, ITypeImm, JTypeImm, STypeImm, ShamtImm, UTypeImm};

/// The RISC-V instructions that are implemented
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Instruction {
    LUI {
        rd: Register,
        imm: UTypeImm,
    },
    AUIPC {
        rd: Register,
        imm: UTypeImm,
    },
    JAL {
        rd: Register,
        imm: JTypeImm,
    },
    JALR {
        rd: Register,
        rs1: Register,
        imm: ITypeImm,
    },
    BEQ {
        rs1: Register,
        rs2: Register,
        imm: BTypeImm,
    },
    BNE {
        rs1: Register,
        rs2: Register,
        imm: BTypeImm,
    },
    BLT {
        rs1: Register,
        rs2: Register,
        imm: BTypeImm,
    },
    BGE {
        rs1: Register,
        rs2: Register,
        imm: BTypeImm,
    },
    BLTU {
        rs1: Register,
        rs2: Register,
        imm: BTypeImm,
    },
    BGEU {
        rs1: Register,
        rs2: Register,
        imm: BTypeImm,
    },
    LB {
        rd: Register,
        rs1: Register,
        imm: ITypeImm
    },
    LH {
        rd: Register,
        rs1: Register,
        imm: ITypeImm,
    },
    LW {
        rd: Register,
        rs1: Register,
        imm: ITypeImm
    },
    LBU {
        rd: Register,
        rs1: Register,
        imm: ITypeImm,
    },
    LHU {
        rd: Register,
        rs1: Register,
        imm: ITypeImm,
    },
    SB {
        rs1: Register,
        rs2: Register,
        imm: STypeImm,
    },
    SH {
        rs1: Register,
        rs2: Register,
        imm: STypeImm,
    },
    SW {
        rs1: Register,
        rs2: Register,
        imm: STypeImm,
    },
    ADDI {
        rd: Register,
        rs1: Register,
        imm: ITypeImm,
    },
    SLTI {
        rd: Register,
        rs1: Register,
        imm: ITypeImm,
    },
    SLTIU {
        rd: Register,
        rs1: Register,
        imm: ITypeImm,
    },
    XORI {
        rd: Register,
        rs1: Register,
        imm: ITypeImm,
    },
    ORI {
        rd: Register,
        rs1: Register,
        imm: ITypeImm,
    },
    ANDI {
        rd: Register,
        rs1: Register,
        imm: ITypeImm,
    },
    SLLI {
        rd: Register,
        rs1: Register,
        imm: ShamtImm,
    },
    SRLI {
        rd: Register,
        rs1: Register,
        imm: ShamtImm,
    },
    SRAI {
        rd: Register,
        rs1: Register,
        imm: ShamtImm,
    },
    ADD {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    SUB {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    SLL {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    SLT {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    SLTU {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    XOR {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    SRL {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    SRA {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    OR {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    AND {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    CSRRW {
        rd: Register,
        rs1: Register,
        csr: Csr,
    },
    CSRRS {
        rd: Register,
        rs1: Register,
        csr: Csr,
    },
    CSRRC {
        rd: Register,
        rs1: Register,
        csr: Csr,
    },
    CSRRWI {
        imm: u32,
        rd: Register,
        csr: Csr,
    },
    CSRRSI {
        imm: u32,
        rd: Register,
        csr: Csr,
    },
    CSRRCI {
        imm: u32,
        rd: Register,
        csr: Csr,
    },
    MRET,
    ECALL,
    MUL {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    MULH {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    MULHSU {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    MULHU {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    DIV {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    DIVU {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    REM {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    REMU {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
}

impl TryFrom<u32> for Instruction {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        super::parse::parse(value)
    }
}

use crate::{csr::Csr, register::Register};

use super::{BTypeImm, ITypeImm, Instruction, JTypeImm, STypeImm, ShamtImm, UTypeImm};

const LUI: u8 = 0b0110111;
const AUIPC: u8 = 0b0010111;
const JAL: u8 = 0b1101111;
const JALR: u8 = 0b1100111;
const BRANCH: u8 = 0b1100011;
const LOAD: u8 = 0b0000011;
const STORE: u8 = 0b0100011;
const OP_IMM: u8 = 0b0010011;
const OP: u8 = 0b0110011;
const SYSTEM: u8 = 0b1110011;

const BEQ: u8 = 0b000;
const BNE: u8 = 0b001;
const BLT: u8 = 0b100;
const BGE: u8 = 0b101;
const BLTU: u8 = 0b110;
const BGEU: u8 = 0b111;
const LB: u8 = 0b000;
const LH: u8 = 0b001;
const LW: u8 = 0b010;
const LBU: u8 = 0b100;
const LHU: u8 = 0b101;
const SB: u8 = 0b000;
const SH: u8 = 0b001;
const SW: u8 = 0b010;
const ADDI: u8 = 0b000;
const SLTI: u8 = 0b010;
const SLTIU: u8 = 0b011;
const XORI: u8 = 0b100;
const ORI: u8 = 0b110;
const ANDI: u8 = 0b111;
const SLLI: u8 = 0b001;
const SRLI_SRAI: u8 = 0b101;
const ADD_SUB: u8 = 0b000;
const SLL: u8 = 0b001;
const SLT: u8 = 0b010;
const SLTU: u8 = 0b011;
const XOR: u8 = 0b100;
const SRL_SRA: u8 = 0b101;
const OR: u8 = 0b110;
const AND: u8 = 0b111;
const MRET_SRET_ECALL: u8 = 0b000;
const CSRRW: u8 = 0b001;
const CSRRS: u8 = 0b010;
const CSRRC: u8 = 0b011;
const CSRRWI: u8 = 0b101;
const CSRRSI: u8 = 0b110;
const CSRRCI: u8 = 0b111;
const MUL: u8 = 0b000;
const MULH: u8 = 0b001;
const MULHSU: u8 = 0b010;
const MULHU: u8 = 0b011;
const DIV: u8 = 0b100;
const DIVU: u8 = 0b101;
const REM: u8 = 0b110;
const REMU: u8 = 0b111;

const MRET: u32 = 0x30200073;
const ECALL: u32 = 0x00000073;

const FUNCT7_SLLI: u8 = 0b0000000;
const FUNCT7_SRLI: u8 = 0b0000000;
const FUNCT7_SRAI: u8 = 0b0100000;

const FUNCT7_ADD: u8 = 0b0000000;
const FUNCT7_SUB: u8 = 0b0100000;
const FUNCT7_SLL: u8 = 0b0000000;
const FUNCT7_SLT: u8 = 0b0000000;
const FUNCT7_SLTU: u8 = 0b0000000;
const FUNCT7_XOR: u8 = 0b0000000;
const FUNCT7_SRL: u8 = 0b0000000;
const FUNCT7_SRA: u8 = 0b0100000;
const FUNCT7_OR: u8 = 0b0000000;
const FUNCT7_AND: u8 = 0b0000000;
const FUNCT7_M_EXT: u8 = 0b0000001;

pub fn parse(raw: u32) -> Result<Instruction, ()> {
    let opcode = opcode(raw);
    let rd = rd(raw);
    let rs1 = rs1(raw);
    let rs2 = rs2(raw);
    let funct3 = funct3(raw);
    let funct7 = funct7(raw);

    match opcode {
        LUI => Ok(Instruction::LUI {
            rd,
            imm: UTypeImm::from_instr(raw),
        }),
        AUIPC => Ok(Instruction::AUIPC {
            rd,
            imm: UTypeImm::from_instr(raw),
        }),
        JAL => Ok(Instruction::JAL {
            rd,
            imm: JTypeImm::from_instr(raw),
        }),
        JALR => Ok(Instruction::JALR {
            rd,
            rs1,
            imm: ITypeImm::from_instr(raw),
        }),
        BRANCH => match funct3 {
            BEQ => Ok(Instruction::BEQ {
                rs1,
                rs2,
                imm: BTypeImm::from_instr(raw),
            }),
            BNE => Ok(Instruction::BNE {
                rs1,
                rs2,
                imm: BTypeImm::from_instr(raw),
            }),
            BLT => Ok(Instruction::BLT {
                rs1,
                rs2,
                imm: BTypeImm::from_instr(raw),
            }),
            BGE => Ok(Instruction::BGE {
                rs1,
                rs2,
                imm: BTypeImm::from_instr(raw),
            }),
            BLTU => Ok(Instruction::BLTU {
                rs1,
                rs2,
                imm: BTypeImm::from_instr(raw),
            }),
            BGEU => Ok(Instruction::BGEU {
                rs1,
                rs2,
                imm: BTypeImm::from_instr(raw),
            }),
            _ => Err(()),
        },
        LOAD => match funct3 {
            LB => Ok(Instruction::LB {
                rd,
                rs1,
                imm: ITypeImm::from_instr(raw),
            }),
            LH => Ok(Instruction::LH {
                rd,
                rs1,
                imm: ITypeImm::from_instr(raw),
            }),
            LW => Ok(Instruction::LW {
                rd,
                rs1,
                imm: ITypeImm::from_instr(raw),
            }),
            LBU => Ok(Instruction::LBU {
                rd,
                rs1,
                imm: ITypeImm::from_instr(raw),
            }),
            LHU => Ok(Instruction::LHU {
                rd,
                rs1,
                imm: ITypeImm::from_instr(raw),
            }),
            _ => Err(()),
        },
        STORE => match funct3 {
            SB => Ok(Instruction::SB {
                rs1,
                rs2,
                imm: STypeImm::from_instr(raw),
            }),
            SH => Ok(Instruction::SH {
                rs1,
                rs2,
                imm: STypeImm::from_instr(raw),
            }),
            SW => Ok(Instruction::SW {
                rs1,
                rs2,
                imm: STypeImm::from_instr(raw),
            }),
            _ => Err(()),
        },
        OP_IMM => match (funct3, funct7) {
            (SLLI, FUNCT7_SLLI) => Ok(Instruction::SLLI {
                rd,
                rs1,
                imm: ShamtImm::from_instr(raw),
            }),
            (SRLI_SRAI, FUNCT7_SRLI) => Ok(Instruction::SRLI {
                rd,
                rs1,
                imm: ShamtImm::from_instr(raw),
            }),
            (SRLI_SRAI, FUNCT7_SRAI) => Ok(Instruction::SRAI {
                rd,
                rs1,
                imm: ShamtImm::from_instr(raw),
            }),
            (ADDI, _) => Ok(Instruction::ADDI {
                rd,
                rs1,
                imm: ITypeImm::from_instr(raw),
            }),
            (SLTI, _) => Ok(Instruction::SLTI {
                rd,
                rs1,
                imm: ITypeImm::from_instr(raw),
            }),
            (SLTIU, _) => Ok(Instruction::SLTIU {
                rd,
                rs1,
                imm: ITypeImm::from_instr(raw),
            }),
            (XORI, _) => Ok(Instruction::XORI {
                rd,
                rs1,
                imm: ITypeImm::from_instr(raw),
            }),
            (ORI, _) => Ok(Instruction::ORI {
                rd,
                rs1,
                imm: ITypeImm::from_instr(raw),
            }),
            (ANDI, _) => Ok(Instruction::ANDI {
                rd,
                rs1,
                imm: ITypeImm::from_instr(raw),
            }),
            _ => Err(()),
        },
        OP => match (funct3, funct7) {
            (ADD_SUB, FUNCT7_ADD) => Ok(Instruction::ADD { rd, rs1, rs2 }),
            (ADD_SUB, FUNCT7_SUB) => Ok(Instruction::SUB { rd, rs1, rs2 }),
            (SLL, FUNCT7_SLL) => Ok(Instruction::SLL { rd, rs1, rs2 }),
            (SLT, FUNCT7_SLT) => Ok(Instruction::SLT { rd, rs1, rs2 }),
            (SLTU, FUNCT7_SLTU) => Ok(Instruction::SLTU { rd, rs1, rs2 }),
            (XOR, FUNCT7_XOR) => Ok(Instruction::XOR { rd, rs1, rs2 }),
            (SRL_SRA, FUNCT7_SRL) => Ok(Instruction::SRL { rd, rs1, rs2 }),
            (SRL_SRA, FUNCT7_SRA) => Ok(Instruction::SRA { rd, rs1, rs2 }),
            (OR, FUNCT7_OR) => Ok(Instruction::OR { rd, rs1, rs2 }),
            (AND, FUNCT7_AND) => Ok(Instruction::AND { rd, rs1, rs2 }),
            (MUL, FUNCT7_M_EXT) => Ok(Instruction::MUL { rd, rs1, rs2 }),
            (MULH, FUNCT7_M_EXT) => Ok(Instruction::MULH { rd, rs1, rs2 }),
            (MULHSU, FUNCT7_M_EXT) => Ok(Instruction::MULHSU { rd, rs1, rs2 }),
            (MULHU, FUNCT7_M_EXT) => Ok(Instruction::MULHU { rd, rs1, rs2 }),
            (DIV, FUNCT7_M_EXT) => Ok(Instruction::DIV { rd, rs1, rs2 }),
            (DIVU, FUNCT7_M_EXT) => Ok(Instruction::DIVU { rd, rs1, rs2 }),
            (REM, FUNCT7_M_EXT) => Ok(Instruction::REM { rd, rs1, rs2 }),
            (REMU, FUNCT7_M_EXT) => Ok(Instruction::REMU { rd, rs1, rs2 }),
            _ => Err(()),
        },
        SYSTEM => match funct3 {
            MRET_SRET_ECALL => match raw {
                MRET => Ok(Instruction::MRET),
                ECALL => Ok(Instruction::ECALL),
                _ => Err(()),
            },
            // We can call expect here because we and with 0xFFF, so the value is always valid
            CSRRW => Ok(Instruction::CSRRW {
                rd,
                rs1,
                csr: Csr::new(get_itype_imm(raw)).expect("Invalid CSR"),
            }),
            CSRRS => Ok(Instruction::CSRRS {
                rd,
                rs1,
                csr: Csr::new(get_itype_imm(raw)).expect("Invalid CSR"),
            }),
            CSRRC => Ok(Instruction::CSRRC {
                rd,
                rs1,
                csr: Csr::new(get_itype_imm(raw)).expect("Invalid CSR"),
            }),
            CSRRWI => Ok(Instruction::CSRRWI {
                imm: rs1.into(),
                rd,
                csr: Csr::new(get_itype_imm(raw)).expect("Invalid CSR"),
            }),
            CSRRSI => Ok(Instruction::CSRRSI {
                imm: rs1.into(),
                rd,
                csr: Csr::new(get_itype_imm(raw)).expect("Invalid CSR"),
            }),
            CSRRCI => Ok(Instruction::CSRRCI {
                imm: rs1.into(),
                rd,
                csr: Csr::new(get_itype_imm(raw)).expect("Invalid CSR"),
            }),
            _ => Err(()),
        },
        _ => Err(()),
    }
}

fn opcode(v: u32) -> u8 {
    (v & 0x7F) as u8
}

fn rs1(v: u32) -> Register {
    Register::new((v >> 15) & 0x1F).expect("Passed something other than a value between 0 and 31")
}

fn rs2(v: u32) -> Register {
    Register::new((v >> 20) & 0x1F).expect("Passed something other than a value between 0 and 31")
}

fn rd(v: u32) -> Register {
    Register::new((v >> 7) & 0x1F).expect("Passed something other than a value between 0 and 31")
}

fn funct3(v: u32) -> u8 {
    ((v >> 12) & 0x7) as u8
}

fn funct7(v: u32) -> u8 {
    ((v >> 25) & 0x7F) as u8
}

fn get_itype_imm(v: u32) -> u32 {
    return v >> 20;
}

#[cfg(test)]
mod tests {
    use crate::{csr::Csr, instruction::Instruction};
    use super::*;



    #[test]
    fn test_parse() {
        let cases = vec![
            (0x00fff337, Instruction::LUI {
                rd: Register::T1,
                imm: UTypeImm::new(0xfff000).unwrap(),
            }),
            (0x0874f093, Instruction::ANDI {
                rd: Register::RA,
                rs1: Register::S1,
                imm: ITypeImm::new(0x87).unwrap(),
            }),
            (0x089323a3, Instruction::SW {
                rs1: Register::T1,
                rs2: Register::S1,
                imm: STypeImm::new(0x87).unwrap(),
            }),
            (0x7410076f, Instruction::JAL {
                rd: Register::A4,
                imm: JTypeImm::new(0xf40).unwrap(),
            }),
            (0x000f1263, Instruction::BNE {
                rs1: Register::T5,
                rs2: Register::ZERO,
                imm: BTypeImm::new(0x4).unwrap(),
            }),
            (0x01901f13, Instruction::SLLI {
                rs1: Register::ZERO,
                rd: Register::T5,
                imm: ShamtImm::new(25).unwrap(),
            })
        ];

        for (raw, instr) in cases {
            assert_eq!(Instruction::try_from(raw), Ok(instr));
        }
    }


    #[test]
    fn test_parse_csrrs() {
        let raw = 0xb80022f3;
        let instr = Instruction::CSRRS {
            rd: Register::T0,
            rs1: Register::ZERO,
            csr: Csr::MCYCLEH,
        };
        assert_eq!(Instruction::try_from(raw), Ok(instr));
    }
}

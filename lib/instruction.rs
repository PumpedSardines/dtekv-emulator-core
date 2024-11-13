/// This module is quite ugly, with a lot of boilerplate and reimplementation. I think this is the
/// best way to accomplish this functionality, and in a way the risc-v instructions are just a long
/// list of hardcoded values.

#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Format {
    R,
    I,
    S,
    B,
    U,
    J,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Instruction {
    LUI { rd: u8, imm: u32 },
    AUIPC { rd: u8, imm: u32 },
    JAL { rd: u8, imm: u32 },
    JALR { rd: u8, rs1: u8, imm: u32 },
    BEQ { rs1: u8, rs2: u8, imm: u32 },
    BNE { rs1: u8, rs2: u8, imm: u32 },
    BLT { rs1: u8, rs2: u8, imm: u32 },
    BGE { rs1: u8, rs2: u8, imm: u32 },
    BLTU { rs1: u8, rs2: u8, imm: u32 },
    BGEU { rs1: u8, rs2: u8, imm: u32 },
    LB { rd: u8, rs1: u8, imm: u32 },
    LH { rd: u8, rs1: u8, imm: u32 },
    LW { rd: u8, rs1: u8, imm: u32 },
    LBU { rd: u8, rs1: u8, imm: u32 },
    LHU { rd: u8, rs1: u8, imm: u32 },
    SB { rs1: u8, rs2: u8, imm: u32 },
    SH { rs1: u8, rs2: u8, imm: u32 },
    SW { rs1: u8, rs2: u8, imm: u32 },
    ADDI { rd: u8, rs1: u8, imm: u32 },
    SLTI { rd: u8, rs1: u8, imm: u32 },
    SLTIU { rd: u8, rs1: u8, imm: u32 },
    XORI { rd: u8, rs1: u8, imm: u32 },
    ORI { rd: u8, rs1: u8, imm: u32 },
    ANDI { rd: u8, rs1: u8, imm: u32 },
    SLLI { rd: u8, rs1: u8, imm: u32 },
    SRLI { rd: u8, rs1: u8, imm: u32 },
    SRAI { rd: u8, rs1: u8, imm: u32 },
    ADD { rd: u8, rs1: u8, rs2: u8 },
    SUB { rd: u8, rs1: u8, rs2: u8 },
    SLL { rd: u8, rs1: u8, rs2: u8 },
    SLT { rd: u8, rs1: u8, rs2: u8 },
    SLTU { rd: u8, rs1: u8, rs2: u8 },
    XOR { rd: u8, rs1: u8, rs2: u8 },
    SRL { rd: u8, rs1: u8, rs2: u8 },
    SRA { rd: u8, rs1: u8, rs2: u8 },
    OR { rd: u8, rs1: u8, rs2: u8 },
    AND { rd: u8, rs1: u8, rs2: u8 },
    CSRRW { rd: u8, rs1: u8, imm: u32 },
    CSRRS { rd: u8, rs1: u8, imm: u32 },
    CSRRC { rd: u8, rs1: u8, imm: u32 },
    CSRRWI { uimm: u8, rd: u8, imm: u32 },
    CSRRSI { uimm: u8, rd: u8, imm: u32 },
    CSRRCI { uimm: u8, rd: u8, imm: u32 },
    MRET,
    ECALL,
    MUL { rd: u8, rs1: u8, rs2: u8 },
    MULH { rd: u8, rs1: u8, rs2: u8 },
    MULHSU { rd: u8, rs1: u8, rs2: u8 },
    MULHU { rd: u8, rs1: u8, rs2: u8 },
    DIV { rd: u8, rs1: u8, rs2: u8 },
    DIVU { rd: u8, rs1: u8, rs2: u8 },
    REM { rd: u8, rs1: u8, rs2: u8 },
    REMU { rd: u8, rs1: u8, rs2: u8 },
}

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

impl TryFrom<u32> for Instruction {
    type Error = ();

    fn try_from(raw: u32) -> Result<Self, Self::Error> {
        let opcode = opcode(raw);
        let rd = rd(raw);
        let rs1 = rs1(raw);
        let rs2 = rs2(raw);
        let funct3 = funct3(raw);
        let funct7 = funct7(raw);

        match opcode {
            LUI => Ok(Instruction::LUI {
                rd,
                imm: imm(raw, Format::U),
            }),
            AUIPC => Ok(Instruction::AUIPC {
                rd,
                imm: imm(raw, Format::U),
            }),
            JAL => Ok(Instruction::JAL {
                rd,
                imm: imm(raw, Format::J),
            }),
            JALR => Ok(Instruction::JALR {
                rd,
                rs1,
                imm: imm(raw, Format::I),
            }),
            BRANCH => match funct3 {
                BEQ => Ok(Instruction::BEQ {
                    rs1,
                    rs2,
                    imm: imm(raw, Format::B),
                }),
                BNE => Ok(Instruction::BNE {
                    rs1,
                    rs2,
                    imm: imm(raw, Format::B),
                }),
                BLT => Ok(Instruction::BLT {
                    rs1,
                    rs2,
                    imm: imm(raw, Format::B),
                }),
                BGE => Ok(Instruction::BGE {
                    rs1,
                    rs2,
                    imm: imm(raw, Format::B),
                }),
                BLTU => Ok(Instruction::BLTU {
                    rs1,
                    rs2,
                    imm: imm(raw, Format::B),
                }),
                BGEU => Ok(Instruction::BGEU {
                    rs1,
                    rs2,
                    imm: imm(raw, Format::B),
                }),
                _ => Err(()),
            },
            LOAD => match funct3 {
                LB => Ok(Instruction::LB {
                    rd,
                    rs1,
                    imm: imm(raw, Format::I),
                }),
                LH => Ok(Instruction::LH {
                    rd,
                    rs1,
                    imm: imm(raw, Format::I),
                }),
                LW => Ok(Instruction::LW {
                    rd,
                    rs1,
                    imm: imm(raw, Format::I),
                }),
                LBU => Ok(Instruction::LBU {
                    rd,
                    rs1,
                    imm: imm(raw, Format::I),
                }),
                LHU => Ok(Instruction::LHU {
                    rd,
                    rs1,
                    imm: imm(raw, Format::I),
                }),
                _ => Err(()),
            },
            STORE => match funct3 {
                SB => Ok(Instruction::SB {
                    rs1,
                    rs2,
                    imm: imm(raw, Format::S),
                }),
                SH => Ok(Instruction::SH {
                    rs1,
                    rs2,
                    imm: imm(raw, Format::S),
                }),
                SW => Ok(Instruction::SW {
                    rs1,
                    rs2,
                    imm: imm(raw, Format::S),
                }),
                _ => Err(()),
            },
            OP_IMM => match (funct3, funct7) {
                (SLLI, FUNCT7_SLLI) => Ok(Instruction::SLLI {
                    rd,
                    rs1,
                    imm: rs2 as u32,
                }),
                (SRLI_SRAI, FUNCT7_SRLI) => Ok(Instruction::SRLI {
                    rd,
                    rs1,
                    imm: rs2 as u32,
                }),
                (SRLI_SRAI, FUNCT7_SRAI) => Ok(Instruction::SRAI {
                    rd,
                    rs1,
                    imm: rs2 as u32,
                }),
                (ADDI, _) => Ok(Instruction::ADDI {
                    rd,
                    rs1,
                    imm: imm(raw, Format::I),
                }),
                (SLTI, _) => Ok(Instruction::SLTI {
                    rd,
                    rs1,
                    imm: imm(raw, Format::I),
                }),
                (SLTIU, _) => Ok(Instruction::SLTIU {
                    rd,
                    rs1,
                    imm: imm(raw, Format::I),
                }),
                (XORI, _) => Ok(Instruction::XORI {
                    rd,
                    rs1,
                    imm: imm(raw, Format::I),
                }),
                (ORI, _) => Ok(Instruction::ORI {
                    rd,
                    rs1,
                    imm: imm(raw, Format::I),
                }),
                (ANDI, _) => Ok(Instruction::ANDI {
                    rd,
                    rs1,
                    imm: imm(raw, Format::I),
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
                CSRRW => Ok(Instruction::CSRRW {
                    rd,
                    rs1,
                    imm: imm(raw, Format::I),
                }),
                CSRRS => Ok(Instruction::CSRRS {
                    rd,
                    rs1,
                    imm: imm(raw, Format::I),
                }),
                CSRRC => Ok(Instruction::CSRRC {
                    rd,
                    rs1,
                    imm: imm(raw, Format::I),
                }),
                CSRRWI => Ok(Instruction::CSRRWI {
                    uimm: rs1,
                    rd,
                    imm: imm(raw, Format::I),
                }),
                CSRRSI => Ok(Instruction::CSRRSI {
                    uimm: rs1,
                    rd,
                    imm: imm(raw, Format::I),
                }),
                CSRRCI => Ok(Instruction::CSRRCI {
                    uimm: rs1,
                    rd,
                    imm: imm(raw, Format::I),
                }),
                _ => Err(()),
            },
            _ => Err(()),
        }
    }
}

fn opcode(v: u32) -> u8 {
    (v & 0x7F) as u8
}

fn rs1(v: u32) -> u8 {
    ((v >> 15) & 0x1F) as u8
}

fn rs2(v: u32) -> u8 {
    ((v >> 20) & 0x1F) as u8
}

fn rd(v: u32) -> u8 {
    ((v >> 7) & 0x1F) as u8
}

fn funct3(v: u32) -> u8 {
    ((v >> 12) & 0x7) as u8
}

fn funct7(v: u32) -> u8 {
    ((v >> 25) & 0x7F) as u8
}

fn imm(v: u32, format: Format) -> u32 {
    match format {
        Format::I => ((v as i32) >> 20) as u32,
        Format::S => (((v as i32) >> 25) << 5) as u32 | ((v >> 7) & 0x1F),
        Format::B => {
            (((v as i32) >> 31) << 12) as u32
                | (((v >> 7) & 0x1) << 11)
                | (((v >> 25) & 0x3F) << 5)
                | (((v >> 8) & 0xF) << 1)
        }
        Format::U => v & 0xFFFFF000,
        Format::J => {
            (((v as i32) >> 31) << 20) as u32
                | (((v >> 21) & 0x3FF) << 1)
                | (((v >> 20) & 0x1) << 11)
                | (((v >> 12) & 0xFF) << 12)
        }
        Format::R => panic!("R format does not have an immediate field"),
    }
}

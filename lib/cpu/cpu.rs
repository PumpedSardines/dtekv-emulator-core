use crate::{
    cpu::{self, Bus, Csr, Regs},
    exception,
    instruction::Instruction,
    Data,
};

// TODO: Implement different instruction speed depending on IO device
pub const CLOCK_FEQ: u32 = 30_000_000;
pub const STORE_CYCLE: u32 = 40;
pub const LOAD_CYCLE: u32 = 40;

#[derive(Debug)]
pub struct Cpu<T: Data<()>> {
    pub bus: T,
    pub regs: Regs,
    pub csr: Csr,
    pub pc: u32,
    pub wait_cycles: u32,
}

impl Cpu<Bus> {
    pub fn new() -> Cpu<Bus> {
        Cpu {
            bus: Bus::new(),
            regs: Regs::new(),
            csr: Csr::new(),
            pc: 0,
            wait_cycles: 0,
        }
    }
}

impl<T: Data<()>> Cpu<T> {
    pub fn new_with_bus(bus: T) -> Cpu<T> {
        Cpu {
            bus,
            regs: Regs::new(),
            csr: Csr::new(),
            pc: 0,
            wait_cycles: 0,
        }
    }

    pub fn reset(&mut self) {
        self.regs.reset();
        self.csr.reset();
        self.pc = 4;
        self.wait_cycles = 0;
        // NOTE: Not sure if this happens when reset is triggered:
        self.csr.set_mstatus_mie(true);
    }

    fn lui(&mut self, imm: u32, rd: u8) -> Result<(), ()> {
        self.regs.set(rd, imm);
        self.pc += 4;
        Ok(())
    }

    fn auipc(&mut self, imm: u32, rd: u8) -> Result<(), ()> {
        self.regs.set(rd, self.pc.wrapping_add(imm));
        self.pc += 4;
        Ok(())
    }

    fn jal(&mut self, imm: u32, rd: u8) -> Result<(), ()> {
        self.regs.set(rd, self.pc + 4);
        self.pc = self.pc.wrapping_add(imm);
        Ok(())
    }

    fn jalr(&mut self, rs1: u8, imm: u32, rd: u8) -> Result<(), ()> {
        let rs1 = self.regs.get(rs1);
        self.regs.set(rd, self.pc + 4);
        self.pc = rs1.wrapping_add(imm);
        Ok(())
    }

    fn beq(&mut self, rs1: u8, rs2: u8, imm: u32) -> Result<(), ()> {
        let rs1 = self.regs.get(rs1);
        let rs2 = self.regs.get(rs2);
        if rs1 == rs2 {
            self.pc = self.pc.wrapping_add(imm);
        } else {
            self.pc += 4;
        }
        Ok(())
    }

    fn bne(&mut self, rs1: u8, rs2: u8, imm: u32) -> Result<(), ()> {
        let rs1 = self.regs.get(rs1);
        let rs2 = self.regs.get(rs2);
        if rs1 != rs2 {
            self.pc = self.pc.wrapping_add(imm);
        } else {
            self.pc += 4;
        }
        Ok(())
    }

    fn blt(&mut self, rs1: u8, rs2: u8, imm: u32) -> Result<(), ()> {
        let rs1 = self.regs.get(rs1);
        let rs2 = self.regs.get(rs2);
        if (rs1 as i32) < (rs2 as i32) {
            self.pc = self.pc.wrapping_add(imm);
        } else {
            self.pc += 4;
        }
        Ok(())
    }

    fn bge(&mut self, rs1: u8, rs2: u8, imm: u32) -> Result<(), ()> {
        let rs1 = self.regs.get(rs1);
        let rs2 = self.regs.get(rs2);
        if (rs1 as i32) >= (rs2 as i32) {
            self.pc = self.pc.wrapping_add(imm);
        } else {
            self.pc += 4;
        }
        Ok(())
    }

    fn bltu(&mut self, rs1: u8, rs2: u8, imm: u32) -> Result<(), ()> {
        let rs1 = self.regs.get(rs1);
        let rs2 = self.regs.get(rs2);
        if rs1 < rs2 {
            self.pc = self.pc.wrapping_add(imm);
        } else {
            self.pc += 4;
        }
        Ok(())
    }

    fn bgeu(&mut self, rs1: u8, rs2: u8, imm: u32) -> Result<(), ()> {
        let rs1 = self.regs.get(rs1);
        let rs2 = self.regs.get(rs2);
        if rs1 >= rs2 {
            self.pc = self.pc.wrapping_add(imm);
        } else {
            self.pc += 4;
        }
        Ok(())
    }

    fn lb(&mut self, rs1: u8, imm: u32, rd: u8) -> Result<(), ()> {
        self.add_wait_cycles(LOAD_CYCLE);

        let rs1 = self.regs.get(rs1);
        let addr = rs1.wrapping_add(imm);
        let byte = self.bus.load_byte(addr)?;
        self.regs.set(rd, byte as i8 as i32 as u32);
        self.pc += 4;
        Ok(())
    }

    fn lh(&mut self, rs1: u8, imm: u32, rd: u8) -> Result<(), ()> {
        self.add_wait_cycles(LOAD_CYCLE);

        let rs1 = self.regs.get(rs1);
        let addr = rs1.wrapping_add(imm);
        let halfword = self.bus.load_halfword(addr)?;
        self.regs.set(rd, halfword as i16 as i32 as u32);
        self.pc += 4;
        Ok(())
    }

    fn lw(&mut self, rs1: u8, imm: u32, rd: u8) -> Result<(), ()> {
        self.add_wait_cycles(LOAD_CYCLE);

        let rs1 = self.regs.get(rs1);
        let addr = rs1.wrapping_add(imm);
        let word = self.bus.load_word(addr)?;
        self.regs.set(rd, word);
        self.pc += 4;
        Ok(())
    }

    fn lbu(&mut self, rs1: u8, imm: u32, rd: u8) -> Result<(), ()> {
        self.add_wait_cycles(LOAD_CYCLE);

        let rs1 = self.regs.get(rs1);
        let addr = rs1.wrapping_add(imm);
        let byte = self.bus.load_byte(addr)?;
        self.regs.set(rd, byte as u32);
        self.pc += 4;
        Ok(())
    }

    fn lhu(&mut self, rs1: u8, imm: u32, rd: u8) -> Result<(), ()> {
        self.add_wait_cycles(LOAD_CYCLE);

        let rs1 = self.regs.get(rs1);
        let addr = rs1.wrapping_add(imm);
        let halfword = self.bus.load_halfword(addr)?;
        self.regs.set(rd, halfword as u32);
        self.pc += 4;
        Ok(())
    }

    fn sb(&mut self, rs1: u8, rs2: u8, imm: u32) -> Result<(), ()> {
        self.add_wait_cycles(STORE_CYCLE);

        let rs1 = self.regs.get(rs1);
        let rs2 = self.regs.get(rs2);
        let addr = rs1.wrapping_add(imm);
        self.bus.store_byte(addr, rs2 as u8)?;
        self.pc += 4;
        Ok(())
    }

    fn sh(&mut self, rs1: u8, rs2: u8, imm: u32) -> Result<(), ()> {
        self.add_wait_cycles(STORE_CYCLE);

        let rs1 = self.regs.get(rs1);
        let rs2 = self.regs.get(rs2);
        let addr = rs1.wrapping_add(imm);
        self.bus.store_halfword(addr, rs2 as u16)?;
        self.pc += 4;
        Ok(())
    }

    fn sw(&mut self, rs1: u8, rs2: u8, imm: u32) -> Result<(), ()> {
        self.add_wait_cycles(STORE_CYCLE);

        let rs1 = self.regs.get(rs1);
        let rs2 = self.regs.get(rs2);
        let addr = rs1.wrapping_add(imm);
        self.bus.store_word(addr, rs2)?;
        self.pc += 4;
        Ok(())
    }

    fn addi(&mut self, rs1: u8, imm: u32, rd: u8) -> Result<(), ()> {
        let rs1 = self.regs.get(rs1);
        self.regs.set(rd, rs1.wrapping_add(imm));
        self.pc += 4;
        Ok(())
    }

    fn andi(&mut self, rs1: u8, imm: u32, rd: u8) -> Result<(), ()> {
        let rs1 = self.regs.get(rs1);
        self.regs.set(rd, rs1 & imm);
        self.pc += 4;
        Ok(())
    }

    fn ori(&mut self, rs1: u8, imm: u32, rd: u8) -> Result<(), ()> {
        let rs1 = self.regs.get(rs1);
        self.regs.set(rd, rs1 | imm);
        self.pc += 4;
        Ok(())
    }

    fn xori(&mut self, rs1: u8, imm: u32, rd: u8) -> Result<(), ()> {
        let rs1 = self.regs.get(rs1);
        self.regs.set(rd, rs1 ^ imm);
        self.pc += 4;
        Ok(())
    }

    fn slli(&mut self, rs1: u8, imm: u32, rd: u8) -> Result<(), ()> {
        let rs1 = self.regs.get(rs1);
        self.regs.set(rd, rs1.wrapping_shl(imm));
        self.pc += 4;
        Ok(())
    }

    fn srli(&mut self, rs1: u8, imm: u32, rd: u8) -> Result<(), ()> {
        let rs1 = self.regs.get(rs1);
        self.regs.set(rd, rs1.wrapping_shr(imm));
        self.pc += 4;
        Ok(())
    }

    fn srai(&mut self, rs1: u8, imm: u32, rd: u8) -> Result<(), ()> {
        let rs1 = self.regs.get(rs1);
        self.regs.set(rd, (rs1 as i32).wrapping_shr(imm) as u32);
        self.pc += 4;
        Ok(())
    }

    fn slti(&mut self, rs1: u8, imm: u32, rd: u8) -> Result<(), ()> {
        let rs1 = self.regs.get(rs1);
        self.regs
            .set(rd, if (rs1 as i32) < (imm as i32) { 1 } else { 0 });
        self.pc += 4;
        Ok(())
    }

    fn sltiu(&mut self, rs1: u8, imm: u32, rd: u8) -> Result<(), ()> {
        let rs1 = self.regs.get(rs1);
        self.regs.set(rd, if rs1 < imm { 1 } else { 0 });
        self.pc += 4;
        Ok(())
    }

    fn add(&mut self, rs1: u8, rs2: u8, rd: u8) -> Result<(), ()> {
        let rs1 = self.regs.get(rs1);
        let rs2 = self.regs.get(rs2);
        self.regs.set(rd, rs1.wrapping_add(rs2));
        self.pc += 4;
        Ok(())
    }

    fn sub(&mut self, rs1: u8, rs2: u8, rd: u8) -> Result<(), ()> {
        let rs1 = self.regs.get(rs1);
        let rs2 = self.regs.get(rs2);
        self.regs.set(rd, rs1.wrapping_sub(rs2));
        self.pc += 4;
        Ok(())
    }

    fn slt(&mut self, rs1: u8, rs2: u8, rd: u8) -> Result<(), ()> {
        let rs1 = self.regs.get(rs1);
        let rs2 = self.regs.get(rs2);
        self.regs
            .set(rd, if (rs1 as i32) < (rs2 as i32) { 1 } else { 0 });
        self.pc += 4;
        Ok(())
    }

    fn sltu(&mut self, rs1: u8, rs2: u8, rd: u8) -> Result<(), ()> {
        let rs1 = self.regs.get(rs1);
        let rs2 = self.regs.get(rs2);
        self.regs.set(rd, if rs1 < rs2 { 1 } else { 0 });
        self.pc += 4;
        Ok(())
    }

    fn sll(&mut self, rs1: u8, rs2: u8, rd: u8) -> Result<(), ()> {
        let rs1 = self.regs.get(rs1);
        let rs2 = self.regs.get(rs2) & 0x1f;
        self.regs.set(rd, rs1.wrapping_shl(rs2));
        self.pc += 4;
        Ok(())
    }

    fn srl(&mut self, rs1: u8, rs2: u8, rd: u8) -> Result<(), ()> {
        let rs1 = self.regs.get(rs1);
        let rs2 = self.regs.get(rs2) & 0x1f;
        self.regs.set(rd, rs1.wrapping_shr(rs2));
        self.pc += 4;
        Ok(())
    }

    fn sra(&mut self, rs1: u8, rs2: u8, rd: u8) -> Result<(), ()> {
        let rs1 = self.regs.get(rs1);
        let rs2 = self.regs.get(rs2) & 0x1f;
        self.regs.set(rd, (rs1 as i32).wrapping_shr(rs2) as u32);
        self.pc += 4;
        Ok(())
    }

    fn and(&mut self, rs1: u8, rs2: u8, rd: u8) -> Result<(), ()> {
        let rs1 = self.regs.get(rs1);
        let rs2 = self.regs.get(rs2);
        self.regs.set(rd, rs1 & rs2);
        self.pc += 4;
        Ok(())
    }

    fn or(&mut self, rs1: u8, rs2: u8, rd: u8) -> Result<(), ()> {
        let rs1 = self.regs.get(rs1);
        let rs2 = self.regs.get(rs2);
        self.regs.set(rd, rs1 | rs2);
        self.pc += 4;
        Ok(())
    }

    fn xor(&mut self, rs1: u8, rs2: u8, rd: u8) -> Result<(), ()> {
        let rs1 = self.regs.get(rs1);
        let rs2 = self.regs.get(rs2);
        self.regs.set(rd, rs1 ^ rs2);
        self.pc += 4;
        Ok(())
    }

    fn csrrw(&mut self, rs1: u8, imm: u32, rd: u8) -> Result<(), ()> {
        let value = self.csr.load(imm);
        self.csr.store(imm, self.regs.get(rs1));
        self.regs.set(rd, value);
        self.pc += 4;
        Ok(())
    }

    fn csrrs(&mut self, rs1: u8, imm: u32, rd: u8) -> Result<(), ()> {
        let value = self.csr.load(imm);
        self.csr.store(imm, self.csr.load(imm) | self.regs.get(rs1));
        self.regs.set(rd, value);
        self.pc += 4;
        Ok(())
    }

    fn csrrc(&mut self, rs1: u8, imm: u32, rd: u8) -> Result<(), ()> {
        let value = self.csr.load(imm);
        self.csr
            .store(imm, self.csr.load(imm) & !self.regs.get(rs1));
        self.regs.set(rd, value);
        self.pc += 4;
        Ok(())
    }

    fn csrrwi(&mut self, _uimm: u8, _imm: u32, _rd: u8) -> Result<(), ()> {
        // WARNING: Need to research what the dtekv chip does when running into this instruction
        panic!("csrrwi not implemented");
        // let value = self.csr.read(imm);
        // self.csr.write(imm, uimm as u32);
        // self.regs.set(rd, value);
        // self.pc += 4;
    }

    fn csrrsi(&mut self, uimm: u8, imm: u32, rd: u8) -> Result<(), ()> {
        let value = self.csr.load(imm);
        // NOTE: Dtekv differs from risc-v here:
        self.csr
            .store(imm, self.csr.load(imm) | (1 << (uimm as u32)));
        self.regs.set(rd, value);
        self.pc += 4;
        Ok(())
    }

    fn csrrci(&mut self, _uimm: u8, _imm: u32, _rd: u8) -> Result<(), ()> {
        // WARNING: Need to research what the dtekv chip does when running into this instruction
        panic!("csrrci not implemented");
        // let value = self.csr.read(imm);
        // self.csr.write(imm, self.csr.read(imm) & !(uimm as u32));
        // self.regs.set(rd, value);
        // self.pc += 4;
    }

    fn mret(&mut self) -> Result<(), ()> {
        self.pc = self.csr.load(cpu::MEPC);
        self.csr.set_mstatus_mie(self.csr.get_mstatus_mpie());
        self.csr.set_mstatus_mpie(true);
        Ok(())
    }

    fn ecall(&mut self) -> Result<(), ()> {
        self.pc += 4;
        self.interrupt(exception::ENVIRONMENT_CALL_FROM_M_MODE);
        Ok(())
    }

    fn mul(&mut self, rs1: u8, rs2: u8, rd: u8) -> Result<(), ()> {
        let rs1 = self.regs.get(rs1) as i32 as i64;
        let rs2 = self.regs.get(rs2) as i32 as i64;
        self.regs.set(rd, rs1.wrapping_mul(rs2) as u32);
        self.pc += 4;
        Ok(())
    }

    fn mulh(&mut self, rs1: u8, rs2: u8, rd: u8) -> Result<(), ()> {
        let rs1 = self.regs.get(rs1) as i32 as i64;
        let rs2 = self.regs.get(rs2) as i32 as i64;
        let result = rs1.wrapping_mul(rs2);
        self.regs.set(rd, (result >> 32) as u32);
        self.pc += 4;
        Ok(())
    }

    fn mulhu(&mut self, rs1: u8, rs2: u8, rd: u8) -> Result<(), ()> {
        let rs1 = self.regs.get(rs1) as u32 as u64;
        let rs2 = self.regs.get(rs2) as u32 as u64;
        let result = rs1.wrapping_mul(rs2);
        self.regs.set(rd, (result >> 32) as u32);
        self.pc += 4;
        Ok(())
    }

    fn mulhsu(&mut self, rs1: u8, rs2: u8, rd: u8) -> Result<(), ()> {
        let rs1 = self.regs.get(rs1) as i32 as i64;
        let rs2 = self.regs.get(rs2) as u32 as i64;
        let result = rs1.wrapping_mul(rs2);
        self.regs.set(rd, (result >> 32) as u32);
        self.pc += 4;
        Ok(())
    }

    fn div(&mut self, rs1: u8, rs2: u8, rd: u8) -> Result<(), ()> {
        let rs1 = self.regs.get(rs1) as i32;
        let rs2 = self.regs.get(rs2) as i32;
        if rs2 == 0 {
            self.regs.set(rd, 0xFFFFFFFF);
        } else {
            if rs1 == i32::MIN && rs2 == -1 {
                self.regs.set(rd, rs1 as u32);
            } else {
                self.regs.set(rd, (rs1 / rs2) as u32);
            }
        }
        self.pc += 4;
        Ok(())
    }

    fn divu(&mut self, rs1: u8, rs2: u8, rd: u8) -> Result<(), ()> {
        let rs1 = self.regs.get(rs1);
        let rs2 = self.regs.get(rs2);
        if rs2 == 0 {
            self.regs.set(rd, 0xFFFFFFFF);
        } else {
            self.regs.set(rd, rs1 / rs2);
        }
        self.pc += 4;
        Ok(())
    }

    fn rem(&mut self, rs1: u8, rs2: u8, rd: u8) -> Result<(), ()> {
        let rs1 = self.regs.get(rs1) as i32;
        let rs2 = self.regs.get(rs2) as i32;
        if rs2 == 0 {
            self.regs.set(rd, rs1 as u32);
        } else {
            if rs1 == i32::MIN && rs2 == -1 {
                self.regs.set(rd, 0);
            } else {
                self.regs.set(rd, (rs1 % rs2) as u32);
            }
        }
        self.pc += 4;
        Ok(())
    }

    fn remu(&mut self, rs1: u8, rs2: u8, rd: u8) -> Result<(), ()> {
        let rs1 = self.regs.get(rs1);
        let rs2 = self.regs.get(rs2);
        if rs2 == 0 {
            self.regs.set(rd, rs1);
        } else {
            self.regs.set(rd, rs1 % rs2);
        }
        self.pc += 4;
        Ok(())
    }

    fn fetch_instruction(&self) -> Result<Instruction, u32> {
        if (self.pc & 3) != 0 {
            return Err(exception::INSTRUCTION_ADDRESS_MISALIGNED);
        }
        self.bus
            .load_word(self.pc)
            .unwrap_or_else(|_| {
                // TODO: Check what happens if this happens
                panic!("Instruction fetch failed")
            })
            .try_into()
            .map_err(|_| exception::ILLEGAL_INSTRUCTION)
    }

    fn exec_instruction(&mut self, instruction: Instruction) -> Result<(), ()> {
        use Instruction as I;

        match instruction {
            I::LUI { imm, rd } => self.lui(imm, rd),
            I::AUIPC { imm, rd } => self.auipc(imm, rd),
            I::JAL { imm, rd } => self.jal(imm, rd),
            I::JALR { rs1, imm, rd } => self.jalr(rs1, imm, rd),
            I::BEQ { rs1, rs2, imm } => self.beq(rs1, rs2, imm),
            I::BNE { rs1, rs2, imm } => self.bne(rs1, rs2, imm),
            I::BLT { rs1, rs2, imm } => self.blt(rs1, rs2, imm),
            I::BGE { rs1, rs2, imm } => self.bge(rs1, rs2, imm),
            I::BLTU { rs1, rs2, imm } => self.bltu(rs1, rs2, imm),
            I::BGEU { rs1, rs2, imm } => self.bgeu(rs1, rs2, imm),
            I::LB { rs1, imm, rd } => self.lb(rs1, imm, rd),
            I::LH { rs1, imm, rd } => self.lh(rs1, imm, rd),
            I::LW { rs1, imm, rd } => self.lw(rs1, imm, rd),
            I::LBU { rs1, imm, rd } => self.lbu(rs1, imm, rd),
            I::LHU { rs1, imm, rd } => self.lhu(rs1, imm, rd),
            I::SB { rs1, rs2, imm } => self.sb(rs1, rs2, imm),
            I::SH { rs1, rs2, imm } => self.sh(rs1, rs2, imm),
            I::SW { rs1, rs2, imm } => self.sw(rs1, rs2, imm),
            I::ADDI { rs1, imm, rd } => self.addi(rs1, imm, rd),
            I::ANDI { rs1, imm, rd } => self.andi(rs1, imm, rd),
            I::ORI { rs1, imm, rd } => self.ori(rs1, imm, rd),
            I::XORI { rs1, imm, rd } => self.xori(rs1, imm, rd),
            I::SLLI { rs1, imm, rd } => self.slli(rs1, imm, rd),
            I::SRLI { rs1, imm, rd } => self.srli(rs1, imm, rd),
            I::SRAI { rs1, imm, rd } => self.srai(rs1, imm, rd),
            I::SLTI { rs1, imm, rd } => self.slti(rs1, imm, rd),
            I::SLTIU { rs1, imm, rd } => self.sltiu(rs1, imm, rd),
            I::ADD { rs1, rs2, rd } => self.add(rs1, rs2, rd),
            I::SUB { rs1, rs2, rd } => self.sub(rs1, rs2, rd),
            I::SLT { rs1, rs2, rd } => self.slt(rs1, rs2, rd),
            I::SLTU { rs1, rs2, rd } => self.sltu(rs1, rs2, rd),
            I::SLL { rs1, rs2, rd } => self.sll(rs1, rs2, rd),
            I::SRL { rs1, rs2, rd } => self.srl(rs1, rs2, rd),
            I::SRA { rs1, rs2, rd } => self.sra(rs1, rs2, rd),
            I::AND { rs1, rs2, rd } => self.and(rs1, rs2, rd),
            I::OR { rs1, rs2, rd } => self.or(rs1, rs2, rd),
            I::XOR { rs1, rs2, rd } => self.xor(rs1, rs2, rd),
            I::CSRRW { rs1, imm, rd } => self.csrrw(rs1, imm, rd),
            I::CSRRS { rs1, imm, rd } => self.csrrs(rs1, imm, rd),
            I::CSRRC { rs1, imm, rd } => self.csrrc(rs1, imm, rd),
            I::CSRRWI { uimm, imm, rd } => self.csrrwi(uimm, imm, rd),
            I::CSRRSI { uimm, imm, rd } => self.csrrsi(uimm, imm, rd),
            I::CSRRCI { uimm, imm, rd } => self.csrrci(uimm, imm, rd),
            I::MRET => self.mret(),
            I::ECALL => self.ecall(),
            I::MUL { rs1, rs2, rd } => self.mul(rs1, rs2, rd),
            I::MULH { rs1, rs2, rd } => self.mulh(rs1, rs2, rd),
            I::MULHSU { rs1, rs2, rd } => self.mulhsu(rs1, rs2, rd),
            I::MULHU { rs1, rs2, rd } => self.mulhu(rs1, rs2, rd),
            I::DIV { rs1, rs2, rd } => self.div(rs1, rs2, rd),
            I::DIVU { rs1, rs2, rd } => self.divu(rs1, rs2, rd),
            I::REM { rs1, rs2, rd } => self.rem(rs1, rs2, rd),
            I::REMU { rs1, rs2, rd } => self.remu(rs1, rs2, rd),
        }
    }

    pub fn interrupt(&mut self, cause: u32) {
        // If interrupts are disabled, ignore the interrupt
        if !self.csr.get_mstatus_mie() {
            return;
        }

        let exception_pc = self.pc.wrapping_sub(4);
        self.pc = 0;
        self.csr.store(cpu::MEPC, exception_pc);
        self.csr.store(cpu::MCAUSE, cause);
        self.csr.set_mstatus_mpie(self.csr.get_mstatus_mie());
        self.csr.set_mstatus_mie(false);

        if cause & 0x80000000 != 0 {
            // When it's an external interrupt, we need to increment the MEPC by 4
            let exception_pc = self.csr.load(cpu::MEPC);
            let exception_pc = exception_pc.wrapping_add(4);
            self.csr.store(cpu::MEPC, exception_pc);
        }
    }

    pub fn external_interrupt(&mut self, cause: u32) {
        if self.csr.load(cpu::MIE) & (1 << cause) == 0 {
            // This interrupt is disabled
            return;
        }

        let cause = cause | 0x80000000;
        self.interrupt(cause);
    }

    pub fn add_wait_cycles(&mut self, cycles: u32) {
        if !cfg!(debug_assertions) {
            self.wait_cycles += cycles;
        } else {
            // In debug mode, the wait cycles are just slowing the emulator down unnecessarily
            // We don't want realism in debug mode, we just want to test that everything works
            self.wait_cycles += 0;
        }
    }

    pub fn clock(&mut self) {
        match self.wait_cycles {
            0 => {
                let instr: Result<Instruction, u32> = self.fetch_instruction();

                if let Ok(instr) = instr {
                    let res = self.exec_instruction(instr);

                    if cfg!(debug_assertions) {
                        res.unwrap_or_else(|_| {
                           panic!("In the future exec_instruction shouldn't return an error, instruction: {:?}, regs: {:?}, pc: {}", instr, self.regs, self.pc)
                       });
                    } else {
                        res.unwrap();
                    }
                } else {
                    self.interrupt(instr.unwrap_err());
                }
            }
            _ => {
                if !cfg!(debug_assertions) {
                    self.wait_cycles -= 1;
                } else {
                    unreachable!("This should never happen since we don't increment the wait cycles in debug mode");
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::io;

    use super::*;

    #[test]
    fn test_lui() {
        let mut cpu = Cpu::new();

        cpu.pc = 0;
        // This seems weird but the imm value is calculated when parsing the instruction and not
        // when the instruction is executed. That's why we check if the reg x1 is the same as we
        // passed in
        cpu.exec_instruction(Instruction::LUI {
            imm: 0x12345,
            rd: 1,
        })
        .unwrap();
        assert_eq!(cpu.regs.get(1), 0x12345);
        assert_eq!(cpu.pc, 4);
    }

    #[test]
    fn test_auipc() {
        let mut cpu = Cpu::new();

        cpu.pc = 0x40000000;
        cpu.exec_instruction(Instruction::AUIPC {
            imm: 0x3000000,
            rd: 1,
        })
        .unwrap();
        assert_eq!(cpu.regs.get(1), 0x43000000);
        assert_eq!(cpu.pc, 0x40000004);
    }

    #[test]
    fn test_jal() {
        let mut cpu = Cpu::new();

        cpu.pc = 0x40000000;
        cpu.exec_instruction(Instruction::JAL { imm: 0x1000, rd: 1 })
            .unwrap();
        assert_eq!(cpu.regs.get(1), 0x40000004);
        assert_eq!(cpu.pc, 0x40001000);
    }

    #[test]
    fn test_jalr() {
        let mut cpu = Cpu::new();

        cpu.pc = 0x40000000;
        cpu.regs.set(2, 0x40000000);
        cpu.exec_instruction(Instruction::JALR {
            rs1: 2,
            imm: 0x1000,
            rd: 1,
        })
        .unwrap();
        assert_eq!(cpu.regs.get(1), 0x40000004);
        assert_eq!(cpu.pc, 0x40001000);
    }

    #[test]
    fn test_beq() {
        let mut cpu = Cpu::new();

        cpu.pc = 0;
        cpu.regs.set(1, 0x1234);
        cpu.regs.set(2, 0x1234);
        cpu.exec_instruction(Instruction::BEQ {
            rs1: 1,
            rs2: 2,
            imm: 0x1000,
        })
        .unwrap();
        assert_eq!(cpu.pc, 0x1000);
        cpu.regs.set(2, 0x1235);
        cpu.exec_instruction(Instruction::BEQ {
            rs1: 1,
            rs2: 2,
            imm: 0x1000,
        })
        .unwrap();
        assert_eq!(cpu.pc, 0x1004);
    }

    #[test]
    fn test_bne() {
        let mut cpu = Cpu::new();

        cpu.pc = 0;
        cpu.regs.set(1, 0x1234);
        cpu.regs.set(2, 0x1234);
        cpu.exec_instruction(Instruction::BNE {
            rs1: 1,
            rs2: 2,
            imm: 0x1000,
        })
        .unwrap();
        assert_eq!(cpu.pc, 4);
        cpu.regs.set(2, 0x1235);
        cpu.exec_instruction(Instruction::BNE {
            rs1: 1,
            rs2: 2,
            imm: 0x1000,
        })
        .unwrap();
        assert_eq!(cpu.pc, 0x1004);
    }

    #[test]
    fn test_blt() {
        let data = vec![
            (0x1234, 0x1235, true),
            (0x1235, 0x1235, false),
            (0x1236, 0x1235, false),
            (u32::MAX, 0x1235, true),
        ];

        for (rs1, rs2, expected) in data {
            let mut cpu = Cpu::new();

            cpu.pc = 8;
            cpu.regs.set(1, rs1);
            cpu.regs.set(2, rs2);
            cpu.exec_instruction(Instruction::BLT {
                rs1: 1,
                rs2: 2,
                imm: 0x1000,
            })
            .unwrap();
            assert_eq!(
                cpu.pc,
                if expected { 0x1008 } else { 12 },
                "rs1: {}, rs2: {}, should've jumped: {}",
                rs1,
                rs2,
                expected
            );
        }
    }

    #[test]
    fn test_bge() {
        let data = vec![
            (0x1234, 0x1235, false),
            (0x1235, 0x1235, true),
            (0x1236, 0x1235, true),
            (u32::MAX, 0x1235, false),
        ];

        for (rs1, rs2, expected) in data {
            let mut cpu = Cpu::new();

            cpu.pc = 8;
            cpu.regs.set(1, rs1);
            cpu.regs.set(2, rs2);
            cpu.exec_instruction(Instruction::BGE {
                rs1: 1,
                rs2: 2,
                imm: 0x1000,
            })
            .unwrap();
            assert_eq!(
                cpu.pc,
                if expected { 0x1008 } else { 12 },
                "rs1: {}, rs2: {}, should've jumped: {}",
                rs1,
                rs2,
                expected
            );
        }
    }

    #[test]
    fn test_bltu() {
        let data = vec![
            (0x1234, 0x1235, true),
            (0x1235, 0x1235, false),
            (0x1236, 0x1235, false),
            (u32::MAX, 0x1235, false),
        ];

        for (rs1, rs2, expected) in data {
            let mut cpu = Cpu::new();

            cpu.pc = 8;
            cpu.regs.set(1, rs1);
            cpu.regs.set(2, rs2);
            cpu.exec_instruction(Instruction::BLTU {
                rs1: 1,
                rs2: 2,
                imm: 0x1000,
            })
            .unwrap();
            assert_eq!(
                cpu.pc,
                if expected { 0x1008 } else { 12 },
                "rs1: {}, rs2: {}, should've jumped: {}",
                rs1,
                rs2,
                expected
            );
        }
    }

    #[test]
    fn test_bgeu() {
        let data = vec![
            (0x1234, 0x1235, false),
            (0x1235, 0x1235, true),
            (0x1236, 0x1235, true),
            (u32::MAX, 0x1235, true),
        ];

        for (rs1, rs2, expected) in data {
            let mut cpu = Cpu::new();

            cpu.pc = 8;
            cpu.regs.set(1, rs1);
            cpu.regs.set(2, rs2);
            cpu.exec_instruction(Instruction::BGEU {
                rs1: 1,
                rs2: 2,
                imm: 0x1000,
            })
            .unwrap();
            assert_eq!(
                cpu.pc,
                if expected { 0x1008 } else { 12 },
                "rs1: {}, rs2: {}, should've jumped: {}",
                rs1,
                rs2,
                expected
            );
        }
    }

    #[test]
    #[should_panic]
    fn test_lb() {
        // TODO: Implement test
        todo!();
    }

    #[test]
    #[should_panic]
    fn test_lh() {
        // TODO: Implement test
        todo!();
    }

    #[test]
    #[should_panic]
    fn test_lw() {
        // TODO: Implement test
        todo!();
    }

    #[test]
    #[should_panic]
    fn test_lbu() {
        // TODO: Implement test
        todo!();
    }

    #[test]
    #[should_panic]
    fn test_lhu() {
        // TODO: Implement test
        todo!();
    }

    #[test]
    #[should_panic]
    fn test_sb() {
        // TODO: Implement test
        todo!();
    }

    #[test]
    #[should_panic]
    fn test_sh() {
        // TODO: Implement test
        todo!();
    }

    #[test]
    #[should_panic]
    fn test_sw() {
        // TODO: Implement test
        todo!();
    }

    #[test]
    fn addi() {
        let data = vec![
            (0, 0, 0),
            (0, 1, 1),
            (1, 1, 2),
            (1, u32::MAX, 0),
            (u32::MAX, 1, 0),
        ];

        for (rs1, imm, expected) in data {
            let mut cpu = Cpu::new();
            cpu.regs.set(1, rs1);
            cpu.pc = 0;
            cpu.exec_instruction(Instruction::ADDI { rs1: 1, imm, rd: 2 })
                .unwrap();
            assert_eq!(cpu.regs.get(2), expected);
            assert_eq!(cpu.pc, 4);
        }
    }

    #[test]
    fn slti() {
        let data = vec![
            (0, 0, 0),
            (0, 1, 1),
            (1, 1, 0),
            (1, u32::MAX, 0),
            (u32::MAX, 1, 1),
        ];

        for (rs1, imm, expected) in data {
            let mut cpu = Cpu::new();
            cpu.regs.set(1, rs1);
            cpu.pc = 0;
            cpu.exec_instruction(Instruction::SLTI { rs1: 1, imm, rd: 2 })
                .unwrap();
            assert_eq!(cpu.regs.get(2), expected);
            assert_eq!(cpu.pc, 4);
        }
    }

    #[test]
    fn sltiu() {
        let data = vec![
            (0, 0, 0),
            (0, 1, 1),
            (1, 1, 0),
            (1, u32::MAX, 1),
            (u32::MAX, 1, 0),
        ];

        for (rs1, imm, expected) in data {
            let mut cpu = Cpu::new();
            cpu.regs.set(1, rs1);
            cpu.pc = 0;
            cpu.exec_instruction(Instruction::SLTIU { rs1: 1, imm, rd: 2 })
                .unwrap();
            assert_eq!(cpu.regs.get(2), expected);
            assert_eq!(cpu.pc, 4);
        }
    }

    #[test]
    fn test_all_alu_imm() {
        let mut cpu = Cpu::new();

        cpu.exec_instruction(0x00700293.try_into().unwrap())
            .unwrap(); // addi  x5,  zero, 7
        assert_eq!(cpu.regs.get(5), 7);
        cpu.exec_instruction(0x0052f393.try_into().unwrap())
            .unwrap(); // andi  x7,  x5, 5
        assert_eq!(cpu.regs.get(7), 5);
        cpu.exec_instruction(0x0052e413.try_into().unwrap())
            .unwrap(); // ori   x8,  x5, 5
        assert_eq!(cpu.regs.get(8), 7);
        cpu.exec_instruction(0x0052c493.try_into().unwrap())
            .unwrap(); // xori  x9,  x5, 5
        assert_eq!(cpu.regs.get(9), 2);
        cpu.exec_instruction(0x00229513.try_into().unwrap())
            .unwrap(); // slli  x10, x5, 2
        assert_eq!(cpu.regs.get(10), 28);
        cpu.exec_instruction(0x0022d593.try_into().unwrap())
            .unwrap(); // srli  x11, x5, 2
        assert_eq!(cpu.regs.get(11), 1);
        cpu.exec_instruction(0x4022d613.try_into().unwrap())
            .unwrap(); // srai  x12, x5, 2
        assert_eq!(cpu.regs.get(12), 1);
        cpu.exec_instruction(0x0052a693.try_into().unwrap())
            .unwrap(); // slti  x13, x5, 5
        assert_eq!(cpu.regs.get(13), 0);
        cpu.exec_instruction(0x0052b713.try_into().unwrap())
            .unwrap(); // sltiu x14, x5, 5
        assert_eq!(cpu.regs.get(14), 0);
    }

    #[test]
    fn test_srli_sali() {
        let mut cpu = Cpu::new();

        cpu.exec_instruction(0xfff00093.try_into().unwrap())
            .unwrap(); // li x1, -1
        cpu.exec_instruction(0x0020d113.try_into().unwrap())
            .unwrap(); // srli  x2, x1, 2
        assert_eq!(cpu.regs.get(2), 0x3fffffff);
        cpu.exec_instruction(0x4020d113.try_into().unwrap())
            .unwrap(); // srai  x2, x1, 2
        assert_eq!(cpu.regs.get(2), 0xffffffff);
    }

    #[test]
    fn test_slti_sltiu() {
        let mut cpu = Cpu::new();

        cpu.exec_instruction(0xfff00093.try_into().unwrap())
            .unwrap(); // li x1, -1
        cpu.exec_instruction(0x0000a113.try_into().unwrap())
            .unwrap(); // slti x2, x1, 0
        assert_eq!(cpu.regs.get(2), 1);
        cpu.exec_instruction(0x0000b193.try_into().unwrap())
            .unwrap(); // sltiu x3, x1, 0
        assert_eq!(cpu.regs.get(3), 0);
    }

    #[test]
    fn test_load_and_save() {
        let sdram = io::SDRam::new();
        let mut cpu = Cpu::new_with_bus(sdram);

        cpu.exec_instruction(0x361880b7.try_into().unwrap())
            .unwrap(); // lui x1, 0x36188
        cpu.exec_instruction(0x71908093.try_into().unwrap())
            .unwrap(); // addi x1, x1, 1817 # 0x36188719
        assert_eq!(cpu.regs.get(1), 0x36188719);
        cpu.exec_instruction(0x00102023.try_into().unwrap())
            .unwrap(); // sw x1, 0(x0)
        assert_eq!(cpu.bus.load_word(0).unwrap(), 0x36188719);
        cpu.exec_instruction(0x00000103.try_into().unwrap())
            .unwrap(); // lb x2, 0(x0)
        assert_eq!(cpu.regs.get(2), 0x19);
        cpu.exec_instruction(0x00100103.try_into().unwrap())
            .unwrap(); // lb x2, 1(x0)
        assert_eq!(cpu.regs.get(2), (-121i32) as u32);
        cpu.exec_instruction(0x00200103.try_into().unwrap())
            .unwrap(); // lb x2, 2(x0)
        assert_eq!(cpu.regs.get(2), 0x18);
        cpu.exec_instruction(0x00300103.try_into().unwrap())
            .unwrap(); // lb x2, 3(x0)
        assert_eq!(cpu.regs.get(2), 0x36);
        cpu.exec_instruction(0x00001103.try_into().unwrap())
            .unwrap(); // lh x2, 0(x0)
        assert_eq!(cpu.regs.get(2), (-30951i32) as u32);
        cpu.exec_instruction(0x00101103.try_into().unwrap())
            .unwrap(); // lh x2, 1(x0)
        assert_eq!(cpu.regs.get(2), 0x1887);
        cpu.exec_instruction(0x00004103.try_into().unwrap())
            .unwrap(); // lbu x2, 0(x0)
        assert_eq!(cpu.regs.get(2), 0x19);
        cpu.exec_instruction(0x00104103.try_into().unwrap())
            .unwrap(); // lbu x2, 1(x0)
        assert_eq!(cpu.regs.get(2), 0x87);
        cpu.exec_instruction(0x00204103.try_into().unwrap())
            .unwrap(); // lbu x2, 2(x0)
        assert_eq!(cpu.regs.get(2), 0x18);
        cpu.exec_instruction(0x00304103.try_into().unwrap())
            .unwrap(); // lbu x2, 3(x0)
        assert_eq!(cpu.regs.get(2), 0x36);
        cpu.exec_instruction(0x00005103.try_into().unwrap())
            .unwrap(); // lhu x2, 0(x0)
        assert_eq!(cpu.regs.get(2), 0x8719);
        cpu.exec_instruction(0x00105103.try_into().unwrap())
            .unwrap(); // lhu x2, 1(x0)
        assert_eq!(cpu.regs.get(2), 0x1887);
    }
}

use crate::{
    csr::{self, Csr},
    exception, Bus, Button, Data, HexDisplay, Instruction, LoadStore, Memory, Regs, Switch, Timer,
    Uart, Vga,
};

#[derive(Debug, Clone)]
pub struct Cpu {
    pub bus: Bus,
    pub regs: Regs,
    pub csr: Csr,
    pub pc: u32,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            bus: Bus {
                mem: Memory::empty(0x3ffffff),
                hex_display: HexDisplay::new(),
                switch: Switch::new(),
                uart: Uart::new(),
                button: Button::new(),
                vga: Vga::new(),
                timer: Timer::new(),
            },
            regs: Regs::new(),
            csr: Csr::new(),
            pc: 0,
        }
    }

    pub fn from_bin(bin: Vec<u8>) -> Cpu {
        let mut cpu = Cpu::new();
        cpu.bus.mem.load_data_at(0, bin);
        cpu
    }

    fn lui(&mut self, imm: u32, rd: u8) {
        self.regs.set(rd, imm);
        self.pc += 4;
    }

    fn auipc(&mut self, imm: u32, rd: u8) {
        self.regs.set(rd, self.pc.wrapping_add(imm));
        self.pc += 4;
    }

    fn jal(&mut self, imm: u32, rd: u8) {
        self.regs.set(rd, self.pc + 4);
        self.pc = self.pc.wrapping_add(imm);
    }

    fn jalr(&mut self, rs1: u8, imm: u32, rd: u8) {
        let rs1 = self.regs.get(rs1);
        self.regs.set(rd, self.pc + 4);
        self.pc = rs1.wrapping_add(imm);
    }

    fn beq(&mut self, rs1: u8, rs2: u8, imm: u32) {
        let rs1 = self.regs.get(rs1);
        let rs2 = self.regs.get(rs2);
        if rs1 == rs2 {
            self.pc = self.pc.wrapping_add(imm);
        } else {
            self.pc += 4;
        }
    }

    fn bne(&mut self, rs1: u8, rs2: u8, imm: u32) {
        let rs1 = self.regs.get(rs1);
        let rs2 = self.regs.get(rs2);
        if rs1 != rs2 {
            self.pc = self.pc.wrapping_add(imm);
        } else {
            self.pc += 4;
        }
    }

    fn blt(&mut self, rs1: u8, rs2: u8, imm: u32) {
        let rs1 = self.regs.get(rs1);
        let rs2 = self.regs.get(rs2);
        if (rs1 as i32) < (rs2 as i32) {
            self.pc = self.pc.wrapping_add(imm);
        } else {
            self.pc += 4;
        }
    }

    fn bge(&mut self, rs1: u8, rs2: u8, imm: u32) {
        let rs1 = self.regs.get(rs1);
        let rs2 = self.regs.get(rs2);
        if (rs1 as i32) >= (rs2 as i32) {
            self.pc = self.pc.wrapping_add(imm);
        } else {
            self.pc += 4;
        }
    }

    fn bltu(&mut self, rs1: u8, rs2: u8, imm: u32) {
        let rs1 = self.regs.get(rs1);
        let rs2 = self.regs.get(rs2);
        if rs1 < rs2 {
            self.pc = self.pc.wrapping_add(imm);
        } else {
            self.pc += 4;
        }
    }

    fn bgeu(&mut self, rs1: u8, rs2: u8, imm: u32) {
        let rs1 = self.regs.get(rs1);
        let rs2 = self.regs.get(rs2);
        if rs1 >= rs2 {
            self.pc = self.pc.wrapping_add(imm);
        } else {
            self.pc += 4;
        }
    }

    fn lb(&mut self, rs1: u8, imm: u32, rd: u8) {
        let rs1 = self.regs.get(rs1);
        let addr = rs1.wrapping_add(imm);
        let byte = self.bus.load_byte(addr);
        self.regs.set(rd, byte as i8 as i32 as u32);
        self.pc += 4;
    }

    fn lh(&mut self, rs1: u8, imm: u32, rd: u8) {
        let rs1 = self.regs.get(rs1);
        let addr = rs1.wrapping_add(imm);
        let halfword = self.bus.load_halfword(addr);
        self.regs.set(rd, halfword as i16 as i32 as u32);
        self.pc += 4;
    }

    fn lw(&mut self, rs1: u8, imm: u32, rd: u8) {
        let rs1 = self.regs.get(rs1);
        let addr = rs1.wrapping_add(imm);
        let word = self.bus.load_word(addr);
        self.regs.set(rd, word);
        self.pc += 4;
    }

    fn lbu(&mut self, rs1: u8, imm: u32, rd: u8) {
        let rs1 = self.regs.get(rs1);
        let addr = rs1.wrapping_add(imm);
        let byte = self.bus.load_byte(addr);
        self.regs.set(rd, byte as u32);
        self.pc += 4;
    }

    fn lhu(&mut self, rs1: u8, imm: u32, rd: u8) {
        let rs1 = self.regs.get(rs1);
        let addr = rs1.wrapping_add(imm);
        let halfword = self.bus.load_halfword(addr);
        self.regs.set(rd, halfword as u32);
        self.pc += 4;
    }

    fn sb(&mut self, rs1: u8, rs2: u8, imm: u32) {
        let rs1 = self.regs.get(rs1);
        let rs2 = self.regs.get(rs2);
        let addr = rs1.wrapping_add(imm);
        self.bus.store_byte(addr, rs2 as u8);
        self.pc += 4;
    }

    fn sh(&mut self, rs1: u8, rs2: u8, imm: u32) {
        let rs1 = self.regs.get(rs1);
        let rs2 = self.regs.get(rs2);
        let addr = rs1.wrapping_add(imm);
        self.bus.store_halfword(addr, rs2 as u16);
        self.pc += 4;
    }

    fn sw(&mut self, rs1: u8, rs2: u8, imm: u32) {
        let rs1 = self.regs.get(rs1);
        let rs2 = self.regs.get(rs2);
        let addr = rs1.wrapping_add(imm);
        self.bus.store_word(addr, rs2);
        self.pc += 4;
    }

    fn addi(&mut self, rs1: u8, imm: u32, rd: u8) {
        let rs1 = self.regs.get(rs1);
        self.regs.set(rd, rs1.wrapping_add(imm));
        self.pc += 4;
    }

    fn andi(&mut self, rs1: u8, imm: u32, rd: u8) {
        let rs1 = self.regs.get(rs1);
        self.regs.set(rd, rs1 & imm);
        self.pc += 4;
    }

    fn ori(&mut self, rs1: u8, imm: u32, rd: u8) {
        let rs1 = self.regs.get(rs1);
        self.regs.set(rd, rs1 | imm);
        self.pc += 4;
    }

    fn xori(&mut self, rs1: u8, imm: u32, rd: u8) {
        let rs1 = self.regs.get(rs1);
        self.regs.set(rd, rs1 ^ imm);
        self.pc += 4;
    }

    fn slli(&mut self, rs1: u8, imm: u32, rd: u8) {
        let rs1 = self.regs.get(rs1);
        self.regs.set(rd, rs1.wrapping_shl(imm));
        self.pc += 4;
    }

    fn srli(&mut self, rs1: u8, imm: u32, rd: u8) {
        let rs1 = self.regs.get(rs1);
        self.regs.set(rd, rs1.wrapping_shr(imm));
        self.pc += 4;
    }

    fn srai(&mut self, rs1: u8, imm: u32, rd: u8) {
        let rs1 = self.regs.get(rs1);
        self.regs.set(rd, (rs1 as i32).wrapping_shr(imm) as u32);
        self.pc += 4;
    }

    fn slti(&mut self, rs1: u8, imm: u32, rd: u8) {
        let rs1 = self.regs.get(rs1);
        self.regs
            .set(rd, if (rs1 as i32) < (imm as i32) { 1 } else { 0 });
        self.pc += 4;
    }

    fn sltiu(&mut self, rs1: u8, imm: u32, rd: u8) {
        let rs1 = self.regs.get(rs1);
        self.regs.set(rd, if rs1 < imm { 1 } else { 0 });
        self.pc += 4;
    }

    fn add(&mut self, rs1: u8, rs2: u8, rd: u8) {
        let rs1 = self.regs.get(rs1);
        let rs2 = self.regs.get(rs2);
        self.regs.set(rd, rs1.wrapping_add(rs2));
        self.pc += 4;
    }

    fn sub(&mut self, rs1: u8, rs2: u8, rd: u8) {
        let rs1 = self.regs.get(rs1);
        let rs2 = self.regs.get(rs2);
        self.regs.set(rd, rs1.wrapping_sub(rs2));
        self.pc += 4;
    }

    fn slt(&mut self, rs1: u8, rs2: u8, rd: u8) {
        let rs1 = self.regs.get(rs1);
        let rs2 = self.regs.get(rs2);
        self.regs
            .set(rd, if (rs1 as i32) < (rs2 as i32) { 1 } else { 0 });
        self.pc += 4;
    }

    fn sltu(&mut self, rs1: u8, rs2: u8, rd: u8) {
        let rs1 = self.regs.get(rs1);
        let rs2 = self.regs.get(rs2);
        self.regs.set(rd, if rs1 < rs2 { 1 } else { 0 });
        self.pc += 4;
    }

    fn sll(&mut self, rs1: u8, rs2: u8, rd: u8) {
        let rs1 = self.regs.get(rs1);
        let rs2 = self.regs.get(rs2) & 0x1f;
        self.regs.set(rd, rs1.wrapping_shl(rs2));
        self.pc += 4;
    }

    fn srl(&mut self, rs1: u8, rs2: u8, rd: u8) {
        let rs1 = self.regs.get(rs1);
        let rs2 = self.regs.get(rs2) & 0x1f;
        self.regs.set(rd, rs1.wrapping_shr(rs2));
        self.pc += 4;
    }

    fn sra(&mut self, rs1: u8, rs2: u8, rd: u8) {
        let rs1 = self.regs.get(rs1);
        let rs2 = self.regs.get(rs2) & 0x1f;
        self.regs.set(rd, (rs1 as i32).wrapping_shr(rs2) as u32);
        self.pc += 4;
    }

    fn and(&mut self, rs1: u8, rs2: u8, rd: u8) {
        let rs1 = self.regs.get(rs1);
        let rs2 = self.regs.get(rs2);
        self.regs.set(rd, rs1 & rs2);
        self.pc += 4;
    }

    fn or(&mut self, rs1: u8, rs2: u8, rd: u8) {
        let rs1 = self.regs.get(rs1);
        let rs2 = self.regs.get(rs2);
        self.regs.set(rd, rs1 | rs2);
        self.pc += 4;
    }

    fn xor(&mut self, rs1: u8, rs2: u8, rd: u8) {
        let rs1 = self.regs.get(rs1);
        let rs2 = self.regs.get(rs2);
        self.regs.set(rd, rs1 ^ rs2);
        self.pc += 4;
    }

    fn csrrw(&mut self, rs1: u8, imm: u32, rd: u8) {
        let value = self.csr.read(imm);
        self.csr.write(imm, self.regs.get(rs1));
        self.regs.set(rd, value);
        self.pc += 4;
    }

    fn csrrs(&mut self, rs1: u8, imm: u32, rd: u8) {
        let value = self.csr.read(imm);
        self.csr.write(imm, self.csr.read(imm) | self.regs.get(rs1));
        self.regs.set(rd, value);
        self.pc += 4;
    }

    fn csrrc(&mut self, rs1: u8, imm: u32, rd: u8) {
        let value = self.csr.read(imm);
        self.csr
            .write(imm, self.csr.read(imm) & !self.regs.get(rs1));
        self.regs.set(rd, value);
        self.pc += 4;
    }

    fn csrrwi(&mut self, _uimm: u8, _imm: u32, _rd: u8) {
        // WARNING: Need to research what the dtekv chip does when running into this instruction
        panic!("csrrwi not implemented");
        // let value = self.csr.read(imm);
        // self.csr.write(imm, uimm as u32);
        // self.regs.set(rd, value);
        // self.pc += 4;
    }

    fn csrrsi(&mut self, uimm: u8, imm: u32, rd: u8) {
        let value = self.csr.read(imm);
        // NOTE: Dtekv differs from risc-v here:
        self.csr
            .write(imm, self.csr.read(imm) | (1 << (uimm as u32)));
        self.regs.set(rd, value);
        self.pc += 4;
    }

    fn csrrci(&mut self, _uimm: u8, _imm: u32, _rd: u8) {
        // WARNING: Need to research what the dtekv chip does when running into this instruction
        panic!("csrrci not implemented");
        // let value = self.csr.read(imm);
        // self.csr.write(imm, self.csr.read(imm) & !(uimm as u32));
        // self.regs.set(rd, value);
        // self.pc += 4;
    }

    fn sret(&mut self) {
        // TODO: Support supervisor mode
        unimplemented!("Supervisor mode not implemented");
    }

    fn mret(&mut self) {
        self.pc = self.csr.read(csr::MEPC);
        self.csr.set_mstatus_mie(self.csr.get_mstatus_mpie());
        self.csr.set_mstatus_mpie(true);
    }

    fn ecall(&mut self) {
        self.pc += 4;
        self.interrupt(exception::ENVIRONMENT_CALL_FROM_M_MODE)
    }

    fn mul(&mut self, rs1: u8, rs2: u8, rd: u8) {
        let rs1 = self.regs.get(rs1) as i32 as i64;
        let rs2 = self.regs.get(rs2) as i32 as i64;
        self.regs.set(rd, rs1.wrapping_mul(rs2) as u32);
        self.pc += 4;
    }

    fn mulh(&mut self, rs1: u8, rs2: u8, rd: u8) {
        let rs1 = self.regs.get(rs1) as i32 as i64;
        let rs2 = self.regs.get(rs2) as i32 as i64;
        let result = rs1.wrapping_mul(rs2);
        self.regs.set(rd, (result >> 32) as u32);
        self.pc += 4;
    }

    fn mulhu(&mut self, rs1: u8, rs2: u8, rd: u8) {
        let rs1 = self.regs.get(rs1) as u32 as u64;
        let rs2 = self.regs.get(rs2) as u32 as u64;
        let result = rs1.wrapping_mul(rs2);
        self.regs.set(rd, (result >> 32) as u32);
        self.pc += 4;
    }

    fn mulhsu(&mut self, rs1: u8, rs2: u8, rd: u8) {
        let rs1 = self.regs.get(rs1) as i32 as i64;
        let rs2 = self.regs.get(rs2) as u32 as i64;
        let result = rs1.wrapping_mul(rs2);
        self.regs.set(rd, (result >> 32) as u32);
        self.pc += 4;
    }

    fn div(&mut self, rs1: u8, rs2: u8, rd: u8) {
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
    }

    fn divu(&mut self, rs1: u8, rs2: u8, rd: u8) {
        let rs1 = self.regs.get(rs1);
        let rs2 = self.regs.get(rs2);
        if rs2 == 0 {
            self.regs.set(rd, 0xFFFFFFFF);
        } else {
            self.regs.set(rd, rs1 / rs2);
        }
        self.pc += 4;
    }

    fn rem(&mut self, rs1: u8, rs2: u8, rd: u8) {
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
    }

    fn remu(&mut self, rs1: u8, rs2: u8, rd: u8) {
        let rs1 = self.regs.get(rs1);
        let rs2 = self.regs.get(rs2);
        if rs2 == 0 {
            self.regs.set(rd, rs1);
        } else {
            self.regs.set(rd, rs1 % rs2);
        }
        self.pc += 4;
    }

    fn fetch_instruction(&self) -> Result<Instruction, u32> {
        if (self.pc & 3) != 0 {
            return Err(exception::INSTRUCTION_ADDRESS_MISALIGN);
        }
        self.bus
            .load_word(self.pc)
            .try_into()
            .map_err(|_| exception::ILLEGAL_INSTRUCTION)
    }

    fn exec_instruction(&mut self, instruction: Instruction) {
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
            I::SRET => self.sret(),
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
        self.csr.write(csr::MEPC, exception_pc);
        self.csr.write(csr::MCAUSE, cause);
        self.csr.set_mstatus_mpie(self.csr.get_mstatus_mie());
        self.csr.set_mstatus_mie(false);

        if cause & 0x80000000 != 0 {
            // When it's an external interrupt, we need to increment the MEPC by 4
            let exception_pc = self.csr.read(csr::MEPC);
            let exception_pc = exception_pc.wrapping_add(4);
            self.csr.write(csr::MEPC, exception_pc);
        }
    }

    pub(crate) fn external_interrupt(&mut self, cause: u32) {
        if self.csr.read(csr::MIE) & (1 << cause) == 0 {
            // This interrupt is disabled
            return;
        }

        let cause = cause | 0x80000000;
        self.interrupt(cause);
    }

    pub fn external_interrupt_switch(&mut self) {
        self.external_interrupt(exception::SWITCH);
    }

    pub fn external_interrupt_button(&mut self) {
        self.external_interrupt(exception::BUTTON);
    }

    pub fn external_interrupt_timer(&mut self) {
        self.external_interrupt(exception::TIMER);
    }

    pub fn clock(&mut self) {
        let instr: Result<Instruction, u32> = self.fetch_instruction();

        if let Ok(instr) = instr {
            self.exec_instruction(instr);
        } else {
            self.interrupt(instr.unwrap_err());
        }
    }
}

#[cfg(test)]
mod tests {
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
        });
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
        });
        assert_eq!(cpu.regs.get(1), 0x43000000);
        assert_eq!(cpu.pc, 0x40000004);
    }

    #[test]
    fn test_jal() {
        let mut cpu = Cpu::new();

        cpu.pc = 0x40000000;
        cpu.exec_instruction(Instruction::JAL { imm: 0x1000, rd: 1 });
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
        });
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
        });
        assert_eq!(cpu.pc, 0x1000);
        cpu.regs.set(2, 0x1235);
        cpu.exec_instruction(Instruction::BEQ {
            rs1: 1,
            rs2: 2,
            imm: 0x1000,
        });
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
        });
        assert_eq!(cpu.pc, 4);
        cpu.regs.set(2, 0x1235);
        cpu.exec_instruction(Instruction::BNE {
            rs1: 1,
            rs2: 2,
            imm: 0x1000,
        });
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
            });
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
            });
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
            });
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
            });
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
            cpu.exec_instruction(Instruction::ADDI { rs1: 1, imm, rd: 2 });
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
            cpu.exec_instruction(Instruction::SLTI { rs1: 1, imm, rd: 2 });
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
            cpu.exec_instruction(Instruction::SLTIU { rs1: 1, imm, rd: 2 });
            assert_eq!(cpu.regs.get(2), expected);
            assert_eq!(cpu.pc, 4);
        }
    }

    #[test]
    fn test_all_alu_imm() {
        let mut cpu = Cpu::new();

        cpu.exec_instruction(0x00700293.try_into().unwrap()); // addi  x5,  zero, 7
        assert_eq!(cpu.regs.get(5), 7);
        cpu.exec_instruction(0x0052f393.try_into().unwrap()); // andi  x7,  x5, 5
        assert_eq!(cpu.regs.get(7), 5);
        cpu.exec_instruction(0x0052e413.try_into().unwrap()); // ori   x8,  x5, 5
        assert_eq!(cpu.regs.get(8), 7);
        cpu.exec_instruction(0x0052c493.try_into().unwrap()); // xori  x9,  x5, 5
        assert_eq!(cpu.regs.get(9), 2);
        cpu.exec_instruction(0x00229513.try_into().unwrap()); // slli  x10, x5, 2
        assert_eq!(cpu.regs.get(10), 28);
        cpu.exec_instruction(0x0022d593.try_into().unwrap()); // srli  x11, x5, 2
        assert_eq!(cpu.regs.get(11), 1);
        cpu.exec_instruction(0x4022d613.try_into().unwrap()); // srai  x12, x5, 2
        assert_eq!(cpu.regs.get(12), 1);
        cpu.exec_instruction(0x0052a693.try_into().unwrap()); // slti  x13, x5, 5
        assert_eq!(cpu.regs.get(13), 0);
        cpu.exec_instruction(0x0052b713.try_into().unwrap()); // sltiu x14, x5, 5
        assert_eq!(cpu.regs.get(14), 0);
    }

    #[test]
    fn test_srli_sali() {
        let mut cpu = Cpu::new();

        cpu.exec_instruction(0xfff00093.try_into().unwrap()); // li x1, -1
        cpu.exec_instruction(0x0020d113.try_into().unwrap()); // srli  x2, x1, 2
        assert_eq!(cpu.regs.get(2), 0x3fffffff);
        cpu.exec_instruction(0x4020d113.try_into().unwrap()); // srai  x2, x1, 2
        assert_eq!(cpu.regs.get(2), 0xffffffff);
    }

    #[test]
    fn test_slti_sltiu() {
        let mut cpu = Cpu::new();

        cpu.exec_instruction(0xfff00093.try_into().unwrap()); // li x1, -1
        cpu.exec_instruction(0x0000a113.try_into().unwrap()); // slti x2, x1, 0
        assert_eq!(cpu.regs.get(2), 1);
        cpu.exec_instruction(0x0000b193.try_into().unwrap()); // sltiu x3, x1, 0
        assert_eq!(cpu.regs.get(3), 0);
    }

    #[test]
    fn test_load_and_save() {
        let mut cpu = Cpu::new();

        cpu.exec_instruction(0x361880b7.try_into().unwrap()); // lui x1, 0x36188
        cpu.exec_instruction(0x71908093.try_into().unwrap()); // addi x1, x1, 1817 # 0x36188719
        assert_eq!(cpu.regs.get(1), 0x36188719);
        cpu.exec_instruction(0x00102023.try_into().unwrap()); // sw x1, 0(x0)
        assert_eq!(cpu.bus.load_word(0), 0x36188719);
        cpu.exec_instruction(0x00000103.try_into().unwrap()); // lb x2, 0(x0)
        assert_eq!(cpu.regs.get(2), 0x19);
        cpu.exec_instruction(0x00100103.try_into().unwrap()); // lb x2, 1(x0)
        assert_eq!(cpu.regs.get(2), (-121i32) as u32);
        cpu.exec_instruction(0x00200103.try_into().unwrap()); // lb x2, 2(x0)
        assert_eq!(cpu.regs.get(2), 0x18);
        cpu.exec_instruction(0x00300103.try_into().unwrap()); // lb x2, 3(x0)
        assert_eq!(cpu.regs.get(2), 0x36);
        cpu.exec_instruction(0x00001103.try_into().unwrap()); // lh x2, 0(x0)
        assert_eq!(cpu.regs.get(2), (-30951i32) as u32);
        cpu.exec_instruction(0x00101103.try_into().unwrap()); // lh x2, 1(x0)
        assert_eq!(cpu.regs.get(2), 0x1887);
        cpu.exec_instruction(0x00004103.try_into().unwrap()); // lbu x2, 0(x0)
        assert_eq!(cpu.regs.get(2), 0x19);
        cpu.exec_instruction(0x00104103.try_into().unwrap()); // lbu x2, 1(x0)
        assert_eq!(cpu.regs.get(2), 0x87);
        cpu.exec_instruction(0x00204103.try_into().unwrap()); // lbu x2, 2(x0)
        assert_eq!(cpu.regs.get(2), 0x18);
        cpu.exec_instruction(0x00304103.try_into().unwrap()); // lbu x2, 3(x0)
        assert_eq!(cpu.regs.get(2), 0x36);
        cpu.exec_instruction(0x00005103.try_into().unwrap()); // lhu x2, 0(x0)
        assert_eq!(cpu.regs.get(2), 0x8719);
        cpu.exec_instruction(0x00105103.try_into().unwrap()); // lhu x2, 1(x0)
        assert_eq!(cpu.regs.get(2), 0x1887);
    }
}

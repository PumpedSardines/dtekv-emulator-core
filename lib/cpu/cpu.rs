#[cfg(feature = "debug-console")]
use crate::debug_console::DebugConsole;
use crate::{
    cpu::{self, Csr, Regs},
    exception,
    instruction::Instruction,
    io,
    io::Data,
    io::SDRAM_SIZE,
};

// TODO: Implement different instruction speed depending on IO device
pub const CLOCK_FEQ: u32 = 30_000_000;

#[derive(Debug)]
pub struct Cpu<T: io::Data<()>> {
    /// Data line struct that allows the CPU to communicate to memory and IO devices
    pub bus: T,
    /// Every time an instruction is fetched we store it into this vector
    /// Instead of fetching it again we can just use the instruction from the cache
    instruction_cache: Vec<Option<Instruction>>,
    #[cfg(feature = "debug-console")]
    pub debug_console: DebugConsole,
    pub regs: Regs,
    pub csr: Csr,
    pub pc: u32,
}

impl<T: io::Data<()>> Cpu<T> {
    pub fn new_with_bus(bus: T) -> Cpu<T> {
        Cpu {
            bus,
            debug_console: DebugConsole::new(),
            regs: Regs::new(),
            instruction_cache: vec![None; SDRAM_SIZE / 4],
            csr: Csr::new(),
            pc: 0,
        }
    }

    pub fn reset(&mut self) {
        self.regs.reset();
        self.csr.reset();
        self.pc = 4;
        // NOTE: Not sure if this happens when reset is triggered:
        self.csr.set_mstatus_mie(true);
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

    fn csrrw(&mut self, rs1: u8, imm: u32, rd: u8) {
        #[cfg(feature = "debug")]
        if !self.csr.emulated_csr(imm) {
            self.debug_output.push(DebugOutputLine::AccessUselessCsr {
                csr: imm,
                instr_addr: self.pc,
            });
        }

        let value = self.csr.load(imm);
        self.csr.store(imm, self.regs.get(rs1));
        self.regs.set(rd, value);
        self.pc += 4;
    }

    fn csrrs(&mut self, rs1: u8, imm: u32, rd: u8) {
        #[cfg(feature = "debug")]
        if !self.csr.emulated_csr(imm) {
            self.debug_output.push(DebugOutputLine::AccessUselessCsr {
                csr: imm,
                instr_addr: self.pc,
            });
        }

        let value = self.csr.load(imm);
        self.csr.store(imm, self.csr.load(imm) | self.regs.get(rs1));
        self.regs.set(rd, value);
        self.pc += 4;
    }

    fn csrrc(&mut self, rs1: u8, imm: u32, rd: u8) {
        #[cfg(feature = "debug")]
        if !self.csr.emulated_csr(imm) {
            self.debug_output.push(DebugOutputLine::AccessUselessCsr {
                csr: imm,
                instr_addr: self.pc,
            });
        }

        let value = self.csr.load(imm);
        self.csr
            .store(imm, self.csr.load(imm) & !self.regs.get(rs1));
        self.regs.set(rd, value);
        self.pc += 4;
    }

    fn csrrwi(&mut self, _uimm: u8, _imm: u32, _rd: u8) {
        #[cfg(feature = "debug")]
        self.debug_output
            .push(DebugOutputLine::InstructionNotImplemented {
                instr: "CSRRWI",
                instr_addr: self.pc,
            });
        self.pc += 4;
    }

    fn csrrsi(&mut self, uimm: u8, imm: u32, rd: u8) {
        #[cfg(feature = "debug")]
        if !self.csr.emulated_csr(imm) {
            self.debug_output.push(DebugOutputLine::AccessUselessCsr {
                csr: imm,
                instr_addr: self.pc,
            });
        }

        let value = self.csr.load(imm);

        // NOTE: Dtekv differs from risc-v here:
        self.csr
            .store(imm, self.csr.load(imm) | (1 << (uimm as u32)));
        self.regs.set(rd, value);
        self.pc += 4;
    }

    fn csrrci(&mut self, _uimm: u8, _imm: u32, _rd: u8) {
        #[cfg(feature = "debug")]
        self.debug_output
            .push(DebugOutputLine::InstructionNotImplemented {
                instr: "CSRRCI",
                instr_addr: self.pc,
            });
        self.pc += 4;
    }

    fn mret(&mut self) {
        self.pc = self.csr.load(cpu::MEPC);
        self.csr.set_mstatus_mie(self.csr.get_mstatus_mpie());
        self.csr.set_mstatus_mpie(true);
    }

    fn ecall(&mut self) {
        self.pc += 4;
        self.interrupt(exception::ENVIRONMENT_CALL_FROM_M_MODE);
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
            #[cfg(feature = "debug")]
            self.debug_output.push(DebugOutputLine::DivisionByZero {
                instr_addr: self.pc,
            });

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
            #[cfg(feature = "debug")]
            self.debug_output.push(DebugOutputLine::DivisionByZero {
                instr_addr: self.pc,
            });

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
            #[cfg(feature = "debug")]
            self.debug_output.push(DebugOutputLine::RemainderByZero {
                instr_addr: self.pc,
            });

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
            #[cfg(feature = "debug")]
            self.debug_output.push(DebugOutputLine::RemainderByZero {
                instr_addr: self.pc,
            });

            self.regs.set(rd, rs1);
        } else {
            self.regs.set(rd, rs1 % rs2);
        }
        self.pc += 4;
    }

    pub fn clear_instruction_cache(&mut self, addr: u32) {
        let addr = addr / 4;

        if addr < self.instruction_cache.len() as u32 {
            self.instruction_cache[addr as usize] = None;
        }
    }

    pub fn update_instruction_cache(&mut self, addr: u32) {
        let instruction: Option<Instruction> = self
            .load_word(addr)
            .map(|v| v.try_into().ok())
            .ok()
            .flatten();
        let addr = addr / 4;
        self.instruction_cache[addr as usize] = instruction;
    }

    pub fn generate_instruction_cache(&mut self) {
        for i in 0..(SDRAM_SIZE / 4) {
            let addr = i as u32;
            let instruction = self
                .bus
                .load_word(addr * 4)
                .map(|word| word.try_into().ok())
                .ok()
                .flatten();

            if let Some(instruction) = instruction {
                self.instruction_cache[i] = Some(instruction);
            }
        }
    }

    fn fetch_instruction(&mut self) -> Result<Instruction, u32> {
        if (self.pc & 3) != 0 {
            #[cfg(feature = "debug")]
            self.debug_output
                .push(DebugOutputLine::InstructionMisaligned {
                    instr_addr: self.pc,
                });

            return Err(exception::INSTRUCTION_ADDRESS_MISALIGNED);
        }

        let cache_index = self.pc / 4;
        let can_cache = cache_index < self.instruction_cache.len() as u32;

        if can_cache {
            if let Some(instruction) = self.instruction_cache[cache_index as usize] {
                return Ok(instruction);
            }
        }

        let instruction = self
            .bus
            .load_word(self.pc)
            .and_then(|word| word.try_into())
            .map_err(|_| exception::ILLEGAL_INSTRUCTION);

        #[cfg(feature = "debug-console")]
        if instruction.is_err() {
            self.debug_output.push(DebugOutputLine::IllegalInstruction {
                instr: self.bus.load_word(self.pc).unwrap_or(0),
                instr_addr: self.pc,
            });
        }

        if can_cache {
            if let Ok(instruction) = instruction {
                self.instruction_cache[cache_index as usize] = Some(instruction);
            }
        }

        instruction
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

    /// Sends a interrupt signal to the CPU with a given cause
    /// This only triggers an interrupt if the Cpu is ready to receive interrupts
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

    /// Sends an external interrupt to the CPU
    pub fn external_interrupt(&mut self, cause: u32) {
        if self.csr.load(cpu::MIE) & (1 << cause) == 0 {
            // This interrupt is disabled
            return;
        }

        let cause = cause | 0x80000000;
        self.interrupt(cause);
    }

    pub fn clock(&mut self) {
        let instr: Result<Instruction, u32> = self.fetch_instruction();

        match instr {
            Ok(instr) => {
                self.exec_instruction(instr);
            }
            Err(exception) => {
                self.interrupt(exception);
            }
        }
    }
}

impl<T> io::Data<()> for Cpu<T>
where
    T: io::Data<()>,
{
    fn load_byte(&self, addr: u32) -> Result<u8, ()> {
        self.bus.load_byte(addr)
    }

    fn store_byte(&mut self, addr: u32, byte: u8) -> Result<(), ()> {
        self.clear_instruction_cache(addr);
        self.bus.store_byte(addr, byte)
    }

    fn load_halfword(&self, addr: u32) -> Result<u16, ()> {
        self.bus.load_halfword(addr)
    }

    fn store_halfword(&mut self, addr: u32, halfword: u16) -> Result<(), ()> {
        self.clear_instruction_cache(addr);
        self.bus.store_halfword(addr, halfword)
    }

    fn load_word(&self, addr: u32) -> Result<u32, ()> {
        self.bus.load_word(addr)
    }

    fn store_word(&mut self, addr: u32, word: u32) -> Result<(), ()> {
        self.clear_instruction_cache(addr);
        self.bus.store_word(addr, word)
    }

    fn store_at<K: Into<u8>, R: IntoIterator<Item = K>>(
        &mut self,
        offset: u32,
        bin: R,
    ) -> Result<(), ()>
    where
        Self: Sized,
    {
        for (i, byte) in bin.into_iter().enumerate() {
            let i = i as u32;
            self.store_byte(offset + i, byte.into())?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::io;
    use crate::test_utils::*;

    #[test]
    fn test_lui() {
        let mut cpu = new_panic_io_cpu();

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
        let mut cpu = new_cpu();

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
        let mut cpu = new_cpu();

        cpu.pc = 0x40000000;
        cpu.exec_instruction(Instruction::JAL { imm: 0x1000, rd: 1 });
        assert_eq!(cpu.regs.get(1), 0x40000004);
        assert_eq!(cpu.pc, 0x40001000);
    }

    #[test]
    fn test_jalr() {
        let mut cpu = new_cpu();

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
        let mut cpu = new_cpu();

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
        let mut cpu = new_cpu();

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
            let mut cpu = new_cpu();

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
            let mut cpu = new_cpu();

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
            let mut cpu = new_cpu();

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
            let mut cpu = new_cpu();

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
    fn test_all_alu_imm() {
        let mut cpu = new_cpu().cpu;

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
        let mut cpu = new_cpu();

        cpu.exec_instruction(0xfff00093.try_into().unwrap()); // li x1, -1
        cpu.exec_instruction(0x0020d113.try_into().unwrap()); // srli  x2, x1, 2
        assert_eq!(cpu.regs.get(2), 0x3fffffff);
        cpu.exec_instruction(0x4020d113.try_into().unwrap()); // srai  x2, x1, 2
        assert_eq!(cpu.regs.get(2), 0xffffffff);
    }

    #[test]
    fn test_slti_sltiu() {
        let mut cpu = new_cpu();

        cpu.exec_instruction(0xfff00093.try_into().unwrap()); // li x1, -1
        cpu.exec_instruction(0x0000a113.try_into().unwrap()); // slti x2, x1, 0
        assert_eq!(cpu.regs.get(2), 1);
        cpu.exec_instruction(0x0000b193.try_into().unwrap()); // sltiu x3, x1, 0
        assert_eq!(cpu.regs.get(3), 0);
    }

}

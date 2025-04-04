//! The CPU module
//!
//! IMPORTANT: After creating the Cpu you should access memory through the Cpu's implementation of
//! Data. This is because the Cpu does some extra caching logic that speeds up the emulator.
//! Otherwise the cache might get out of sync with the memory

use std::{cell::RefCell, rc::Rc};

#[cfg(feature = "debug-console")]
use crate::debug_console::DebugConsole;
use crate::{
    csr::{Csr, CsrBlock},
    instruction::Instruction,
    interrupt::InterruptSignal,
    memory_mapped::MemoryMapped,
    peripheral::{Peripheral, SDRAM_SIZE},
    register::RegisterBlock,
};

mod instructions;

pub const CLOCK_FEQ: u32 = 30_000_000;

#[derive(Debug)]
pub struct Cpu<T: Peripheral<()>> {
    /// Data line struct that allows the CPU to communicate to memory and IO devices
    pub bus: T,
    /// Every time an instruction is fetched we store it into this vector
    /// Instead of fetching it again we can just use the instruction from the cache
    instruction_cache: Vec<Option<Instruction>>,
    #[cfg(feature = "debug-console")]
    debug_console: Option<Rc<RefCell<DebugConsole>>>,
    pub regs: RegisterBlock,
    pub csr: CsrBlock,
    pub pc: u32,
}

impl<T: Peripheral<()>> Cpu<T> {
    pub fn new_with_bus(bus: T) -> Cpu<T> {
        Cpu {
            bus,
            #[cfg(feature = "debug-console")]
            debug_console: None,
            regs: RegisterBlock::new(),
            instruction_cache: vec![None; SDRAM_SIZE / 4],
            csr: CsrBlock::new(),
            pc: 0,
        }
    }

    #[cfg(feature = "debug-console")]
    pub fn with_debug_console(mut self, debug_console: Rc<RefCell<DebugConsole>>) -> Self {
        self.debug_console = Some(debug_console);
        self
    }

    /// Sends a reset signal to the CPU, the same as pressing the reset button on the DTEK-V board
    pub fn reset(&mut self) {
        self.regs.reset();
        self.csr.reset();
        self.pc = 4;
        // NOTE: Not sure if this happens when reset is triggered:
        self.csr.set_mstatus_mie(true);
    }

    pub fn clear_instruction_cache(&mut self, addr: u32) {
        let addr = addr / 4;

        if addr < self.instruction_cache.len() as u32 {
            self.instruction_cache[addr as usize] = None;
        }
    }

    pub fn update_instruction_cache(&mut self, addr: u32) {
        let instruction: Option<Instruction> =
            self.load_word(addr).ok().and_then(|v| v.try_into().ok());
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

    fn fetch_instruction(&mut self) -> Result<Instruction, InterruptSignal> {
        if (self.pc & 3) != 0 {
            if let Some(db) = &self.debug_console {
                db.borrow_mut().instruction_misaligned(self.pc);
            }
            return Err(InterruptSignal::INSTRUCTION_ADDRESS_MISALIGNED);
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
            .map_err(|_| {
                #[cfg(feature = "debug-console")]
                if let Some(db) = &self.debug_console {
                    db.borrow_mut()
                        .illegal_instruction(self.bus.load_word(self.pc).unwrap_or(0), self.pc);
                }

                InterruptSignal::ILLEGAL_INSTRUCTION
            });

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
            I::CSRRW { rs1, csr, rd } => self.csrrw(rs1, csr, rd),
            I::CSRRS { rs1, csr, rd } => self.csrrs(rs1, csr, rd),
            I::CSRRC { rs1, csr, rd } => self.csrrc(rs1, csr, rd),
            I::CSRRWI { imm, csr, rd } => self.csrrwi(imm, csr, rd),
            I::CSRRSI { imm, csr, rd } => self.csrrsi(imm, csr, rd),
            I::CSRRCI { imm, csr, rd } => self.csrrci(imm, csr, rd),
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

    /// Sends a interrupt signal to the CPU
    pub fn handle_interrupt(&mut self, exception: InterruptSignal) {
        // If interrupts are disabled, ignore the interrupt
        if !self.csr.get_mstatus_mie() {
            return;
        }

        if exception.external() && self.csr.load(Csr::MIE) & (1 << exception.cause()) == 0 {
            // This interrupt is disabled
            return;
        }

        let exception_pc = self.pc.wrapping_sub(4);
        self.pc = 0;
        self.csr.store(Csr::MEPC, exception_pc);
        self.csr.store(Csr::MCAUSE, exception.cause());
        self.csr.set_mstatus_mpie(self.csr.get_mstatus_mie());
        self.csr.set_mstatus_mie(false);

        if exception.external() {
            // When it's an external interrupt, we need to increment the MEPC by 4
            let exception_pc = self.csr.load(Csr::MEPC);
            let exception_pc = exception_pc.wrapping_add(4);
            self.csr.store(Csr::MEPC, exception_pc);
        }
    }

    pub fn clock(&mut self) {
        let instr: Result<Instruction, InterruptSignal> = self.fetch_instruction();

        match instr {
            Ok(instr) => {
                self.exec_instruction(instr);
            }
            Err(exception) => {
                self.handle_interrupt(exception);
            }
        }
    }
}

impl<T> MemoryMapped<()> for Cpu<T>
where
    T: Peripheral<()>,
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::peripheral;
    use crate::register::Register;
    use crate::test_utils::*;

    #[test]
    fn test_all_alu_imm() {
        let mut cpu = new_panic_io_cpu();
        cpu.exec_instruction(0x00700293.try_into().unwrap()); // addi  x5,  zero, 7
        assert_eq!(cpu.regs.get(Register::T0), 7);
        cpu.exec_instruction(0x0052f393.try_into().unwrap()); // andi  x7,  x5, 5
        assert_eq!(cpu.regs.get(Register::T2), 5);
        cpu.exec_instruction(0x0052e413.try_into().unwrap()); // ori   x8,  x5, 5
        assert_eq!(cpu.regs.get(Register::S0), 7);
        cpu.exec_instruction(0x0052c493.try_into().unwrap()); // xori  x9,  x5, 5
        assert_eq!(cpu.regs.get(Register::S1), 2);
        cpu.exec_instruction(0x00229513.try_into().unwrap()); // slli  x10, x5, 2
        assert_eq!(cpu.regs.get(Register::A0), 28);
        cpu.exec_instruction(0x0022d593.try_into().unwrap()); // srli  x11, x5, 2
        assert_eq!(cpu.regs.get(Register::A1), 1);
        cpu.exec_instruction(0x4022d613.try_into().unwrap()); // srai  x12, x5, 2
        assert_eq!(cpu.regs.get(Register::A2), 1);
        cpu.exec_instruction(0x0052a693.try_into().unwrap()); // slti  x13, x5, 5
        assert_eq!(cpu.regs.get(Register::A3), 0);
        cpu.exec_instruction(0x0052b713.try_into().unwrap()); // sltiu x14, x5, 5
        assert_eq!(cpu.regs.get(Register::A4), 0);
    }

    #[test]
    fn test_srli_sali() {
        let mut cpu = new_panic_io_cpu();

        cpu.exec_instruction(0xfff00093.try_into().unwrap()); // li x1, -1
        cpu.exec_instruction(0x0020d113.try_into().unwrap()); // srli  x2, x1, 2
        assert_eq!(cpu.regs.get(Register::SP), 0x3fffffff);
        cpu.exec_instruction(0x4020d113.try_into().unwrap()); // srai  x2, x1, 2
        assert_eq!(cpu.regs.get(Register::SP), 0xffffffff);
    }

    #[test]
    fn test_slti_sltiu() {
        let mut cpu = new_panic_io_cpu();

        cpu.exec_instruction(0xfff00093.try_into().unwrap()); // li x1, -1
        cpu.exec_instruction(0x0000a113.try_into().unwrap()); // slti x2, x1, 0
        assert_eq!(cpu.regs.get(Register::SP), 1);
        cpu.exec_instruction(0x0000b193.try_into().unwrap()); // sltiu x3, x1, 0
        assert_eq!(cpu.regs.get(Register::GP), 0);
    }

    #[test]
    fn test_load_and_save() {
        let sdram = peripheral::SDRam::new();
        let mut cpu = Cpu::new_with_bus(sdram);

        cpu.exec_instruction(0x361880b7.try_into().unwrap()); // lui x1, 0x36188
        cpu.exec_instruction(0x71908093.try_into().unwrap()); // addi x1, x1, 1817 # 0x36188719
        assert_eq!(cpu.regs.get(Register::RA), 0x36188719);
        cpu.exec_instruction(0x00102023.try_into().unwrap()); // sw x1, 0(x0)
        assert_eq!(cpu.bus.load_word(0).unwrap(), 0x36188719);
        cpu.exec_instruction(0x00000103.try_into().unwrap()); // lb x2, 0(x0)
        assert_eq!(cpu.regs.get(Register::SP), 0x19);
        cpu.exec_instruction(0x00100103.try_into().unwrap()); // lb x2, 1(x0)
        assert_eq!(cpu.regs.get(Register::SP), (-121i32) as u32);
        cpu.exec_instruction(0x00200103.try_into().unwrap()); // lb x2, 2(x0)
        assert_eq!(cpu.regs.get(Register::SP), 0x18);
        cpu.exec_instruction(0x00300103.try_into().unwrap()); // lb x2, 3(x0)
        assert_eq!(cpu.regs.get(Register::SP), 0x36);
        cpu.exec_instruction(0x00001103.try_into().unwrap()); // lh x2, 0(x0)
        assert_eq!(cpu.regs.get(Register::SP), (-30951i32) as u32);
        cpu.exec_instruction(0x00101103.try_into().unwrap()); // lh x2, 1(x0)
        assert_eq!(cpu.regs.get(Register::SP), 0x1887);
        cpu.exec_instruction(0x00004103.try_into().unwrap()); // lbu x2, 0(x0)
        assert_eq!(cpu.regs.get(Register::SP), 0x19);
        cpu.exec_instruction(0x00104103.try_into().unwrap()); // lbu x2, 1(x0)
        assert_eq!(cpu.regs.get(Register::SP), 0x87);
        cpu.exec_instruction(0x00204103.try_into().unwrap()); // lbu x2, 2(x0)
        assert_eq!(cpu.regs.get(Register::SP), 0x18);
        cpu.exec_instruction(0x00304103.try_into().unwrap()); // lbu x2, 3(x0)
        assert_eq!(cpu.regs.get(Register::SP), 0x36);
        cpu.exec_instruction(0x00005103.try_into().unwrap()); // lhu x2, 0(x0)
        assert_eq!(cpu.regs.get(Register::SP), 0x8719);
        cpu.exec_instruction(0x00105103.try_into().unwrap()); // lhu x2, 1(x0)
        assert_eq!(cpu.regs.get(Register::SP), 0x1887);
    }
}

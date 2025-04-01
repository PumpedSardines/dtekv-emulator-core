use crate::{
    cpu::Cpu,
    instruction::{ITypeImm, STypeImm},
    memory_mapped::MemoryMapped,
    peripheral::Peripheral,
    register::Register,
};

fn debug_console_load_oob<T: Peripheral<()>>(cpu: &mut Cpu<T>, addr: u32) {
    #[cfg(feature = "debug-console")]
    if let Some(db) = &cpu.debug_console {
        db.borrow_mut().load_out_of_bounds(addr, cpu.pc);
    }
}

fn debug_console_store_oob<T: Peripheral<()>>(cpu: &mut Cpu<T>, addr: u32) {
    #[cfg(feature = "debug-console")]
    if let Some(db) = &cpu.debug_console {
        db.borrow_mut().store_out_of_bounds(addr, cpu.pc);
    }
}

impl<T: Peripheral<()>> Cpu<T> {
    pub(crate) fn lb(&mut self, rs1: Register, imm: ITypeImm, rd: Register) {
        let imm = imm.as_u32();
        let rs1 = self.regs.get(rs1);
        let addr = rs1.wrapping_add(imm);

        let byte = self.load_byte(addr).unwrap_or_else(|_| {
            debug_console_load_oob(self, addr);

            0xDE
        }) as i8 as i32 as u32;

        self.regs.set(rd, byte);
        self.pc += 4;
    }

    pub(crate) fn lh(&mut self, rs1: Register, imm: ITypeImm, rd: Register) {
        let imm = imm.as_u32();
        let rs1 = self.regs.get(rs1);
        let addr = rs1.wrapping_add(imm);

        let halfword = self.load_halfword(addr).unwrap_or_else(|_| {
            debug_console_load_oob(self, addr);

            0xDEAD
        }) as i16 as i32 as u32;

        self.regs.set(rd, halfword);
        self.pc += 4;
    }

    pub(crate) fn lw(&mut self, rs1: Register, imm: ITypeImm, rd: Register) {
        let imm = imm.as_u32();
        let rs1 = self.regs.get(rs1);
        let addr = rs1.wrapping_add(imm);
        let word = self.load_word(addr).unwrap_or_else(|_| {
            debug_console_load_oob(self, addr);

            0xDEAD_BEEF
        });

        self.regs.set(rd, word);
        self.pc += 4;
    }

    pub(crate) fn lbu(&mut self, rs1: Register, imm: ITypeImm, rd: Register) {
        let imm = imm.as_u32();
        let rs1 = self.regs.get(rs1);
        let addr = rs1.wrapping_add(imm);

        let byte = self.load_byte(addr).unwrap_or_else(|_| {
            debug_console_load_oob(self, addr);

            0xDE
        });

        self.regs.set(rd, byte as u32);
        self.pc += 4;
    }

    pub(crate) fn lhu(&mut self, rs1: Register, imm: ITypeImm, rd: Register) {
        let imm = imm.as_u32();
        let rs1 = self.regs.get(rs1);
        let addr = rs1.wrapping_add(imm);

        let halfword = self.load_halfword(addr).unwrap_or_else(|_| {
            debug_console_load_oob(self, addr);

            0xDEAD
        });

        self.regs.set(rd, halfword as u32);
        self.pc += 4;
    }

    pub(crate) fn sb(&mut self, rs1: Register, rs2: Register, imm: STypeImm) {
        let imm = imm.as_u32();
        let rs1 = self.regs.get(rs1);
        let rs2 = self.regs.get(rs2);
        let addr = rs1.wrapping_add(imm);

        if self.store_byte(addr, rs2 as u8).is_err() {
            debug_console_store_oob(self, addr);
        }

        self.pc += 4;
    }

    pub(crate) fn sh(&mut self, rs1: Register, rs2: Register, imm: STypeImm) {
        let imm = imm.as_u32();
        let rs1 = self.regs.get(rs1);
        let rs2 = self.regs.get(rs2);
        let addr = rs1.wrapping_add(imm);

        if self.store_halfword(addr, rs2 as u16).is_err() {
            debug_console_store_oob(self, addr);
        }

        self.pc += 4;
    }

    pub(crate) fn sw(&mut self, rs1: Register, rs2: Register, imm: STypeImm) {
        let imm = imm.as_u32();
        let rs1 = self.regs.get(rs1);
        let rs2 = self.regs.get(rs2);
        let addr = rs1.wrapping_add(imm);

        if self.store_word(addr, rs2).is_err() {
            debug_console_store_oob(self, addr);
        }

        self.pc += 4;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;
    use test_case::test_case;

    /// Helper to create imm value without the boilerplate
    macro_rules! imm {
        ($v:expr) => {
            $v.try_into().unwrap()
        };
    }

    #[test]
    fn test_lb() {
        let mut cpu = new_io_cpu().cpu;

        let data: Vec<(u8, u32)> = vec![(0x83, (-125i32) as u32), (0x12, 18)];

        for (inp, out) in data {
            for i in 0..4 {
                cpu.store_byte(i, inp).unwrap();
                cpu.lb(Register::ZERO, imm!(i), Register::RA);
                assert_eq!(cpu.regs.get(Register::RA), out);
            }
        }
    }

    #[test]
    fn test_lh() {
        let mut cpu = new_io_cpu().cpu;

        let data: Vec<(u16, u32)> = vec![(0x8313, (-31981i32) as u32), (0x1245, 4677)];

        for (inp, out) in data {
            for i in 0..4 {
                cpu.store_halfword(i, inp).unwrap();
                cpu.lh(Register::ZERO, imm!(i), Register::RA);
                assert_eq!(
                    cpu.regs.get(Register::RA),
                    out,
                    "{}",
                    format!("i: {}, inp: {}, out: {}", i, inp, out)
                );
            }
        }
    }

    #[test]
    fn test_lw() {
        let mut cpu = new_io_cpu().cpu;

        let data: Vec<u32> = vec![
            0x12345678, 0x87654321, 0x00000000, 0xFFFFFFFF, 0x0000FFFF, 0xFFFF0000,
        ];

        for v in data {
            for i in 0..4 {
                cpu.store_word(i, v).unwrap();
                cpu.lw(Register::ZERO, imm!(i), Register::RA);
                assert_eq!(
                    cpu.regs.get(Register::RA),
                    v,
                    "{}",
                    format!("i: {}, value: {}", i, v)
                );
            }
        }
    }

    #[test]
    fn test_lbu() {
        let mut cpu = new_io_cpu().cpu;

        let data: Vec<(u8, u32)> = vec![(0x83, 131), (0x12, 18)];

        for (inp, out) in data {
            for i in 0..4 {
                cpu.store_byte(i, inp).unwrap();
                cpu.lbu(Register::ZERO, imm!(i), Register::RA);
                assert_eq!(cpu.regs.get(Register::RA), out);
            }
        }
    }

    #[test]
    fn test_lhu() {
        let mut cpu = new_io_cpu().cpu;

        let data: Vec<(u16, u32)> = vec![(0x8313, 0x8313), (0x1245, 4677)];

        for (inp, out) in data {
            for i in 0..4 {
                cpu.store_halfword(i, inp).unwrap();
                cpu.lhu(Register::ZERO, imm!(i), Register::RA);
                assert_eq!(
                    cpu.regs.get(Register::RA),
                    out,
                    "{}",
                    format!("i: {}, inp: {}, out: {}", i, inp, out)
                );
            }
        }
    }

    struct SbTestData {
        rs1: u32,
        offset: u32,
        value: u8,
        exp_addr: u32,
    }
    #[test_case(SbTestData { rs1: 0, offset: 0, value: 0xf3, exp_addr: 0 } => 0xf3; "store byte")]
    #[test_case(SbTestData { rs1: 0x40, offset: 0, value: 0xf3, exp_addr: 0x40 } => 0xf3; "byte at reg addr")]
    #[test_case(SbTestData { rs1: 0, offset: 0x30, value: 0xf3, exp_addr: 0x30 } => 0xf3; "byte at offset addr")]
    #[test_case(SbTestData { rs1: 0x40, offset: 0x30, value: 0xf3, exp_addr: 0x70 } => 0xf3; "byte at reg addr and offset addr")]
    #[test_case(SbTestData { rs1: 0x1, offset: 0x0, value: 0xf3, exp_addr: 0x1 } => 0xf3; "unaligned addr")]
    fn sb(data: SbTestData) -> u8 {
        let mut s = new_io_cpu();
        let cpu = &mut s.cpu;
        let sdram = &mut s.sdram;

        cpu.regs.set(Register::T0, data.rs1);
        cpu.regs.set(Register::T1, data.value as u32);
        cpu.sb(
            Register::T0,
            Register::T1,
            STypeImm::new(data.offset).unwrap(),
        );
        sdram.load_byte(data.exp_addr).unwrap()
    }

    struct ShTestData {
        rs1: u32,
        offset: u32,
        value: u16,
        exp_addr: u32,
    }
    #[test_case(ShTestData { rs1: 0, offset: 0, value: 0xf3, exp_addr: 0 } => 0xf3; "store byte")]
    #[test_case(ShTestData { rs1: 0x40, offset: 0, value: 0xFFF3, exp_addr: 0x40 } => 0xFFF3; "byte at reg addr")]
    #[test_case(ShTestData { rs1: 0, offset: 0x30, value: 0xf3, exp_addr: 0x30 } => 0xf3; "byte at offset addr")]
    #[test_case(ShTestData { rs1: 0x40, offset: 0x30, value: 0xf3, exp_addr: 0x70 } => 0xf3; "byte at reg addr and offset addr")]
    #[test_case(ShTestData { rs1: 0x1, offset: 0x0, value: 0xf3, exp_addr: 0x1 } => 0xf3; "unaligned addr")]
    fn sh(data: ShTestData) -> u16 {
        let mut s = new_io_cpu();
        let cpu = &mut s.cpu;
        let sdram = &mut s.sdram;

        cpu.regs.set(Register::T0, data.rs1);
        cpu.regs.set(Register::T1, data.value as u32);
        cpu.sh(
            Register::T0,
            Register::T1,
            STypeImm::new(data.offset).unwrap(),
        );
        sdram.load_halfword(data.exp_addr).unwrap()
    }

    struct SwTestData {
        rs1: u32,
        offset: u32,
        value: u32,
        exp_addr: u32,
    }
    #[test_case(SwTestData { rs1: 0, offset: 0, value: 0xf3, exp_addr: 0 } => 0xf3; "store byte")]
    #[test_case(SwTestData { rs1: 0x40, offset: 0, value: 0xFFF3, exp_addr: 0x40 } => 0xFFF3; "byte at reg addr")]
    #[test_case(SwTestData { rs1: 0, offset: 0x30, value: 0xf3, exp_addr: 0x30 } => 0xf3; "byte at offset addr")]
    #[test_case(SwTestData { rs1: 0x40, offset: 0x30, value: 0xf3, exp_addr: 0x70 } => 0xf3; "byte at reg addr and offset addr")]
    #[test_case(SwTestData { rs1: 0x1, offset: 0x0, value: 0xFFFF_FFFF, exp_addr: 0x1 } => 0xFFFF_FFFF; "unaligned addr")]
    fn sw(data: SwTestData) -> u32 {
        let mut s = new_io_cpu();
        let cpu = &mut s.cpu;
        let sdram = &mut s.sdram;

        cpu.regs.set(Register::T0, data.rs1);
        cpu.regs.set(Register::T1, data.value as u32);
        cpu.sw(
            Register::T0,
            Register::T1,
            STypeImm::new(data.offset).unwrap(),
        );
        sdram.load_word(data.exp_addr).unwrap()
    }
}

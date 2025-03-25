use crate::{
    cpu::Cpu,
    io::{self, Data},
    register::Register,
};

impl<T: io::Data<()>> Cpu<T> {
    pub(crate) fn lb(&mut self, rs1: Register, imm: u32, rd: Register) {
        let rs1 = self.regs.get(rs1);
        let addr = rs1.wrapping_add(imm);

        let byte = self.load_byte(addr).unwrap_or_else(|_| {
            #[cfg(feature = "debug-console")]
            self.debug_console.load_out_of_bounds(addr, self.pc);
            0xDE
        }) as i8 as i32 as u32;

        self.regs.set(rd, byte);
        self.pc += 4;
    }

    pub(crate) fn lh(&mut self, rs1: Register, imm: u32, rd: Register) {
        let rs1 = self.regs.get(rs1);
        let addr = rs1.wrapping_add(imm);

        let halfword = self.load_halfword(addr).unwrap_or_else(|_| {
            #[cfg(feature = "debug-console")]
            self.debug_console.load_out_of_bounds(addr, self.pc);
            0xDEAD
        }) as i16 as i32 as u32;

        self.regs.set(rd, halfword);
        self.pc += 4;
    }

    pub(crate) fn lw(&mut self, rs1: Register, imm: u32, rd: Register) {
        let rs1 = self.regs.get(rs1);
        let addr = rs1.wrapping_add(imm);
        let word = self.load_word(addr).unwrap_or_else(|_| {
            #[cfg(feature = "debug-console")]
            self.debug_console.load_out_of_bounds(addr, self.pc);
            0xDEAD_BEEF
        });

        self.regs.set(rd, word);
        self.pc += 4;
    }

    pub(crate) fn lbu(&mut self, rs1: Register, imm: u32, rd: Register) {
        let rs1 = self.regs.get(rs1);
        let addr = rs1.wrapping_add(imm);

        let byte = self.load_byte(addr).unwrap_or_else(|_| {
            #[cfg(feature = "debug-console")]
            self.debug_console.load_out_of_bounds(addr, self.pc);
            0xDE
        });

        self.regs.set(rd, byte as u32);
        self.pc += 4;
    }

    pub(crate) fn lhu(&mut self, rs1: Register, imm: u32, rd: Register) {
        let rs1 = self.regs.get(rs1);
        let addr = rs1.wrapping_add(imm);

        let halfword = self.load_halfword(addr).unwrap_or_else(|_| {
            #[cfg(feature = "debug-console")]
            self.debug_console.load_out_of_bounds(addr, self.pc);
            0xDEAD
        });

        self.regs.set(rd, halfword as u32);
        self.pc += 4;
    }

    pub(crate) fn sb(&mut self, rs1: Register, rs2: Register, imm: u32) {
        let rs1 = self.regs.get(rs1);
        let rs2 = self.regs.get(rs2);
        let addr = rs1.wrapping_add(imm);

        if self.store_byte(addr, rs2 as u8).is_err() {
            #[cfg(feature = "debug-console")]
            self.debug_console.store_out_of_bounds(addr, self.pc);
        }

        self.pc += 4;
    }

    pub(crate) fn sh(&mut self, rs1: Register, rs2: Register, imm: u32) {
        let rs1 = self.regs.get(rs1);
        let rs2 = self.regs.get(rs2);
        let addr = rs1.wrapping_add(imm);

        if self.store_halfword(addr, rs2 as u16).is_err() {
            #[cfg(feature = "debug-console")]
            self.debug_console.store_out_of_bounds(addr, self.pc);
        }

        self.pc += 4;
    }

    pub(crate) fn sw(&mut self, rs1: Register, rs2: Register, imm: u32) {
        let rs1 = self.regs.get(rs1);
        let rs2 = self.regs.get(rs2);
        let addr = rs1.wrapping_add(imm);

        if self.store_word(addr, rs2).is_err() {
            #[cfg(feature = "debug-console")]
            self.debug_console.store_out_of_bounds(addr, self.pc);
        }

        self.pc += 4;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;

    #[test]
    fn test_lb() {
        let mut cpu = new_io_cpu().cpu;

        let data: Vec<(u8, u32)> = vec![(0x83, (-125i32) as u32), (0x12, 18)];

        for (inp, out) in data {
            for i in 0..4 {
                cpu.store_byte(i, inp).unwrap();
                cpu.lb(Register::ZERO, i, Register::RA);
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
                cpu.lh(Register::ZERO, i, Register::RA);
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
                cpu.lw(Register::ZERO, i, Register::RA);
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
                cpu.lbu(Register::ZERO, i, Register::RA);
                assert_eq!(cpu.regs.get(Register::RA), out);
            }
        }
    }

    #[test]
    fn test_lhu() {
        // TODO: Implement test
        todo!();
    }

    #[test]
    fn test_sb() {
        // TODO: Implement test
        todo!();
    }

    #[test]
    fn test_sh() {
        // TODO: Implement test
        todo!();
    }

    #[test]
    fn test_sw() {
        // TODO: Implement test
        todo!();
    }
}

use crate::{cpu::Cpu, io, io::Data};

impl<T: io::Data<()>> Cpu<T> {
    pub(crate) fn lb(&mut self, rs1: u8, imm: u32, rd: u8) {
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

    pub(crate) fn lh(&mut self, rs1: u8, imm: u32, rd: u8) {
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

    pub(crate) fn lw(&mut self, rs1: u8, imm: u32, rd: u8) {
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

    pub(crate) fn lbu(&mut self, rs1: u8, imm: u32, rd: u8) {
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

    pub(crate) fn lhu(&mut self, rs1: u8, imm: u32, rd: u8) {
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

    pub(crate) fn sb(&mut self, rs1: u8, rs2: u8, imm: u32) {
        let rs1 = self.regs.get(rs1);
        let rs2 = self.regs.get(rs2);
        let addr = rs1.wrapping_add(imm);

        if self.store_byte(addr, rs2 as u8).is_err() {
            #[cfg(feature = "debug-console")]
            self.debug_console.store_out_of_bounds(addr, self.pc);
        }

        self.pc += 4;
    }

    pub(crate) fn sh(&mut self, rs1: u8, rs2: u8, imm: u32) {
        let rs1 = self.regs.get(rs1);
        let rs2 = self.regs.get(rs2);
        let addr = rs1.wrapping_add(imm);

        if self.store_halfword(addr, rs2 as u16).is_err() {
            #[cfg(feature = "debug-console")]
            self.debug_console.store_out_of_bounds(addr, self.pc);
        }

        self.pc += 4;
    }

    pub(crate) fn sw(&mut self, rs1: u8, rs2: u8, imm: u32) {
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
    use crate::instruction::Instruction;
    use crate::io;
    use crate::test_utils::*;

    #[test]
    fn test_lb() {
        let mut cpu = new_cpu();

        let data: Vec<(u8, u32)> = vec![(0x83, (-125i32) as u32), (0x12, 18)];

        for (inp, out) in data {
            for i in 0..4 {
                cpu.store_byte(i, inp).unwrap();
                cpu.exec_instruction(Instruction::LB {
                    rs1: 0,
                    imm: i,
                    rd: 1,
                });
                assert_eq!(cpu.regs.get(1), out);
            }
        }
    }

    #[test]
    fn test_lh() {
        let mut cpu = new_cpu();

        let data: Vec<(u16, u32)> = vec![(0x8313, (-31981i32) as u32), (0x1245, 4677)];

        for (inp, out) in data {
            for i in 0..4 {
                cpu.store_halfword(i, inp).unwrap();
                cpu.exec_instruction(Instruction::LH {
                    rs1: 0,
                    imm: i,
                    rd: 1,
                });
                assert_eq!(
                    cpu.regs.get(1),
                    out,
                    "{}",
                    format!("i: {}, inp: {}, out: {}", i, inp, out)
                );
            }
        }
    }

    #[test]
    fn test_lw() {
        let mut cpu = new_cpu();

        let data: Vec<u32> = vec![
            0x12345678, 0x87654321, 0x00000000, 0xFFFFFFFF, 0x0000FFFF, 0xFFFF0000,
        ];

        for v in data {
            for i in 0..4 {
                cpu.store_word(i, v).unwrap();
                cpu.exec_instruction(Instruction::LW {
                    rs1: 0,
                    imm: i,
                    rd: 1,
                });
                assert_eq!(cpu.regs.get(1), v, "{}", format!("i: {}, value: {}", i, v));
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
                cpu.lbu(0, i, 1);
                assert_eq!(cpu.regs.get(1), out);
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

    #[test]
    fn test_load_and_save() {
        let sdram = io::SDRam::new();
        let mut cpu = Cpu::new_with_bus(sdram);

        cpu.exec_instruction(0x361880b7.try_into().unwrap()); // lui x1, 0x36188
        cpu.exec_instruction(0x71908093.try_into().unwrap()); // addi x1, x1, 1817 # 0x36188719
        assert_eq!(cpu.regs.get(1), 0x36188719);
        cpu.exec_instruction(0x00102023.try_into().unwrap()); // sw x1, 0(x0)
        assert_eq!(cpu.bus.load_word(0).unwrap(), 0x36188719);
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

use crate::{cpu::Cpu, io, register::Register};

impl<T: io::Data<()>> Cpu<T> {
    pub(crate) fn addi(&mut self, rs1: Register, imm: u32, rd: Register) {
        let rs1 = self.regs.get(rs1);
        self.regs.set(rd, rs1.wrapping_add(imm));
        self.pc += 4;
    }

    pub(crate) fn andi(&mut self, rs1: Register, imm: u32, rd: Register) {
        let rs1 = self.regs.get(rs1);
        self.regs.set(rd, rs1 & imm);
        self.pc += 4;
    }

    pub(crate) fn ori(&mut self, rs1: Register, imm: u32, rd: Register) {
        let rs1 = self.regs.get(rs1);
        self.regs.set(rd, rs1 | imm);
        self.pc += 4;
    }

    pub(crate) fn xori(&mut self, rs1: Register, imm: u32, rd: Register) {
        let rs1 = self.regs.get(rs1);
        self.regs.set(rd, rs1 ^ imm);
        self.pc += 4;
    }

    pub(crate) fn slli(&mut self, rs1: Register, imm: u32, rd: Register) {
        let rs1 = self.regs.get(rs1);
        self.regs.set(rd, rs1.wrapping_shl(imm));
        self.pc += 4;
    }

    pub(crate) fn srli(&mut self, rs1: Register, imm: u32, rd: Register) {
        let rs1 = self.regs.get(rs1);
        self.regs.set(rd, rs1.wrapping_shr(imm));
        self.pc += 4;
    }

    pub(crate) fn srai(&mut self, rs1: Register, imm: u32, rd: Register) {
        let rs1 = self.regs.get(rs1);
        self.regs.set(rd, (rs1 as i32).wrapping_shr(imm) as u32);
        self.pc += 4;
    }

    pub(crate) fn slti(&mut self, rs1: Register, imm: u32, rd: Register) {
        let rs1 = self.regs.get(rs1);
        self.regs
            .set(rd, if (rs1 as i32) < (imm as i32) { 1 } else { 0 });
        self.pc += 4;
    }

    pub(crate) fn sltiu(&mut self, rs1: Register, imm: u32, rd: Register) {
        let rs1 = self.regs.get(rs1);
        self.regs.set(rd, if rs1 < imm { 1 } else { 0 });
        self.pc += 4;
    }
}

#[cfg(test)]
mod tests {
    use crate::{register::Register, test_utils::*};

    #[test]
    fn test_addi() {
        struct AddiTestCase {
            rs1: Register,
            rs1_value: u32,
            imm: u32,
            rd: Register,
            expected: u32,
        }

        let cases = vec![
            AddiTestCase {
                rs1: Register::T1,
                rs1_value: 5,
                imm: 3,
                rd: Register::T2,
                expected: 8,
            },
            AddiTestCase {
                rs1: Register::T1,
                rs1_value: 5,
                imm: 0xffff_fffc,
                rd: Register::T1,
                expected: 1,
            },
            AddiTestCase {
                rs1: Register::S1,
                rs1_value: 5,
                imm: 0xffff_ffff,
                rd: Register::T6,
                expected: 4,
            },
        ];

        for case in cases {
            let mut cpu = new_panic_io_cpu();
            cpu.pc = 0;
            cpu.regs.set(case.rs1, case.rs1_value);
            cpu.addi(case.rs1, case.imm, case.rd);
            assert_eq!(cpu.regs.get(case.rd), case.expected);
            assert_eq!(cpu.pc, 4);
        }
    }

    #[test]
    fn test_andi() {
        todo!();
    }

    #[test]
    fn test_ori() {
        todo!();
    }

    #[test]
    fn test_xori() {
        todo!();
    }

    #[test]
    fn test_slli() {
        todo!();
    }

    #[test]
    fn test_srli() {
        todo!();
    }

    #[test]
    fn test_srai() {
        todo!();
    }

    #[test]
    fn test_slti() {
        struct SltiTestCase {
            rs1: Register,
            rs1_value: u32,
            imm: u32,
            rd: Register,
            expected: u32,
        }

        let cases = vec![
            SltiTestCase {
                rs1: Register::T0,
                rs1_value: 0,
                imm: 0,
                rd: Register::T1,
                expected: 0,
            },
            SltiTestCase {
                rs1: Register::A0,
                rs1_value: 0,
                imm: 1,
                rd: Register::T4,
                expected: 1,
            },
            SltiTestCase {
                rs1: Register::T1,
                rs1_value: 1,
                imm: 1,
                rd: Register::S3,
                expected: 0,
            },
            SltiTestCase {
                rs1: Register::S3,
                rs1_value: 1,
                imm: u32::MAX,
                rd: Register::RA,
                expected: 0,
            },
            SltiTestCase {
                rs1: Register::A0,
                rs1_value: u32::MAX,
                imm: 1,
                rd: Register::S1,
                expected: 1,
            },
        ];

        for case in cases {
            let mut cpu = new_panic_io_cpu();
            cpu.pc = 0;
            cpu.regs.set(case.rs1, case.rs1_value);
            cpu.slti(case.rs1, case.imm, case.rd);
            assert_eq!(cpu.regs.get(case.rd), case.expected);
            assert_eq!(cpu.pc, 4);
        }
    }

    #[test]
    fn test_sltiu() {
        #[derive(Debug)]
        struct SltiuTestCase {
            rs1: Register,
            rs1_value: u32,
            imm: u32,
            rd: Register,
            expected: u32,
        }

        let cases = vec![
            SltiuTestCase {
                rs1: Register::T0,
                rs1_value: 0,
                imm: 0,
                rd: Register::T1,
                expected: 0,
            },
            SltiuTestCase  {
                rs1: Register::A0,
                rs1_value: 0,
                imm: 1,
                rd: Register::T4,
                expected: 1,
            },
            SltiuTestCase {
                rs1: Register::T1,
                rs1_value: 1,
                imm: 1,
                rd: Register::S3,
                expected: 0,
            },
            SltiuTestCase {
                rs1: Register::S3,
                rs1_value: 1,
                imm: u32::MAX,
                rd: Register::RA,
                expected: 1,
            },
            SltiuTestCase {
                rs1: Register::A0,
                rs1_value: u32::MAX,
                imm: 1,
                rd: Register::S1,
                expected: 0,
            },
        ];

        for case in cases {
            let mut cpu = new_panic_io_cpu();
            cpu.pc = 0;
            cpu.regs.set(case.rs1, case.rs1_value);
            cpu.sltiu(case.rs1, case.imm, case.rd);
            assert_eq!(cpu.regs.get(case.rd), case.expected, "{:?}", case);
            assert_eq!(cpu.pc, 4);
        }
    }
}

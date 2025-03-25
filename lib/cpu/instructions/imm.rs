use crate::{
    cpu::Cpu,
    instruction::{ITypeImm, ShamtImm},
    io,
    register::Register,
};

impl<T: io::Data<()>> Cpu<T> {
    pub(crate) fn addi(&mut self, rs1: Register, imm: ITypeImm, rd: Register) {
        let imm = imm.as_u32();
        let rs1 = self.regs.get(rs1);
        self.regs.set(rd, rs1.wrapping_add(imm));
        self.pc += 4;
    }

    pub(crate) fn andi(&mut self, rs1: Register, imm: ITypeImm, rd: Register) {
        let imm = imm.as_u32();
        let rs1 = self.regs.get(rs1);
        self.regs.set(rd, rs1 & imm);
        self.pc += 4;
    }

    pub(crate) fn ori(&mut self, rs1: Register, imm: ITypeImm, rd: Register) {
        let imm = imm.as_u32();
        let rs1 = self.regs.get(rs1);
        self.regs.set(rd, rs1 | imm);
        self.pc += 4;
    }

    pub(crate) fn xori(&mut self, rs1: Register, imm: ITypeImm, rd: Register) {
        let imm = imm.as_u32();
        let rs1 = self.regs.get(rs1);
        self.regs.set(rd, rs1 ^ imm);
        self.pc += 4;
    }

    pub(crate) fn slli(&mut self, rs1: Register, imm: ShamtImm, rd: Register) {
        let imm = imm.as_u32();
        let rs1 = self.regs.get(rs1);
        debug_assert!(
            imm < 32,
            "When creating an SLLI instruction, the imm value should be less than 32"
        );
        self.regs.set(rd, rs1 << imm);
        self.pc += 4;
    }

    pub(crate) fn srli(&mut self, rs1: Register, imm: ShamtImm, rd: Register) {
        let imm = imm.as_u32();
        let rs1 = self.regs.get(rs1);
        debug_assert!(
            imm < 32,
            "When creating an SRLI instruction, the imm value should be less than 32"
        );
        self.regs.set(rd, rs1 >> imm);
        self.pc += 4;
    }

    pub(crate) fn srai(&mut self, rs1: Register, imm: ShamtImm, rd: Register) {
        let imm = imm.as_u32();
        let rs1 = self.regs.get(rs1);
        debug_assert!(
            imm < 32,
            "When creating an SRAI instruction, the imm value should be less than 32"
        );
        self.regs.set(rd, ((rs1 as i32) >> imm) as u32);
        self.pc += 4;
    }

    pub(crate) fn slti(&mut self, rs1: Register, imm: ITypeImm, rd: Register) {
        let imm = imm.as_u32();
        let rs1 = self.regs.get(rs1);
        self.regs
            .set(rd, if (rs1 as i32) < (imm as i32) { 1 } else { 0 });
        self.pc += 4;
    }

    pub(crate) fn sltiu(&mut self, rs1: Register, imm: ITypeImm, rd: Register) {
        let imm = imm.as_u32();
        let rs1 = self.regs.get(rs1);
        self.regs.set(rd, if rs1 < imm { 1 } else { 0 });
        self.pc += 4;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;

    /// Helper to create imm value without the boilerplate
    macro_rules! imm {
        ($v:expr) => {
            $v.try_into().unwrap()
        };
    }

    #[test]
    fn test_addi() {
        struct AddiTestCase {
            rs1: Register,
            rs1_value: u32,
            imm: ITypeImm,
            rd: Register,
            expected: u32,
        }

        let cases = vec![
            AddiTestCase {
                rs1: Register::T1,
                rs1_value: 5,
                imm: imm!(3),
                rd: Register::T2,
                expected: 8,
            },
            AddiTestCase {
                rs1: Register::T1,
                rs1_value: 5,
                imm: imm!(0xffff_fffc),
                rd: Register::T1,
                expected: 1,
            },
            AddiTestCase {
                rs1: Register::S1,
                rs1_value: 5,
                imm: imm!(0xffff_ffff),
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
        #[derive(Debug)]
        struct AndiTestCase {
            rs1: Register,
            rs1_value: u32,
            imm: ITypeImm,
            rd: Register,
            expected: u32,
        }

        let cases = vec![
            AndiTestCase {
                rs1: Register::T0,
                rs1_value: 0b1010,
                imm: imm!(0b1100),
                rd: Register::T1,
                expected: 0b1000,
            },
            AndiTestCase {
                rs1: Register::GP,
                rs1_value: 0b1111,
                imm: imm!(0b1100),
                rd: Register::T3,
                expected: 0b1100,
            },
            AndiTestCase {
                rs1: Register::RA,
                rs1_value: 0b0,
                imm: imm!(0b1100),
                rd: Register::S5,
                expected: 0b0,
            },
        ];

        for case in cases {
            let mut cpu = new_panic_io_cpu();
            cpu.pc = 0;
            cpu.regs.set(case.rs1, case.rs1_value);
            cpu.andi(case.rs1, case.imm, case.rd);
            assert_eq!(cpu.regs.get(case.rd), case.expected, "{:?}", case);
            assert_eq!(cpu.pc, 4);
        }
    }

    #[test]
    fn test_ori() {
        struct OriTestCase {
            rs1: Register,
            rs1_value: u32,
            imm: ITypeImm,
            rd: Register,
            expected: u32,
        }

        let cases = vec![
            OriTestCase {
                rs1: Register::T0,
                rs1_value: 0b1010,
                imm: imm!(0b1100),
                rd: Register::T1,
                expected: 0b1110,
            },
            OriTestCase {
                rs1: Register::GP,
                rs1_value: 0b1111,
                imm: imm!(0b1100),
                rd: Register::T3,
                expected: 0b1111,
            },
            OriTestCase {
                rs1: Register::RA,
                rs1_value: 0b0,
                imm: imm!(0b1100),
                rd: Register::S5,
                expected: 0b1100,
            },
        ];

        for case in cases {
            let mut cpu = new_panic_io_cpu();
            cpu.pc = 0;
            cpu.regs.set(case.rs1, case.rs1_value);
            cpu.ori(case.rs1, case.imm, case.rd);
            assert_eq!(cpu.regs.get(case.rd), case.expected);
            assert_eq!(cpu.pc, 4);
        }
    }

    #[test]
    fn test_xori() {
        struct XoriTestCase {
            rs1: Register,
            rs1_value: u32,
            imm: ITypeImm,
            rd: Register,
            expected: u32,
        }

        let cases = vec![
            XoriTestCase {
                rs1: Register::T0,
                rs1_value: 0b1010,
                imm: imm!(0b1100),
                rd: Register::T1,
                expected: 0b0110,
            },
            XoriTestCase {
                rs1: Register::GP,
                rs1_value: 0b1111,
                imm: imm!(0b1100),
                rd: Register::T3,
                expected: 0b0011,
            },
            XoriTestCase {
                rs1: Register::RA,
                rs1_value: 0b0,
                imm: imm!(0b1100),
                rd: Register::S5,
                expected: 0b1100,
            },
        ];

        for case in cases {
            let mut cpu = new_panic_io_cpu();
            cpu.pc = 0;
            cpu.regs.set(case.rs1, case.rs1_value);
            cpu.xori(case.rs1, case.imm, case.rd);
            assert_eq!(cpu.regs.get(case.rd), case.expected);
            assert_eq!(cpu.pc, 4);
        }
    }

    #[test]
    fn test_slli() {
        #[derive(Debug)]
        struct SlliTestCase {
            rs1: Register,
            rs1_value: u32,
            imm: ShamtImm,
            rd: Register,
            expected: u32,
        }

        let cases = vec![
            SlliTestCase {
                rs1: Register::T0,
                rs1_value: 0b1010,
                imm: imm!(2),
                rd: Register::T1,
                expected: 0b101000,
            },
            SlliTestCase {
                rs1: Register::GP,
                rs1_value: 0b1111,
                imm: imm!(3),
                rd: Register::T3,
                expected: 0b1111000,
            },
            SlliTestCase {
                rs1: Register::RA,
                rs1_value: 0b1,
                imm: imm!(0),
                rd: Register::S5,
                expected: 0b1,
            },
            SlliTestCase {
                rs1: Register::A1,
                rs1_value: 0xFFFF_FFFE,
                imm: imm!(31),
                rd: Register::S5,
                expected: 0b0,
            },
        ];

        for (i, case) in cases.into_iter().enumerate() {
            let mut cpu = new_panic_io_cpu();
            cpu.pc = 0;
            cpu.regs.set(case.rs1, case.rs1_value);
            cpu.slli(case.rs1, case.imm, case.rd);
            assert_eq!(cpu.regs.get(case.rd), case.expected, "{}: {:?}", i, case);
            assert_eq!(cpu.pc, 4);
        }
    }

    #[test]
    fn test_srli() {
        struct SrliTestCase {
            rs1: Register,
            rs1_value: u32,
            imm: ShamtImm,
            rd: Register,
            expected: u32,
        }

        let cases = vec![
            SrliTestCase {
                rs1: Register::T0,
                rs1_value: u32::MAX,
                imm: imm!(2),
                rd: Register::T1,
                expected: 0x3FFF_FFFF,
            },
            SrliTestCase {
                rs1: Register::GP,
                rs1_value: 0b1111,
                imm: imm!(2),
                rd: Register::T3,
                expected: 0b11,
            },
            SrliTestCase {
                rs1: Register::RA,
                rs1_value: 0b11,
                imm: imm!(0),
                rd: Register::S5,
                expected: 0b11,
            },
            SrliTestCase {
                rs1: Register::RA,
                rs1_value: 0b11,
                imm: imm!(4),
                rd: Register::S5,
                expected: 0b0,
            },
        ];

        for case in cases {
            let mut cpu = new_panic_io_cpu();
            cpu.pc = 0;
            cpu.regs.set(case.rs1, case.rs1_value);
            cpu.srli(case.rs1, case.imm, case.rd);
            assert_eq!(cpu.regs.get(case.rd), case.expected);
            assert_eq!(cpu.pc, 4);
        }
    }

    #[test]
    fn test_srai() {
        #[derive(Debug)]
        struct SraiTestCase {
            rs1: Register,
            rs1_value: u32,
            imm: ShamtImm,
            rd: Register,
            expected: u32,
        }

        let cases = vec![
            SraiTestCase {
                rs1: Register::T0,
                rs1_value: u32::MAX,
                imm: imm!(2),
                rd: Register::T1,
                expected: 0xFFFF_FFFF,
            },
            SraiTestCase {
                rs1: Register::T0,
                rs1_value: (-7i32) as u32,
                imm: imm!(1),
                rd: Register::T1,
                expected: (-4i32) as u32,
            },
            SraiTestCase {
                rs1: Register::GP,
                rs1_value: 0b1111,
                imm: imm!(2),
                rd: Register::T3,
                expected: 0b11,
            },
            SraiTestCase {
                rs1: Register::RA,
                rs1_value: 0b11,
                imm: imm!(0),
                rd: Register::S5,
                expected: 0b11,
            },
            SraiTestCase {
                rs1: Register::RA,
                rs1_value: 0b11,
                imm: imm!(4),
                rd: Register::S5,
                expected: 0b0,
            },
        ];

        for (i, case) in cases.into_iter().enumerate() {
            let mut cpu = new_panic_io_cpu();
            cpu.pc = 0;
            cpu.regs.set(case.rs1, case.rs1_value);
            cpu.srai(case.rs1, case.imm, case.rd);
            assert_eq!(cpu.regs.get(case.rd), case.expected, "{}: {:?}", i, case);
            assert_eq!(cpu.pc, 4);
        }
    }

    #[test]
    fn test_slti() {
        #[derive(Debug)]
        struct SltiTestCase {
            rs1: Register,
            rs1_value: u32,
            imm: ITypeImm,
            rd: Register,
            expected: u32,
        }

        let cases = vec![
            SltiTestCase {
                rs1: Register::T0,
                rs1_value: 0,
                imm: imm!(0),
                rd: Register::T1,
                expected: 0,
            },
            SltiTestCase {
                rs1: Register::A0,
                rs1_value: 0,
                imm: imm!(1),
                rd: Register::T4,
                expected: 1,
            },
            SltiTestCase {
                rs1: Register::T1,
                rs1_value: 1,
                imm: imm!(1),
                rd: Register::S3,
                expected: 0,
            },
            SltiTestCase {
                rs1: Register::S3,
                rs1_value: 1,
                imm: imm!(u32::MAX),
                rd: Register::RA,
                expected: 0,
            },
            SltiTestCase {
                rs1: Register::A0,
                rs1_value: u32::MAX,
                imm: imm!(1),
                rd: Register::S1,
                expected: 1,
            },
        ];

        for case in cases {
            let mut cpu = new_panic_io_cpu();
            cpu.pc = 0;
            cpu.regs.set(case.rs1, case.rs1_value);
            cpu.slti(case.rs1, case.imm, case.rd);
            assert_eq!(cpu.regs.get(case.rd), case.expected, "{:?}", case);
            assert_eq!(cpu.pc, 4);
        }
    }

    #[test]
    fn test_sltiu() {
        #[derive(Debug)]
        struct SltiuTestCase {
            rs1: Register,
            rs1_value: u32,
            imm: ITypeImm,
            rd: Register,
            expected: u32,
        }

        let cases = vec![
            SltiuTestCase {
                rs1: Register::T0,
                rs1_value: 0,
                imm: imm!(0),
                rd: Register::T1,
                expected: 0,
            },
            SltiuTestCase {
                rs1: Register::A0,
                rs1_value: 0,
                imm: imm!(1),
                rd: Register::T4,
                expected: 1,
            },
            SltiuTestCase {
                rs1: Register::T1,
                rs1_value: 1,
                imm: imm!(1),
                rd: Register::S3,
                expected: 0,
            },
            SltiuTestCase {
                rs1: Register::S3,
                rs1_value: 1,
                imm: imm!(u32::MAX),
                rd: Register::RA,
                expected: 1,
            },
            SltiuTestCase {
                rs1: Register::A0,
                rs1_value: u32::MAX,
                imm: imm!(1),
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

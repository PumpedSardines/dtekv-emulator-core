use crate::{cpu::Cpu, peripheral::Peripheral, register::Register};

const XLEN_MASK: u32 = 0x1f;

impl<T: Peripheral<()>> Cpu<T> {
    pub(crate) fn add(&mut self, rs1: Register, rs2: Register, rd: Register) {
        let rs1 = self.regs.get(rs1);
        let rs2 = self.regs.get(rs2);
        self.regs.set(rd, rs1.wrapping_add(rs2));
        self.pc += 4;
    }

    pub(crate) fn sub(&mut self, rs1: Register, rs2: Register, rd: Register) {
        let rs1 = self.regs.get(rs1);
        let rs2 = self.regs.get(rs2);
        self.regs.set(rd, rs1.wrapping_sub(rs2));
        self.pc += 4;
    }

    pub(crate) fn slt(&mut self, rs1: Register, rs2: Register, rd: Register) {
        let rs1 = self.regs.get(rs1);
        let rs2 = self.regs.get(rs2);
        self.regs
            .set(rd, if (rs1 as i32) < (rs2 as i32) { 1 } else { 0 });
        self.pc += 4;
    }

    pub(crate) fn sltu(&mut self, rs1: Register, rs2: Register, rd: Register) {
        let rs1 = self.regs.get(rs1);
        let rs2 = self.regs.get(rs2);
        self.regs.set(rd, if rs1 < rs2 { 1 } else { 0 });
        self.pc += 4;
    }

    pub(crate) fn sll(&mut self, rs1: Register, rs2: Register, rd: Register) {
        let rs1 = self.regs.get(rs1);
        let rs2 = self.regs.get(rs2) & XLEN_MASK;
        self.regs.set(rd, rs1 << rs2);
        self.pc += 4;
    }

    pub(crate) fn srl(&mut self, rs1: Register, rs2: Register, rd: Register) {
        let rs1 = self.regs.get(rs1);
        let rs2 = self.regs.get(rs2) & XLEN_MASK;
        self.regs.set(rd, rs1.wrapping_shr(rs2));
        self.pc += 4;
    }

    pub(crate) fn sra(&mut self, rs1: Register, rs2: Register, rd: Register) {
        let rs1 = self.regs.get(rs1);
        let rs2 = self.regs.get(rs2) & XLEN_MASK;
        self.regs.set(rd, (rs1 as i32).wrapping_shr(rs2) as u32);
        self.pc += 4;
    }

    pub(crate) fn and(&mut self, rs1: Register, rs2: Register, rd: Register) {
        let rs1 = self.regs.get(rs1);
        let rs2 = self.regs.get(rs2);
        self.regs.set(rd, rs1 & rs2);
        self.pc += 4;
    }

    pub(crate) fn or(&mut self, rs1: Register, rs2: Register, rd: Register) {
        let rs1 = self.regs.get(rs1);
        let rs2 = self.regs.get(rs2);
        self.regs.set(rd, rs1 | rs2);
        self.pc += 4;
    }

    pub(crate) fn xor(&mut self, rs1: Register, rs2: Register, rd: Register) {
        let rs1 = self.regs.get(rs1);
        let rs2 = self.regs.get(rs2);
        self.regs.set(rd, rs1 ^ rs2);
        self.pc += 4;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;
    use test_case::test_case;

    #[test]
    fn test_add() {
        struct AddTestCase {
            rs1: Register,
            rs1_value: u32,
            rs2: Register,
            rs2_value: u32,
            rd: Register,
            expected: u32,
        }

        let cases = vec![
            AddTestCase {
                rs1: Register::T0,
                rs1_value: 1,
                rs2: Register::T1,
                rs2_value: 2,
                rd: Register::T2,
                expected: 3,
            },
            AddTestCase {
                rs1: Register::RA,
                rs1_value: (-5i32) as u32,
                rs2: Register::A0,
                rs2_value: 2,
                rd: Register::S7,
                expected: (-3i32) as u32,
            },
        ];

        for case in cases {
            let mut cpu = new_panic_io_cpu();
            cpu.pc = 0;
            cpu.regs.set(case.rs1, case.rs1_value);
            cpu.regs.set(case.rs2, case.rs2_value);
            cpu.add(case.rs1, case.rs2, case.rd);
            assert_eq!(cpu.regs.get(case.rd), case.expected);
            assert_eq!(cpu.pc, 4);
        }
    }

    #[test_case(0x22, 0x11 => 0x11; "positive result")]
    #[test_case(0x11, 0x22 => (-17i32) as u32; "negative result")]
    #[test_case(10, (-10i32) as u32 => 20; "negative second operand")]
    #[test_case(10, 0xFFFF_FFFF => 11; "overflow")]
    fn test_sub(a: u32, b: u32) -> u32 {
        let mut cpu = new_panic_io_cpu();
        cpu.pc = 0;
        cpu.regs.set(Register::T0, a);
        cpu.regs.set(Register::T1, b);
        cpu.sub(Register::T0, Register::T1, Register::T2);
        assert_eq!(cpu.pc, 4);
        cpu.regs.get(Register::T2)
    }

    #[test_case(0, 100 => 1; "operand is less")]
    #[test_case(100, 0 => 0; "operand is greater")]
    #[test_case((-100i32) as u32, 0 => 1; "negative less than")]
    #[test_case(0, (-100i32) as u32 => 0; "negative greater than")]
    fn test_slt(a: u32, b: u32) -> u32 {
        let mut cpu = new_panic_io_cpu();
        cpu.pc = 0;
        cpu.regs.set(Register::T0, a);
        cpu.regs.set(Register::T1, b);
        cpu.slt(Register::T0, Register::T1, Register::T2);
        assert_eq!(cpu.pc, 4);
        cpu.regs.get(Register::T2)
    }

    #[test_case(0, 100 => 1; "operand is less")]
    #[test_case(100, 0 => 0; "operand is greater")]
    #[test_case((-100i32) as u32, 0 => 0; "negative less than")]
    #[test_case(0, (-100i32) as u32 => 1; "negative greater than")]
    fn test_sltu(a: u32, b: u32) -> u32 {
        let mut cpu = new_panic_io_cpu();
        cpu.pc = 0;
        cpu.regs.set(Register::T0, a);
        cpu.regs.set(Register::T1, b);
        cpu.sltu(Register::T0, Register::T1, Register::T2);
        assert_eq!(cpu.pc, 4);
        cpu.regs.get(Register::T2)
    }

    #[test_case(1, 3 => 0b1000; "regular")]
    #[test_case(1, 400 => 65536; "overflow")]
    fn test_sll(a: u32, b: u32) -> u32 {
        let mut cpu = new_panic_io_cpu();
        cpu.pc = 0;
        cpu.regs.set(Register::T0, a);
        cpu.regs.set(Register::T1, b);
        cpu.sll(Register::T0, Register::T1, Register::T2);
        assert_eq!(cpu.pc, 4);
        cpu.regs.get(Register::T2)
    }

    #[test_case(0b1000, 3 => 1; "regular")]
    #[test_case(0b1000, 0x103 => 1; "overflow 1")]
    #[test_case(0b100000, 837 => 1; "overflow 2")]
    #[test_case(0xFFFF_FFFF, 4 => 0x0FFF_FFFF; "negative")]
    fn test_srl(a: u32, b: u32) -> u32 {
        let mut cpu = new_panic_io_cpu();
        cpu.pc = 0;
        cpu.regs.set(Register::T0, a);
        cpu.regs.set(Register::T1, b);
        cpu.srl(Register::T0, Register::T1, Register::T2);
        assert_eq!(cpu.pc, 4);
        cpu.regs.get(Register::T2)
    }

    #[test_case(0b1000, 3 => 1; "regular")]
    #[test_case(0b1000, 0x103 => 1; "overflow 1")]
    #[test_case(0b100000, 837 => 1; "overflow 2")]
    #[test_case(0xFFF3_FAFF, 4 => 0xFFFF_3FAF; "negative")]
    fn test_sra(a: u32, b: u32) -> u32 {
        let mut cpu = new_panic_io_cpu();
        cpu.pc = 0;
        cpu.regs.set(Register::T0, a);
        cpu.regs.set(Register::T1, b);
        cpu.sra(Register::T0, Register::T1, Register::T2);
        assert_eq!(cpu.pc, 4);
        cpu.regs.get(Register::T2)
    }

    #[test_case(0xF67B615, 0x71625F => 0x612215; "regular")]
    fn test_and(a: u32, b: u32) -> u32 {
        let mut cpu = new_panic_io_cpu();
        cpu.pc = 0;
        cpu.regs.set(Register::T0, a);
        cpu.regs.set(Register::T1, b);
        cpu.and(Register::T0, Register::T1, Register::T2);
        assert_eq!(cpu.pc, 4);
        cpu.regs.get(Register::T2)
    }

    #[test_case(0xF67B615, 0x71625F => 0xF77F65F; "regular")]
    fn test_or(a: u32, b: u32) -> u32 {
        let mut cpu = new_panic_io_cpu();
        cpu.pc = 0;
        cpu.regs.set(Register::T0, a);
        cpu.regs.set(Register::T1, b);
        cpu.or(Register::T0, Register::T1, Register::T2);
        assert_eq!(cpu.pc, 4);
        cpu.regs.get(Register::T2)
    }

    #[test_case(0xF67B615, 0x71625F => 0xF16D44A; "regular")]
    fn test_xor(a: u32, b: u32) -> u32 {
        let mut cpu = new_panic_io_cpu();
        cpu.pc = 0;
        cpu.regs.set(Register::T0, a);
        cpu.regs.set(Register::T1, b);
        cpu.xor(Register::T0, Register::T1, Register::T2);
        assert_eq!(cpu.pc, 4);
        cpu.regs.get(Register::T2)
    }
}

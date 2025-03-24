use crate::{cpu::Cpu, io};

impl<T: io::Data<()>> Cpu<T> {
    pub(crate) fn add(&mut self, rs1: u8, rs2: u8, rd: u8) {
        let rs1 = self.regs.get(rs1);
        let rs2 = self.regs.get(rs2);
        self.regs.set(rd, rs1.wrapping_add(rs2));
        self.pc += 4;
    }

    pub(crate) fn sub(&mut self, rs1: u8, rs2: u8, rd: u8) {
        let rs1 = self.regs.get(rs1);
        let rs2 = self.regs.get(rs2);
        self.regs.set(rd, rs1.wrapping_sub(rs2));
        self.pc += 4;
    }

    pub(crate) fn slt(&mut self, rs1: u8, rs2: u8, rd: u8) {
        let rs1 = self.regs.get(rs1);
        let rs2 = self.regs.get(rs2);
        self.regs
            .set(rd, if (rs1 as i32) < (rs2 as i32) { 1 } else { 0 });
        self.pc += 4;
    }

    pub(crate) fn sltu(&mut self, rs1: u8, rs2: u8, rd: u8) {
        let rs1 = self.regs.get(rs1);
        let rs2 = self.regs.get(rs2);
        self.regs.set(rd, if rs1 < rs2 { 1 } else { 0 });
        self.pc += 4;
    }

    pub(crate) fn sll(&mut self, rs1: u8, rs2: u8, rd: u8) {
        let rs1 = self.regs.get(rs1);
        let rs2 = self.regs.get(rs2) & 0x1f;
        self.regs.set(rd, rs1.wrapping_shl(rs2));
        self.pc += 4;
    }

    pub(crate) fn srl(&mut self, rs1: u8, rs2: u8, rd: u8) {
        let rs1 = self.regs.get(rs1);
        let rs2 = self.regs.get(rs2) & 0x1f;
        self.regs.set(rd, rs1.wrapping_shr(rs2));
        self.pc += 4;
    }

    pub(crate) fn sra(&mut self, rs1: u8, rs2: u8, rd: u8) {
        let rs1 = self.regs.get(rs1);
        let rs2 = self.regs.get(rs2) & 0x1f;
        self.regs.set(rd, (rs1 as i32).wrapping_shr(rs2) as u32);
        self.pc += 4;
    }

    pub(crate) fn and(&mut self, rs1: u8, rs2: u8, rd: u8) {
        let rs1 = self.regs.get(rs1);
        let rs2 = self.regs.get(rs2);
        self.regs.set(rd, rs1 & rs2);
        self.pc += 4;
    }

    pub(crate) fn or(&mut self, rs1: u8, rs2: u8, rd: u8) {
        let rs1 = self.regs.get(rs1);
        let rs2 = self.regs.get(rs2);
        self.regs.set(rd, rs1 | rs2);
        self.pc += 4;
    }

    pub(crate) fn xor(&mut self, rs1: u8, rs2: u8, rd: u8) {
        let rs1 = self.regs.get(rs1);
        let rs2 = self.regs.get(rs2);
        self.regs.set(rd, rs1 ^ rs2);
        self.pc += 4;
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::*;
    
    #[test]
    fn test_add() {
        struct AddTestCase {
            rs1: u8,
            rs1_value: u32,
            rs2: u8,
            rs2_value: u32,
            rd: u8,
            expected: u32,
        }

        let cases = vec![
            AddTestCase {
                rs1: 1,
                rs1_value: 1,
                rs2: 2,
                rs2_value: 2,
                rd: 3,
                expected: 3,
            },
            AddTestCase {
                rs1: 7,
                rs1_value: (-5i32) as u32,
                rs2: 4,
                rs2_value: 2,
                rd: 3,
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

    #[test]
    fn test_sub() {
        todo!()
    }

    #[test]
    fn test_slt() {
        todo!()
    }

    #[test]
    fn test_sltu() {
        todo!()
    }

    #[test]
    fn test_sll() {
        todo!()
    }

    #[test]
    fn test_srl() {
        todo!()
    }

    #[test]
    fn test_sra() {
        todo!()
    }

    #[test]
    fn test_and() {
        todo!()
    }

    #[test]
    fn test_or() {
        todo!()
    }

    #[test]
    fn test_xor() {
        todo!()
    }
}

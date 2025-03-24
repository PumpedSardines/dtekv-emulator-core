use crate::{cpu::Cpu, io};

impl<T: io::Data<()>> Cpu<T> {
    pub(crate) fn beq(&mut self, rs1: u8, rs2: u8, imm: u32) {
        let rs1 = self.regs.get(rs1);
        let rs2 = self.regs.get(rs2);
        if rs1 == rs2 {
            self.pc = self.pc.wrapping_add(imm);
        } else {
            self.pc += 4;
        }
    }

    pub(crate) fn bne(&mut self, rs1: u8, rs2: u8, imm: u32) {
        let rs1 = self.regs.get(rs1);
        let rs2 = self.regs.get(rs2);
        if rs1 != rs2 {
            self.pc = self.pc.wrapping_add(imm);
        } else {
            self.pc += 4;
        }
    }

    pub(crate) fn blt(&mut self, rs1: u8, rs2: u8, imm: u32) {
        let rs1 = self.regs.get(rs1);
        let rs2 = self.regs.get(rs2);
        if (rs1 as i32) < (rs2 as i32) {
            self.pc = self.pc.wrapping_add(imm);
        } else {
            self.pc += 4;
        }
    }

    pub(crate) fn bge(&mut self, rs1: u8, rs2: u8, imm: u32) {
        let rs1 = self.regs.get(rs1);
        let rs2 = self.regs.get(rs2);
        if (rs1 as i32) >= (rs2 as i32) {
            self.pc = self.pc.wrapping_add(imm);
        } else {
            self.pc += 4;
        }
    }

    pub(crate) fn bltu(&mut self, rs1: u8, rs2: u8, imm: u32) {
        let rs1 = self.regs.get(rs1);
        let rs2 = self.regs.get(rs2);
        if rs1 < rs2 {
            self.pc = self.pc.wrapping_add(imm);
        } else {
            self.pc += 4;
        }
    }

    pub(crate) fn bgeu(&mut self, rs1: u8, rs2: u8, imm: u32) {
        let rs1 = self.regs.get(rs1);
        let rs2 = self.regs.get(rs2);
        if rs1 >= rs2 {
            self.pc = self.pc.wrapping_add(imm);
        } else {
            self.pc += 4;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::*;

    #[test]
    fn test_beq() {
        let mut cpu = new_panic_io_cpu();

        cpu.pc = 0;
        cpu.regs.set(1, 0x1234);
        cpu.regs.set(2, 0x1234);
        cpu.beq(1, 2, 0x1000);
        assert_eq!(cpu.pc, 0x1000);
        cpu.regs.set(2, 0x1235);
        cpu.beq(1, 2, 0x1000);
        assert_eq!(cpu.pc, 0x1004);
    }

    #[test]
    fn test_bne() {
        let mut cpu = new_panic_io_cpu();

        cpu.pc = 0;
        cpu.regs.set(1, 0x1234);
        cpu.regs.set(2, 0x1234);
        cpu.bne(1, 2, 0x1000);
        assert_eq!(cpu.pc, 4);
        cpu.regs.set(2, 0x1235);
        cpu.bne(1, 2, 0x1000);
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
            let mut cpu = new_panic_io_cpu();

            cpu.pc = 8;
            cpu.regs.set(1, rs1);
            cpu.regs.set(2, rs2);
            cpu.blt(1, 2, 0x1000);
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
            let mut cpu = new_panic_io_cpu();

            cpu.pc = 8;
            cpu.regs.set(1, rs1);
            cpu.regs.set(2, rs2);
            cpu.bge(1, 2, 0x1000);
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
            let mut cpu = new_panic_io_cpu();

            cpu.pc = 8;
            cpu.regs.set(1, rs1);
            cpu.regs.set(2, rs2);
            cpu.bltu(1, 2, 0x1000);
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
            let mut cpu = new_panic_io_cpu();

            cpu.pc = 8;
            cpu.regs.set(1, rs1);
            cpu.regs.set(2, rs2);
            cpu.bgeu(1, 2, 0x1000);
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
}

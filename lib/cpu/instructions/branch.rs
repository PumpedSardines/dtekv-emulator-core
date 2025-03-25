use crate::{cpu::Cpu, io, register::Register};

impl<T: io::Data<()>> Cpu<T> {
    pub(crate) fn beq(&mut self, rs1: Register, rs2: Register, imm: u32) {
        let rs1 = self.regs.get(rs1);
        let rs2 = self.regs.get(rs2);
        if rs1 == rs2 {
            self.pc = self.pc.wrapping_add(imm);
        } else {
            self.pc += 4;
        }
    }

    pub(crate) fn bne(&mut self, rs1: Register, rs2: Register, imm: u32) {
        let rs1 = self.regs.get(rs1);
        let rs2 = self.regs.get(rs2);
        if rs1 != rs2 {
            self.pc = self.pc.wrapping_add(imm);
        } else {
            self.pc += 4;
        }
    }

    pub(crate) fn blt(&mut self, rs1: Register, rs2: Register, imm: u32) {
        let rs1 = self.regs.get(rs1);
        let rs2 = self.regs.get(rs2);
        if (rs1 as i32) < (rs2 as i32) {
            self.pc = self.pc.wrapping_add(imm);
        } else {
            self.pc += 4;
        }
    }

    pub(crate) fn bge(&mut self, rs1: Register, rs2: Register, imm: u32) {
        let rs1 = self.regs.get(rs1);
        let rs2 = self.regs.get(rs2);
        if (rs1 as i32) >= (rs2 as i32) {
            self.pc = self.pc.wrapping_add(imm);
        } else {
            self.pc += 4;
        }
    }

    pub(crate) fn bltu(&mut self, rs1: Register, rs2: Register, imm: u32) {
        let rs1 = self.regs.get(rs1);
        let rs2 = self.regs.get(rs2);
        if rs1 < rs2 {
            self.pc = self.pc.wrapping_add(imm);
        } else {
            self.pc += 4;
        }
    }

    pub(crate) fn bgeu(&mut self, rs1: Register, rs2: Register, imm: u32) {
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
    use crate::{register::Register, test_utils::*};

    #[test]
    fn test_beq() {
        let mut cpu = new_panic_io_cpu();

        cpu.pc = 0;
        cpu.regs.set(Register::T0, 0x1234);
        cpu.regs.set(Register::T1, 0x1234);
        cpu.beq(Register::T0, Register::T1, 0x1000);
        assert_eq!(cpu.pc, 0x1000);
        cpu.regs.set(Register::T1, 0x1235);
        cpu.beq(Register::T0, Register::T1, 0x1000);
        assert_eq!(cpu.pc, 0x1004);
    }

    #[test]
    fn test_bne() {
        let mut cpu = new_panic_io_cpu();

        cpu.pc = 0;
        cpu.regs.set(Register::T0, 0x1234);
        cpu.regs.set(Register::T1, 0x1234);
        cpu.bne(Register::T0, Register::T1, 0x1000);
        assert_eq!(cpu.pc, 4);
        cpu.regs.set(Register::T1, 0x1235);
        cpu.bne(Register::T0, Register::T1, 0x1000);
        assert_eq!(cpu.pc, 0x1004);
    }

    #[test]
    fn test_blt() {
        todo!();
    }

    #[test]
    fn test_bge() {
        todo!();
    }

    #[test]
    fn test_bltu() {
        todo!();
    }

    #[test]
    fn test_bgeu() {
        todo!();
    }
}

use crate::{cpu::Cpu, io};

impl<T: io::Data<()>> Cpu<T> {
    pub(crate) fn lui(&mut self, imm: u32, rd: u8) {
        self.regs.set(rd, imm);
        self.pc += 4;
    }

    pub(crate) fn auipc(&mut self, imm: u32, rd: u8) {
        self.regs.set(rd, self.pc.wrapping_add(imm));
        self.pc += 4;
    }

    pub(crate) fn jal(&mut self, imm: u32, rd: u8) {
        self.regs.set(rd, self.pc + 4);
        self.pc = self.pc.wrapping_add(imm);
    }

    pub(crate) fn jalr(&mut self, rs1: u8, imm: u32, rd: u8) {
        let rs1 = self.regs.get(rs1);
        self.regs.set(rd, self.pc + 4);
        self.pc = rs1.wrapping_add(imm);
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::*;

    #[test]
    fn test_lui() {
        let mut cpu = new_panic_io_cpu();

        cpu.pc = 0;
         
        // This seems weird but the imm value is calculated when parsing the instruction and not
        // when the instruction is executed. That's why we check if the reg x1 is the same as we
        // passed in
        cpu.lui(0x12345, 1);
        assert_eq!(cpu.regs.get(1), 0x12345);
        assert_eq!(cpu.pc, 4);
    }

    #[test]
    fn test_auipc() {
        let mut cpu = new_panic_io_cpu();

        cpu.pc = 0x40000000;
        cpu.auipc(0x3000000, 1);
        assert_eq!(cpu.regs.get(1), 0x43000000);
        assert_eq!(cpu.pc, 0x40000004);
    }

    #[test]
    fn test_jal() {
        let mut cpu = new_panic_io_cpu();

        cpu.pc = 0x40000000;
        cpu.jal(0x1000, 1);
        assert_eq!(cpu.regs.get(1), 0x40000004);
        assert_eq!(cpu.pc, 0x40001000);
    }

    #[test]
    fn test_jalr() {
        let mut cpu = new_panic_io_cpu();

        cpu.pc = 0x40000000;
        cpu.regs.set(2, 0x40000000);
        cpu.jalr(2, 0x1000, 1);
        assert_eq!(cpu.regs.get(1), 0x40000004);
        assert_eq!(cpu.pc, 0x40001000);
    }
}

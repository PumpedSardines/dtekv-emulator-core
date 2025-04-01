use crate::{
    cpu::Cpu,
    instruction::{ITypeImm, JTypeImm, UTypeImm},
    peripheral::Peripheral,
    register::Register,
};

impl<T: Peripheral<()>> Cpu<T> {
    pub(crate) fn lui(&mut self, imm: UTypeImm, rd: Register) {
        self.regs.set(rd, imm.as_u32());
        self.pc += 4;
    }

    pub(crate) fn auipc(&mut self, imm: UTypeImm, rd: Register) {
        self.regs.set(rd, self.pc.wrapping_add(imm.as_u32()));
        self.pc += 4;
    }

    pub(crate) fn jal(&mut self, imm: JTypeImm, rd: Register) {
        let imm = imm.as_u32();
        self.regs.set(rd, self.pc + 4);
        self.pc = self.pc.wrapping_add(imm);
    }

    pub(crate) fn jalr(&mut self, rs1: Register, imm: ITypeImm, rd: Register) {
        let imm = imm.as_u32();
        let rs1 = self.regs.get(rs1);
        self.regs.set(rd, self.pc + 4);
        self.pc = rs1.wrapping_add(imm);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{register::Register, test_utils::*};

    #[test]
    fn test_lui() {
        let mut cpu = new_panic_io_cpu();

        cpu.pc = 0;

        cpu.lui(UTypeImm::new(0x12345000).unwrap(), Register::T0);
        assert_eq!(cpu.regs.get(Register::T0), 0x12345000);
        assert_eq!(cpu.pc, 4);
    }

    #[test]
    fn test_auipc() {
        let mut cpu = new_panic_io_cpu();

        cpu.pc = 0x40000000;
        cpu.auipc(UTypeImm::new(0x3000000).unwrap(), Register::T0);
        assert_eq!(cpu.regs.get(Register::T0), 0x43000000);
        assert_eq!(cpu.pc, 0x40000004);
    }

    #[test]
    fn test_jal() {
        let mut cpu = new_panic_io_cpu();

        cpu.pc = 0x40000000;
        cpu.jal(JTypeImm::new(0x1000).unwrap(), Register::T0);
        assert_eq!(cpu.regs.get(Register::T0), 0x40000004);
        assert_eq!(cpu.pc, 0x40001000);
    }

    #[test]
    fn test_jalr() {
        let mut cpu = new_panic_io_cpu();

        cpu.pc = 0x40000000;
        cpu.regs.set(Register::T2, 0x40000000);
        cpu.jalr(Register::T2, ITypeImm::new(0x100).unwrap(), Register::T1);
        assert_eq!(cpu.regs.get(Register::T1), 0x40000004);
        assert_eq!(cpu.pc, 0x40000100);
    }
}

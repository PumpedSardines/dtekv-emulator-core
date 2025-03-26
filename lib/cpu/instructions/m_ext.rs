use crate::{cpu::Cpu, peripheral::{Peripheral}, register::Register};

fn debug_console_division_by_zero<T: Peripheral<()>>(cpu: &mut Cpu<T>) {
    #[cfg(feature = "debug-console")]
    if let Some(db) = &cpu.debug_console {
        db.borrow_mut().division_by_zero(cpu.pc);
    }
}

fn debug_console_remainder_by_zero<T: Peripheral<()>>(cpu: &mut Cpu<T>) {
    #[cfg(feature = "debug-console")]
    if let Some(db) = &cpu.debug_console {
        db.borrow_mut().remainder_by_zero(cpu.pc);
    }
}

impl<T: Peripheral<()>> Cpu<T> {
    pub(crate) fn mul(&mut self, rs1: Register, rs2: Register, rd: Register) {
        let rs1 = self.regs.get(rs1) as i32 as i64;
        let rs2 = self.regs.get(rs2) as i32 as i64;
        self.regs.set(rd, rs1.wrapping_mul(rs2) as u32);
        self.pc += 4;
    }

    pub(crate) fn mulh(&mut self, rs1: Register, rs2: Register, rd: Register) {
        let rs1 = self.regs.get(rs1) as i32 as i64;
        let rs2 = self.regs.get(rs2) as i32 as i64;
        let result = rs1.wrapping_mul(rs2);
        self.regs.set(rd, (result >> 32) as u32);
        self.pc += 4;
    }

    pub(crate) fn mulhu(&mut self, rs1: Register, rs2: Register, rd: Register) {
        let rs1 = self.regs.get(rs1) as u32 as u64;
        let rs2 = self.regs.get(rs2) as u32 as u64;
        let result = rs1.wrapping_mul(rs2);
        self.regs.set(rd, (result >> 32) as u32);
        self.pc += 4;
    }

    pub(crate) fn mulhsu(&mut self, rs1: Register, rs2: Register, rd: Register) {
        let rs1 = self.regs.get(rs1) as i32 as i64;
        let rs2 = self.regs.get(rs2) as u32 as i64;
        let result = rs1.wrapping_mul(rs2);
        self.regs.set(rd, (result >> 32) as u32);
        self.pc += 4;
    }

    pub(crate) fn div(&mut self, rs1: Register, rs2: Register, rd: Register) {
        let rs1 = self.regs.get(rs1) as i32;
        let rs2 = self.regs.get(rs2) as i32;
        if rs2 == 0 {
            debug_console_division_by_zero(self);

            self.regs.set(rd, 0xFFFFFFFF);
        } else {
            if rs1 == i32::MIN && rs2 == -1 {
                self.regs.set(rd, rs1 as u32);
            } else {
                self.regs.set(rd, (rs1 / rs2) as u32);
            }
        }
        self.pc += 4;
    }

    pub(crate) fn divu(&mut self, rs1: Register, rs2: Register, rd: Register) {
        let rs1 = self.regs.get(rs1);
        let rs2 = self.regs.get(rs2);
        if rs2 == 0 {
            debug_console_division_by_zero(self);

            self.regs.set(rd, 0xFFFFFFFF);
        } else {
            self.regs.set(rd, rs1 / rs2);
        }
        self.pc += 4;
    }

    pub(crate) fn rem(&mut self, rs1: Register, rs2: Register, rd: Register) {
        let rs1 = self.regs.get(rs1) as i32;
        let rs2 = self.regs.get(rs2) as i32;
        if rs2 == 0 {
            debug_console_remainder_by_zero(self);

            self.regs.set(rd, rs1 as u32);
        } else {
            if rs1 == i32::MIN && rs2 == -1 {
                self.regs.set(rd, 0);
            } else {
                self.regs.set(rd, (rs1 % rs2) as u32);
            }
        }
        self.pc += 4;
    }

    pub(crate) fn remu(&mut self, rs1: Register, rs2: Register, rd: Register) {
        let rs1 = self.regs.get(rs1);
        let rs2 = self.regs.get(rs2);
        if rs2 == 0 {
            debug_console_remainder_by_zero(self);

            self.regs.set(rd, rs1);
        } else {
            self.regs.set(rd, rs1 % rs2);
        }
        self.pc += 4;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;
    use test_case::test_case;

    #[test_case(3, 2 => 6; "regular")]
    #[test_case((-3i32) as u32, 2 => (-6i32) as u32; "first operand negative")]
    #[test_case(3, (-2i32) as u32 => (-6i32) as u32; "second operand negative")]
    #[test_case((-3i32) as u32, (-2i32) as u32 => 6; "both operand negative")]
    #[test_case(0x2000_0002, 0xF8 => 0x1F0; "overflow")]
    fn test_mul(a: u32, b: u32) -> u32 {
        let mut cpu = new_panic_io_cpu();
        cpu.pc = 0;
        cpu.regs.set(Register::T0, a);
        cpu.regs.set(Register::T1, b);
        cpu.mul(Register::T0, Register::T1, Register::T2);
        assert_eq!(cpu.pc, 4);
        cpu.regs.get(Register::T2)
    }

    #[test_case(3, 2 => 0; "underflow")]
    #[test_case((-3i32) as u32, 2 => 0xFFFF_FFFF; "first operand negative underflow")]
    #[test_case(3, (-2i32) as u32 => 0xFFFF_FFFF; "second operand negative underflow")]
    #[test_case((-3i32) as u32, (-2i32) as u32 => 0; "both operand negative underflow")]
    #[test_case(0x2000_0002, 0xF8 => 0x1F; "overflow positive")]
    #[test_case(0xA000_0002, 0xF8 => 0xFFFF_FFA3; "overflow negative")]
    fn test_mulh(a: u32, b: u32) -> u32 {
        let mut cpu = new_panic_io_cpu();
        cpu.pc = 0;
        cpu.regs.set(Register::T0, a);
        cpu.regs.set(Register::T1, b);
        cpu.mulh(Register::T0, Register::T1, Register::T2);
        assert_eq!(cpu.pc, 4);
        cpu.regs.get(Register::T2)
    }

    #[test_case(3, 2 => 0; "underflow")]
    #[test_case((-7i32) as u32, (-5i32) as u32 => 0xFFFF_FFF4; "both operand negative")]
    #[test_case(0x2000_0002, 0xF8 => 0x1F; "two positive overflow")]
    #[test_case(0xA000_0002, 0xF8 => 0x9b; "one negative overflow")]
    fn test_mulhu(a: u32, b: u32) -> u32 {
        let mut cpu = new_panic_io_cpu();
        cpu.pc = 0;
        cpu.regs.set(Register::T0, a);
        cpu.regs.set(Register::T1, b);
        cpu.mulhu(Register::T0, Register::T1, Register::T2);
        assert_eq!(cpu.pc, 4);
        cpu.regs.get(Register::T2)
    }
}

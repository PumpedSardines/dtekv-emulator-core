use crate::{cpu::Cpu, io};

impl<T: io::Data<()>> Cpu<T> {
    pub(crate) fn mul(&mut self, rs1: u8, rs2: u8, rd: u8) {
        let rs1 = self.regs.get(rs1) as i32 as i64;
        let rs2 = self.regs.get(rs2) as i32 as i64;
        self.regs.set(rd, rs1.wrapping_mul(rs2) as u32);
        self.pc += 4;
    }

    pub(crate) fn mulh(&mut self, rs1: u8, rs2: u8, rd: u8) {
        let rs1 = self.regs.get(rs1) as i32 as i64;
        let rs2 = self.regs.get(rs2) as i32 as i64;
        let result = rs1.wrapping_mul(rs2);
        self.regs.set(rd, (result >> 32) as u32);
        self.pc += 4;
    }

    pub(crate) fn mulhu(&mut self, rs1: u8, rs2: u8, rd: u8) {
        let rs1 = self.regs.get(rs1) as u32 as u64;
        let rs2 = self.regs.get(rs2) as u32 as u64;
        let result = rs1.wrapping_mul(rs2);
        self.regs.set(rd, (result >> 32) as u32);
        self.pc += 4;
    }

    pub(crate) fn mulhsu(&mut self, rs1: u8, rs2: u8, rd: u8) {
        let rs1 = self.regs.get(rs1) as i32 as i64;
        let rs2 = self.regs.get(rs2) as u32 as i64;
        let result = rs1.wrapping_mul(rs2);
        self.regs.set(rd, (result >> 32) as u32);
        self.pc += 4;
    }

    pub(crate) fn div(&mut self, rs1: u8, rs2: u8, rd: u8) {
        let rs1 = self.regs.get(rs1) as i32;
        let rs2 = self.regs.get(rs2) as i32;
        if rs2 == 0 {
            #[cfg(feature = "debug-console")]
            self.debug_console.division_by_zero(self.pc);

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

    pub(crate) fn divu(&mut self, rs1: u8, rs2: u8, rd: u8) {
        let rs1 = self.regs.get(rs1);
        let rs2 = self.regs.get(rs2);
        if rs2 == 0 {
            #[cfg(feature = "debug-console")]
            self.debug_console.division_by_zero(self.pc);

            self.regs.set(rd, 0xFFFFFFFF);
        } else {
            self.regs.set(rd, rs1 / rs2);
        }
        self.pc += 4;
    }

    pub(crate) fn rem(&mut self, rs1: u8, rs2: u8, rd: u8) {
        let rs1 = self.regs.get(rs1) as i32;
        let rs2 = self.regs.get(rs2) as i32;
        if rs2 == 0 {
            #[cfg(feature = "debug-console")]
            self.debug_console.remainder_by_zero(self.pc);

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

    pub(crate) fn remu(&mut self, rs1: u8, rs2: u8, rd: u8) {
        let rs1 = self.regs.get(rs1);
        let rs2 = self.regs.get(rs2);
        if rs2 == 0 {
            #[cfg(feature = "debug-console")]
            self.debug_console.remainder_by_zero(self.pc);

            self.regs.set(rd, rs1);
        } else {
            self.regs.set(rd, rs1 % rs2);
        }
        self.pc += 4;
    }
}

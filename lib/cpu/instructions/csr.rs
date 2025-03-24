use crate::{cpu::Cpu, io};

fn debug_console_csr_helper<T: io::Data<()>>(cpu: &mut Cpu<T>, csr: u32) {
    #[cfg(feature = "debug-console")]
    if !cpu.csr.emulated_csr(csr) {
        cpu.debug_console.access_useless_csr(csr, cpu.pc);
    }
}

impl<T: io::Data<()>> Cpu<T> {
    pub(crate) fn csrrw(&mut self, rs1: u8, imm: u32, rd: u8) {
        debug_console_csr_helper(self, imm);

        let value = self.csr.load(imm);
        self.csr.store(imm, self.regs.get(rs1));
        self.regs.set(rd, value);
        self.pc += 4;
    }

    pub(crate) fn csrrs(&mut self, rs1: u8, imm: u32, rd: u8) {
        debug_console_csr_helper(self, imm);

        let value = self.csr.load(imm);
        self.csr.store(imm, self.csr.load(imm) | self.regs.get(rs1));
        self.regs.set(rd, value);
        self.pc += 4;
    }

    pub(crate) fn csrrc(&mut self, rs1: u8, imm: u32, rd: u8) {
        debug_console_csr_helper(self, imm);

        let value = self.csr.load(imm);
        self.csr
            .store(imm, self.csr.load(imm) & !self.regs.get(rs1));
        self.regs.set(rd, value);
        self.pc += 4;
    }

    pub(crate) fn csrrwi(&mut self, _uimm: u8, _imm: u32, _rd: u8) {
        #[cfg(feature = "debug-console")]
        self.debug_console
            .instruction_not_implemented("CSRRWI", self.pc);

        self.pc += 4;
    }

    pub(crate) fn csrrsi(&mut self, uimm: u8, imm: u32, rd: u8) {
        debug_console_csr_helper(self, imm);

        let value = self.csr.load(imm);

        // NOTE: Dtekv differs from risc-v here:
        self.csr
            .store(imm, self.csr.load(imm) | (1 << (uimm as u32)));
        self.regs.set(rd, value);
        self.pc += 4;
    }

    pub(crate) fn csrrci(&mut self, _uimm: u8, _imm: u32, _rd: u8) {
        #[cfg(feature = "debug-console")]
        self.debug_console
            .instruction_not_implemented("csrrci", self.pc);

        self.pc += 4;
    }
}

#[cfg(test)]
mod tests {}

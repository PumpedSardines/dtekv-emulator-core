use crate::{cpu::Cpu, csr::Csr, io, register::Register};

fn debug_console_csr_helper<T: io::Data<()>>(cpu: &mut Cpu<T>, csr: Csr) {
    #[cfg(feature = "debug-console")]
    if !csr.meaningfully_emulated() {
        cpu.debug_console.access_useless_csr(csr, cpu.pc);
    }
}

impl<T: io::Data<()>> Cpu<T> {
    pub(crate) fn csrrw(&mut self, rs1: Register, csr: Csr, rd: Register) {
        debug_console_csr_helper(self, csr);

        let value = self.csr.load(csr);
        self.csr.store(csr, self.regs.get(rs1));
        self.regs.set(rd, value);
        self.pc += 4;
    }

    pub(crate) fn csrrs(&mut self, rs1: Register, csr: Csr, rd: Register) {
        debug_console_csr_helper(self, csr);

        let value = self.csr.load(csr);
        self.csr.store(csr, self.csr.load(csr) | self.regs.get(rs1));
        self.regs.set(rd, value);
        self.pc += 4;
    }

    pub(crate) fn csrrc(&mut self, rs1: Register, csr: Csr, rd: Register) {
        debug_console_csr_helper(self, csr);

        let value = self.csr.load(csr);
        self.csr
            .store(csr, self.csr.load(csr) & !self.regs.get(rs1));
        self.regs.set(rd, value);
        self.pc += 4;
    }

    pub(crate) fn csrrwi(&mut self, _imm: u32, _csr: Csr, _rd: Register) {
        #[cfg(feature = "debug-console")]
        self.debug_console
            .instruction_not_implemented("CSRRWI", self.pc);

        self.pc += 4;
    }

    pub(crate) fn csrrsi(&mut self, imm: u32, csr: Csr, rd: Register) {
        debug_console_csr_helper(self, csr);

        let value = self.csr.load(csr);

        // NOTE: Dtekv differs from risc-v here:
        self.csr
            .store(csr, self.csr.load(csr) | (1 << imm));
        self.regs.set(rd, value);
        self.pc += 4;
    }

    pub(crate) fn csrrci(&mut self, _imm: u32, _csr: Csr, _rd: Register) {
        #[cfg(feature = "debug-console")]
        self.debug_console
            .instruction_not_implemented("csrrci", self.pc);

        self.pc += 4;
    }
}

#[cfg(test)]
mod tests {}

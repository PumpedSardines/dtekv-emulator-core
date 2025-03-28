use crate::{cpu::Cpu, csr::Csr, peripheral::Peripheral, register::Register};

fn debug_console_csr_helper<T: Peripheral<()>>(cpu: &mut Cpu<T>, csr: Csr) {
    #[cfg(feature = "debug-console")]
    if !csr.meaningfully_emulated() {
        if let Some(db) = &cpu.debug_console {
            db.borrow_mut().access_useless_csr(csr, cpu.pc);
        }
    }
}

fn debug_console_not_implemented<T: Peripheral<()>>(cpu: &mut Cpu<T>, instruction: &'static str) {
    #[cfg(feature = "debug-console")]
    if let Some(db) = &cpu.debug_console {
        db.borrow_mut()
            .instruction_not_implemented(instruction, cpu.pc);
    }
}

impl<T: Peripheral<()>> Cpu<T> {
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
        debug_console_not_implemented(self, "CSRRWI");

        self.pc += 4;
    }

    pub(crate) fn csrrsi(&mut self, imm: u32, csr: Csr, rd: Register) {
        debug_console_csr_helper(self, csr);

        let value = self.csr.load(csr);
        self.csr.store(csr, self.csr.load(csr) | (1 << imm));
        self.regs.set(rd, value);
        self.pc += 4;
    }

    pub(crate) fn csrrci(&mut self, _imm: u32, _csr: Csr, _rd: Register) {
        debug_console_not_implemented(self, "CSRRCI");

        self.pc += 4;
    }
}

#[cfg(test)]
mod tests {
    // TODO: Add tests
}

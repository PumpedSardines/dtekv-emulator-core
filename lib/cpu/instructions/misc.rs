use crate::{cpu::Cpu, csr::Csr, interrupt::InterruptSignal, peripheral::Peripheral};

impl<T: Peripheral<()>> Cpu<T> {
    pub(crate) fn mret(&mut self) {
        self.pc = self.csr.load(Csr::MEPC);
        self.csr.set_mstatus_mie(self.csr.get_mstatus_mpie());
        self.csr.set_mstatus_mpie(true);
    }

    pub(crate) fn ecall(&mut self) {
        self.pc += 4;
        self.handle_interrupt(InterruptSignal::ENVIRONMENT_CALL_FROM_M_MODE);
    }
}

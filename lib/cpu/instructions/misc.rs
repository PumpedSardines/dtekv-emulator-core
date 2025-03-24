use crate::{
    cpu::{csr, Cpu},
    exception, io,
};

impl<T: io::Data<()>> Cpu<T> {
    pub(crate) fn mret(&mut self) {
        self.pc = self.csr.load(csr::MEPC);
        self.csr.set_mstatus_mie(self.csr.get_mstatus_mpie());
        self.csr.set_mstatus_mpie(true);
    }

    pub(crate) fn ecall(&mut self) {
        self.pc += 4;
        self.interrupt(exception::ENVIRONMENT_CALL_FROM_M_MODE);
    }
}

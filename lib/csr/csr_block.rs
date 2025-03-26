use super::Csr;

pub struct CsrBlock {
    csrs: [u32; 4096],
}

impl CsrBlock {
    pub fn new() -> Self {
        Self { csrs: [0; 4096] }
    }

    /// Set all CSRs to 0
    pub fn reset(&mut self) {
        for csr in self.csrs.iter_mut() {
            *csr = 0;
        }
    }

    pub fn get_mstatus_mpie(&self) -> bool {
        self.load(Csr::MSTATUS) & (1 << 7) != 0
    }

    pub fn set_mstatus_mpie(&mut self, value: bool) {
        let v = self.load(Csr::MSTATUS);
        let v = if value { v | 1 << 7 } else { v & !(1 << 7) };
        self.store(Csr::MSTATUS, v);
    }

    pub fn get_mstatus_mie(&self) -> bool {
        self.load(Csr::MSTATUS) & (1 << 3) != 0
    }

    /// Set's the mie bit in mstatus to either 0 or 1
    pub fn set_mstatus_mie(&mut self, value: bool) {
        let v = self.load(Csr::MSTATUS);
        let v = if value { v | 1 << 3 } else { v & !(1 << 3) };
        self.store(Csr::MSTATUS, v);
    }

    pub fn load(&self, csr: Csr) -> u32 {
        // SAFETY: Csr is guaranteed to be in the range 0..4096
        let csr = Into::<usize>::into(csr);
        debug_assert!(csr < 4096);
        unsafe { *self.csrs.get_unchecked(csr) }
    }

    pub fn store(&mut self, csr: Csr, value: u32) {
        // SAFETY: Csr is guaranteed to be in the range 0..4096
        let csr = Into::<usize>::into(csr);
        debug_assert!(csr < 4096);
        unsafe { *self.csrs.get_unchecked_mut(csr) = value }
    }
}

impl std::fmt::Debug for CsrBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Csr {{ ")?;

        for res in vec![Csr::MSTATUS, Csr::MIE, Csr::MEPC, Csr::MCAUSE]
            .into_iter()
            .map(|csr| {
                write!(
                    f,
                    "{}: {:08x}, ",
                    csr.name().expect("Csr is not named"),
                    self.load(csr)
                )
            })
        {
            res?;
        }

        write!(f, " ... }}")
    }
}

use super::Register;

#[derive(Clone)]
pub struct RegisterBlock {
    registers: [u32; 31],
}

impl RegisterBlock {
    pub fn new() -> RegisterBlock {
        RegisterBlock { registers: [0; 31] }
    }

    /// Get's the value of a certain register
    pub fn get(&self, reg: Register) -> u32 {
        if reg == Register::ZERO {
            return 0;
        }

        let reg = reg.as_usize() - 1;
        debug_assert!(reg < 31);

        // SAFETY: Register is guaranteed to be in the range 0..=31
        unsafe { *self.registers.get_unchecked(reg) }
    }

    /// Sets the value of a certain register
    pub fn set(&mut self, reg: Register, val: u32) {
        if reg == Register::ZERO {
            return;
        }

        let reg = reg.as_usize() - 1;
        debug_assert!(reg < 31);

        // SAFETY: Register is guaranteed to be in the range 0..31
        unsafe {
            *self.registers.get_unchecked_mut(reg) = val;
        }
    }

    /// Sets all registers to 0
    pub fn reset(&mut self) {
        for reg in self.registers.iter_mut() {
            *reg = 0;
        }
    }
}

impl std::fmt::Debug for RegisterBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = (0..32)
            .map(|i| {
                Register::new(i)
                    .map(|reg| format!("x{:02}: {:0x}", i, self.get(reg)))
                    .unwrap()
            })
            .collect::<Vec<_>>()
            .join(", ");

        write!(f, "Regs {{ {} }}", s)
    }
}

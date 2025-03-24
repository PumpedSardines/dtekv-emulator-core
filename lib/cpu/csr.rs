pub const MSTATUS: u32 = 0x300;
pub const MIE: u32 = 0x304;
pub const MEPC: u32 = 0x341;
pub const MTVEC: u32 = 0x305;
pub const MCAUSE: u32 = 0x342;

pub const MCYCLE: u32 = 0xB00;
pub const MCYCLEH: u32 = 0xB80;
pub const MINSTRET: u32 = 0xB02;
pub const MINSTRETH: u32 = 0xB82;

pub const MHPMCOUNTER3: u32 = 0xB03;
pub const MHPMCOUNTER3H: u32 = 0xB83;
pub const MHPMCOUNTER4: u32 = 0xB04;
pub const MHPMCOUNTER4H: u32 = 0xB84;
pub const MHPMCOUNTER5: u32 = 0xB05;
pub const MHPMCOUNTER5H: u32 = 0xB85;
pub const MHPMCOUNTER6: u32 = 0xB06;
pub const MHPMCOUNTER6H: u32 = 0xB86;
pub const MHPMCOUNTER7: u32 = 0xB07;
pub const MHPMCOUNTER7H: u32 = 0xB87;
pub const MHPMCOUNTER8: u32 = 0xB08;
pub const MHPMCOUNTER8H: u32 = 0xB88;
pub const MHPMCOUNTER9: u32 = 0xB09;
pub const MHPMCOUNTER9H: u32 = 0xB89;

pub fn csr_to_str(csr: u32) -> Option<&'static str> {
    Some(match csr {
        MSTATUS => "mstatus",
        MIE => "mie",
        MEPC => "mepc",
        MTVEC => "mtvec",
        MCAUSE => "mcause",
        MCYCLE => "mcycle",
        MCYCLEH => "mcycleh",
        MINSTRET => "minstret",
        MINSTRETH => "minstreth",
        MHPMCOUNTER3 => "mhpmcounter3",
        MHPMCOUNTER3H => "mhpmcounter3h",
        MHPMCOUNTER4 => "mhpmcounter4",
        MHPMCOUNTER4H => "mhpmcounter4h",
        MHPMCOUNTER5 => "mhpmcounter5",
        MHPMCOUNTER5H => "mhpmcounter5h",
        MHPMCOUNTER6 => "mhpmcounter6",
        MHPMCOUNTER6H => "mhpmcounter6h",
        MHPMCOUNTER7 => "mhpmcounter7",
        MHPMCOUNTER7H => "mhpmcounter7h",
        MHPMCOUNTER8 => "mhpmcounter8",
        MHPMCOUNTER8H => "mhpmcounter8h",
        MHPMCOUNTER9 => "mhpmcounter9",
        MHPMCOUNTER9H => "mhpmcounter9h",
        _ => return None,
    })
}

#[derive(Clone)]
pub struct CSR {
    pub csrs: [u32; 4096],
}

impl CSR {
    pub fn new() -> Self {
        CSR { csrs: [0; 4096] }
    }

    pub fn reset(&mut self) {
        for csr in self.csrs.iter_mut() {
            *csr = 0;
        }
    }

    pub fn get_mstatus_mpie(&self) -> bool {
        self.csrs[MSTATUS as usize] & (1 << 7) != 0
    }

    pub fn set_mstatus_mpie(&mut self, value: bool) {
        if value {
            self.csrs[MSTATUS as usize] |= 1 << 7;
        } else {
            self.csrs[MSTATUS as usize] &= !(1 << 7);
        }
    }

    pub fn get_mstatus_mie(&self) -> bool {
        self.csrs[MSTATUS as usize] & (1 << 3) != 0
    }

    pub fn set_mstatus_mie(&mut self, value: bool) {
        if value {
            self.csrs[MSTATUS as usize] |= 1 << 3;
        } else {
            self.csrs[MSTATUS as usize] &= !(1 << 3);
        }
    }

    pub fn clear_mstatus_mpp(&mut self) {
        self.csrs[MSTATUS as usize] &= !(0b11 << 11);
    }

    /// If a given CSR is emulated. By that meaning that there is a reason to read or write to it.
    ///
    /// # Arguments
    /// * `addr` - The address of the CSR to check.
    ///
    /// # Returns
    /// * `bool` - If the CSR is used and has meaning within the emulator.
    pub fn emulated_csr(&self, addr: u32) -> bool {
        match addr {
            MSTATUS | MIE | MEPC | MCAUSE => true,
            _ => false,
        }
    }

    pub fn load(&self, addr: u32) -> u32 {
        self.csrs[addr as usize]
    }

    pub fn store(&mut self, addr: u32, value: u32) {
        self.csrs[addr as usize] = value;
    }
}


impl std::fmt::Debug for CSR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Csr {{ ... }}")
    }
}

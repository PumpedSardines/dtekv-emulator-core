pub const MSTATUS: u32 = 0x300;
pub const MIE: u32 = 0x304;
pub const MTVEC: u32 = 0x305;
pub const MEPC: u32 = 0x341;
pub const MCAUSE: u32 = 0x342;
pub const MTVAL: u32 = 0x343;
pub const MIP: u32 = 0x344;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    User = 0b00,
    Supervisor = 0b01,
    Machine = 0b11,
}

#[derive(Clone)]
pub struct Csr {
    pub csrs: [u32; 4096],
    pub mode: Mode,
}

impl Csr {
    pub fn new() -> Self {
        Csr {
            csrs: [0; 4096],
            mode: Mode::Machine,
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

    pub fn get_mstatus_mpp(&self) -> Mode {
        match (self.csrs[MSTATUS as usize] >> 11) & 0b11 {
            0b00 => Mode::User,
            0b01 => Mode::Supervisor,
            0b11 => Mode::Machine,
            _ => panic!("Invalid mode"),
        }
    }

    pub fn clear_mstatus_mpp(&mut self) {
        self.csrs[MSTATUS as usize] &= !(0b11 << 11);
    }

    pub fn set_mstatus_mpp(&mut self, value: Mode) {
        self.clear_mstatus_mpp();
        self.csrs[MSTATUS as usize] |= (value as u32) << 11;
    }

    pub fn get_mtvec(&self) -> u32 {
        self.csrs[MTVEC as usize] & !1
    }


    pub fn read(&self, addr: u32) -> u32 {
        self.csrs[addr as usize]
    }

    pub fn write(&mut self, addr: u32, value: u32) {
        self.csrs[addr as usize] = value;
    }
}

impl std::fmt::Debug for Csr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Csr {{ ... }}")
    }
}

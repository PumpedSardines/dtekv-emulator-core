#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Csr(u16);

pub const MAX_CSR: u32 = 0xFFF;
pub const MIN_CSR: u32 = 0;

impl Csr {
    /// Creates a new CSR, returns None if the given CSR is not a valid address
    pub fn new(csr: u32) -> Option<Self> {
        if csr > MAX_CSR || csr < MIN_CSR {
            return None;
        } else {
            Some(unsafe { Csr::new_unchecked(csr as u16) })
        }
    }

    /// Creates a new CSR without checking if the given CSR is a valid address
    pub unsafe fn new_unchecked(csr: u16) -> Self {
        if (csr as u32) > MAX_CSR || (csr as u32) < MIN_CSR {
            if cfg!(debug_assertions) {
                unreachable!("CSR is set to an invalid value");
            } else {
                std::hint::unreachable_unchecked();
            }
        }
        Csr(csr)
    }

    /// If a given CSR is has a meaningful implementation. I.e if there is a reason to write and read
    /// from this register. This is useful for generating warnings when a CSR is accessed that wouldn't matter,
    /// informing the user that what is trying to be done is not implemented.
    pub fn meaningfully_emulated(&self) -> bool {
        match *self {
            Csr::MSTATUS | Csr::MIE | Csr::MEPC | Csr::MCAUSE => true,
            _ => false,
        }
    }
}

impl std::fmt::Debug for Csr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(name) = self.name() {
            write!(f, "Csr({})", name)
        } else {
            write!(f, "Csr({:x})", self.0)
        }
    }
}

impl TryFrom<u32> for Csr {
    type Error = ();

    fn try_from(code: u32) -> Result<Self, Self::Error> {
        Csr::new(code).ok_or(())
    }
}

impl Into<u16> for Csr {
    fn into(self) -> u16 {
        self.0
    }
}

impl Into<u32> for Csr {
    fn into(self) -> u32 {
        self.0 as u32
    }
}

impl Into<usize> for Csr {
    fn into(self) -> usize {
        // Since CSR can't be negative, this is safe
        self.0 as usize
    }
}

macro_rules! csr_list {
    ($(($const_name:ident, $code:expr, $desc:expr),)+) => {
impl Csr {
        $(
        pub const $const_name: Csr = Csr($code);
        )*

        pub fn name(&self) -> Option<&'static str> {
            match self.0 {
                $($code => Some($desc),)*
                _ => None
            }
        }
}
    };
}

csr_list! {
    (MSTATUS, 0x300, "mstatus"),
    (MIE, 0x304, "mie"),
    (MEPC, 0x341, "mepc"),
    (MTVEC, 0x305, "mtvec"),
    (MCAUSE, 0x342, "mcause"),
    (MCYCLE, 0xB00, "mcycle"),
    (MCYCLEH, 0xB80, "mcycleh"),
    (MINSTRET, 0xB02, "minstret"),
    (MINSTRETH, 0xB82, "minstreth"),
    (MHPMCOUNTER3, 0xB03, "mhpmcounter3"),
    (MHPMCOUNTER3H, 0xB83, "mhpmcounter3h"),
    (MHPMCOUNTER4, 0xB04, "mhpmcounter4"),
    (MHPMCOUNTER4H, 0xB84, "mhpmcounter4h"),
    (MHPMCOUNTER5, 0xB05, "mhpmcounter5"),
    (MHPMCOUNTER5H, 0xB85, "mhpmcounter5h"),
    (MHPMCOUNTER6, 0xB06, "mhpmcounter6"),
    (MHPMCOUNTER6H, 0xB86, "mhpmcounter6h"),
    (MHPMCOUNTER7, 0xB07, "mhpmcounter7"),
    (MHPMCOUNTER7H, 0xB87, "mhpmcounter7h"),
    (MHPMCOUNTER8, 0xB08, "mhpmcounter8"),
    (MHPMCOUNTER8H, 0xB88, "mhpmcounter8h"),
    (MHPMCOUNTER9, 0xB09, "mhpmcounter9"),
    (MHPMCOUNTER9H, 0xB89, "mhpmcounter9h"),
}

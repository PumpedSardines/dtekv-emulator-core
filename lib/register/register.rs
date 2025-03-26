#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Register(u8);

const MIN_REG: u32 = 0;
const MAX_REG: u32 = 31;

impl Register {
    /// Creates a new Register, only if the given register is a valid reg
    pub fn new(reg: u32) -> Option<Register> {
        if (MIN_REG..=MAX_REG).contains(&reg) {
            Some(unsafe { Register::new_unchecked(reg as u8) })
        } else {
            None
        }
    }

    /// Creates a new CSR without checking if the given CSR is a valid address
    pub unsafe fn new_unchecked(reg: u8) -> Self {
        if (reg as u32) > MAX_REG || (reg as u32) < MIN_REG {
            if cfg!(debug_assertions) {
                unreachable!("Register is set to an invalid value");
            } else {
                std::hint::unreachable_unchecked();
            }
        }
        Register(reg)
    }

    pub fn as_u32(&self) -> u32 {
        self.0 as u32
    }

    pub fn as_usize(&self) -> usize {
        self.0 as usize
    }
}

impl std::fmt::Debug for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Register({})", self.name())
    }
}

impl TryFrom<u32> for Register {
    type Error = ();

    fn try_from(reg: u32) -> Result<Self, Self::Error> {
        Register::new(reg).ok_or(())
    }
}

impl Into<u32> for Register {
    fn into(self) -> u32 {
        self.as_u32()
    }
}

impl Into<usize> for Register {
    fn into(self) -> usize {
        // Since Register can't be negative, this is safe
        self.as_usize()
    }
}

macro_rules! reg_list {
    ($(($name:ident, $code:expr, $desc:expr),)+) => {
impl Register {
        $(
        pub const $name: Register = Register($code);
        )*

        pub fn name(&self) -> &'static str {
            match self.0 {
                $($code => $desc,)*
                _ => panic!("Register is set to an invalid value")
            }
        }
}
    };
}

reg_list! {
    (ZERO, 0, "zero"),
    (RA, 1, "ra"),
    (SP, 2, "sp"),
    (GP, 3, "gp"),
    (TP, 4, "tp"),
    (T0, 5, "t0"),
    (T1, 6, "t1"),
    (T2, 7, "t2"),
    (S0, 8, "s0"),
    (S1, 9, "s1"),
    (A0, 10, "a0"),
    (A1, 11, "a1"),
    (A2, 12, "a2"),
    (A3, 13, "a3"),
    (A4, 14, "a4"),
    (A5, 15, "a5"),
    (A6, 16, "a6"),
    (A7, 17, "a7"),
    (S2, 18, "s2"),
    (S3, 19, "s3"),
    (S4, 20, "s4"),
    (S5, 21, "s5"),
    (S6, 22, "s6"),
    (S7, 23, "s7"),
    (S8, 24, "s8"),
    (S9, 25, "s9"),
    (S10, 26, "s10"),
    (S11, 27, "s11"),
    (T3, 28, "t3"),
    (T4, 29, "t4"),
    (T5, 30, "t5"),
    (T6, 31, "t6"),
}

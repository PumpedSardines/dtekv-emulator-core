#[derive(Clone)]
pub struct Regs {
    registers: [u32; 31],
}

impl Regs {
    pub fn new() -> Regs {
        Regs {
            registers: [0; 31],
        }
    }

    pub fn get(&self, reg: u8) -> u32 {
        if reg == 0 {
            return 0;
        }
        self.registers[reg as usize - 1]
    }

    pub fn set(&mut self, reg: u8, val: u32) {
        if reg == 0 {
            return;
        }
        self.registers[reg as usize - 1] = val;
    }

    pub fn reset(&mut self) {
        for reg in self.registers.iter_mut() {
            *reg = 0;
        }
    }
}

impl std::fmt::Debug for Regs {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut s = String::new();
        for (i, reg) in self.registers.iter().enumerate() {
            s.push_str(&format!("x{:02}: {:0x}", i + 1, reg));
            if i != self.registers.len() - 1 {
                s.push_str(", ");
            }
        }
        write!(f, "Regs {{ {} }}", s)
    }
}

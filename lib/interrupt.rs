//! Exception codes for the DTEK-V

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct InterruptSignal(u32, bool);

impl InterruptSignal {
    pub fn new(cause: u32, external: bool) -> Option<Self> {
        // Cause can't have highest bit set since that represents an hardware interrupt
        if cause & 0x80000000 == 1 {
            return None;
        }

        Some(unsafe { Self::new_unchecked(cause, external) })
    }

    unsafe fn new_unchecked(cause: u32, external: bool) -> Self {
        if cause & 0x80000000 == 1 {
            debug_assert!(cause & 0x80000000 == 0);
            std::hint::unreachable_unchecked();
        }
        Self(cause, external)
    }

    pub fn cause(&self) -> u32 {
        if self.external() {
            return self.0 | 0x80000000;
        } else {
            self.0 
        }
    }

    pub fn external(&self) -> bool {
        self.1
    }
}

macro_rules! interrupt_list {
    ($(($name:ident, $code:expr, $external:expr, $desc:expr),)+) => {
impl InterruptSignal {
    $(
    pub const $name: InterruptSignal = InterruptSignal($code, $external);
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

interrupt_list! {
    (INSTRUCTION_ADDRESS_MISALIGNED, 0, false, "Instruction address misaligned"),
    (ILLEGAL_INSTRUCTION, 2, false, "Illegal instruction"),
    (ENVIRONMENT_CALL_FROM_M_MODE, 11, false, "Environment call from M-mode"),
    (TIMER_INTERRUPT, 16, true, "Timer interrupt"),
    (SWITCH_INTERRUPT, 17, true, "Switch interrupt"),
    (BUTTON_INTERRUPT, 18, true, "Button interrupt"),
}

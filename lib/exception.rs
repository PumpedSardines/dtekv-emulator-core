//! Exception codes for the DTEK-V

pub struct Exception(u32, bool);

impl Exception {
    pub fn new(cause: u32, external: bool) -> Self {
        Self(cause, external)
    }

    pub fn cause(&self) -> u32 {
        self.0
    }

    pub fn external(&self) -> bool {
        self.1
    }
}

macro_rules! exception_list {
    ($(($name:ident, $code:expr, $external:expr, $desc:expr),)+) => {
impl Exception {
    $(
    pub const $name: Exception = Exception($code, $external);
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

exception_list! {
    (INSTRUCTION_ADDRESS_MISALIGNED, 0, false, "Instruction address misaligned"),
    (ILLEGAL_INSTRUCTION, 2, false, "Illegal instruction"),
    (ENVIRONMENT_CALL_FROM_M_MODE, 11, false, "Environment call from M-mode"),
    (TIMER_INTERRUPT, 16, true, "Timer interrupt"),
    (SWITCH_INTERRUPT, 17, true, "Switch interrupt"),
    (BUTTON_INTERRUPT, 18, true, "Button interrupt"),
}

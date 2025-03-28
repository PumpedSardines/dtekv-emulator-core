use crate::{interrupt::InterruptSignal, memory_mapped::MemoryMapped, utils};

use super::Peripheral;

pub const TIMER_LOWER_ADDR: u32 = 0x4000020;
pub const TIMER_HIGHER_ADDR: u32 = 0x400003f;
pub const TIMER_FEQ: u32 = 30_000_000;

#[derive(Clone)]
pub struct Timer {
    period: u32,
    running: bool,
    time_out: bool,
    cont: bool,
    irq: bool,
    clock: u32,
    timer: u32,
}

impl Default for Timer {
    fn default() -> Self {
        Self::new()
    }
}

impl Timer {
    /// Returns a new Memory object with a given size all set to 0
    pub fn new() -> Self {
        Timer {
            period: 0,
            running: false,
            time_out: true,
            cont: false,
            irq: false,
            clock: 0,
            timer: 0,
        }
    }

    /// Set the clock to a new value.
    /// 0 is the initial value, 1000 is 1 second
    pub fn update_clock(&mut self, clock: u32) {
        let last_clock = self.clock;
        // THis can potentially overflow, but come on you'd have to run this emulator for like 3
        // years for that to happen.
        self.clock = clock;
        if !self.running {
            return;
        }
        self.timer += (TIMER_FEQ / 1000) * (self.clock - last_clock);
        if self.timer >= self.period {
            self.timer -= self.period;
            self.time_out = true;
        }
    }

    fn should_interrupt(&self) -> bool {
        self.time_out && self.irq
    }
}

impl Peripheral<()> for Timer {
    fn poll_interrupt(&self) -> Option<InterruptSignal> {
        if self.should_interrupt() {
            Some(InterruptSignal::TIMER_INTERRUPT)
        } else {
            None
        }
    }
}

impl MemoryMapped<()> for Timer {
    fn load_byte(&self, addr: u32) -> Result<u8, ()> {
        let addr = addr - TIMER_LOWER_ADDR;
        let part = addr / 4;

        Ok(match part {
            0 => utils::get_in_u32(
                match (self.running, self.time_out) {
                    (true, true) => 3,
                    (true, false) => 2,
                    (false, true) => 1,
                    (false, false) => 0,
                },
                addr,
            ),
            1 => utils::get_in_u32(
                match (self.cont, self.irq) {
                    (true, true) => 3,
                    (true, false) => 2,
                    (false, true) => 1,
                    (false, false) => 0,
                },
                addr,
            ),
            _ => 0,
        })
    }

    fn store_byte(&mut self, addr: u32, byte: u8) -> Result<(), ()> {
        let addr = addr - TIMER_LOWER_ADDR;
        let part = addr / 4;
        let i = addr % 4;

        match part {
            0 => {
                if i == 0 {
                    self.time_out = byte & 1 == 1;
                }
            } // Data address, can store here
            1 => {
                if i == 0 {
                    self.irq = byte & 1 != 0;
                    self.cont = byte & 2 != 0;
                    let start = byte & 4 != 0;
                    let stop = byte & 8 != 0;

                    if start && stop {
                        unimplemented!("Sending start and stop signal is not supported");
                    }

                    if start {
                        self.running = true;
                    } else if stop {
                        self.running = false;
                    }
                }
            } // Direction address, can store here, but changes nothing
            2 => {
                // Lower 16 bits of the state
                if i == 0 || i == 1 {
                    self.period = utils::set_in_u32(self.period, byte, i);
                }
            }
            3 => {
                // Lower 16 bits of the state
                if i == 0 || i == 1 {
                    self.period = utils::set_in_u32(self.period, byte, i + 2);
                }
            }
            _ => unreachable!("The timer address space is only 4 words long, if this error happens, update the bus module"),
        };

        Ok(())
    }
}

impl std::fmt::Debug for Timer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Timer {{ Running: {}, IRQ: {} }}",
            self.running, self.irq
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![(0x3333, 0x2 << 2), (0x0, 0x3 << 2)] => 0x3333; "store only lower bits")]
    #[test_case(vec![(0x0, 0x2 << 2), (0xAAAA, 0x3 << 2)] => 0xAAAA0000; "store only higher bits")]
    #[test_case(vec![(0x33330000, 0x2 << 2), (0xAAAA0000, 0x3 << 2)] => 0x0; "upper bits ignored")]
    pub fn test_set_period(writes: Vec<(u32, u32)>) -> u32 {
        let mut timer = Timer::new();

        for (byte, addr) in writes {
            timer.store_word(TIMER_LOWER_ADDR + addr, byte).unwrap();
        }

        timer.period
    }

    #[test]
    pub fn test_set_time_out() {
        let mut timer = Timer::new();
        timer.time_out = false;
        // Set TIME OUT bit
        timer.store_byte(TIMER_LOWER_ADDR, 0b1).unwrap();
        assert!(timer.time_out);
        timer.store_byte(TIMER_LOWER_ADDR, 0b0).unwrap();
        assert!(!timer.time_out);
    }

    #[test]
    pub fn test_set_irq() {
        let mut timer = Timer::new();
        // Set IRQ bit
        timer.store_byte(TIMER_LOWER_ADDR + 4, 0b1).unwrap();
        assert!(timer.irq);
        timer.store_byte(TIMER_LOWER_ADDR + 4, 0b0).unwrap();
        assert!(!timer.irq);
    }

    #[test]
    pub fn test_set_start_stop() {
        let mut timer = Timer::new();
        // Set START bit
        timer.store_byte(TIMER_LOWER_ADDR + 4, 0b1).unwrap();
        assert!(timer.irq);
        // Set STOP bit
        timer.store_byte(TIMER_LOWER_ADDR + 4, 0b0).unwrap();
        assert!(!timer.irq);
    }

    #[test]
    pub fn test_update_clock_correctly() {
        let mut timer = Timer::new();
        // Set a high period to make sure not to trigger overflow
        timer.period = 60_000_000;
        timer.update_clock(1);
        timer.running = true;
        timer.update_clock(2);
        assert_eq!(timer.timer, TIMER_FEQ / 1000);
        timer.timer = 0;
        timer.update_clock(1002);
        assert_eq!(timer.timer, TIMER_FEQ);
    }

    #[test]
    pub fn test_update_clock_overflow() {
        let mut timer = Timer::new();
        // Set a high period to make sure not to trigger overflow
        timer.period = 300_000; // 100 times a second
        timer.running = true;
        timer.time_out = false;
        timer.update_clock(1);
        assert_eq!(timer.timer, 30_000);
        assert!(!timer.time_out);

        timer.update_clock(11);
        assert_eq!(timer.timer, 30_000);
        assert!(timer.time_out);
    }

    #[test]
    pub fn test_interrupt() {
        let mut timer = Timer::new();
        timer.period = 300_000; // 100 times a second
        timer.running = true;
        timer.time_out = false;
        timer.irq = true;

        timer.update_clock(10);

        assert!(timer.poll_interrupt().is_some());
    }
}

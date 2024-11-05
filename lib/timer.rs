use crate::LoadStore;
use std::time::{Duration, Instant};

const CLOCK_FEQ: u32 = 30_000_000;

// #define IO_TIMER_STATUS_ADDR (volatile int *)0x04000020
// #define IO_TIMER_CONTROL_ADDR (volatile int *)0x04000024
// #define IO_TIMER_PERIOD_LOWER_ADDR (volatile int *)0x04000028
// #define IO_TIMER_PERIOD_HIGHER_ADDR (volatile int *)0x0400002C

#[derive(Clone)]
pub struct Timer {
    pub state: u32,
    pub period: u32,
    pub running: bool,
    pub time_out: bool,
    pub cont: bool,
    pub irq: bool,
    pub period_duration: Duration,
    pub clock_start: Instant,
}

impl Timer {
    /// Returns a new Memory object with a given size all set to 0
    pub fn new() -> Self {
        Timer {
            state: 0,
            period: 0,
            running: false,
            time_out: true,
            cont: false,
            irq: false,
            period_duration: Duration::new(0, 0),
            clock_start: Instant::now(),
        }
    }

    fn set_part_period(&mut self, i: u32, byte: u8) {
        match i {
            0 => self.period = (self.period & 0xFFFF_FF00) | ((byte as u32) << 0),
            1 => self.period = (self.period & 0xFFFF_00FF) | ((byte as u32) << 8),
            2 => self.period = (self.period & 0xFF00_FFFF) | ((byte as u32) << 16),
            3 => self.period = (self.period & 0x00FF_FFFF) | ((byte as u32) << 24),
            _ => unreachable!(),
        }

        self.period_duration =
            Duration::from_nanos(((self.period as u64) * 1_000_000_000) / CLOCK_FEQ as u64);
    }

    pub fn update(&mut self) {
        if self.running {
            let elapsed = self.clock_start.elapsed();
            if elapsed >= self.period_duration {
                self.time_out = true;
                self.clock_start = Instant::now();
            }
        }
    }

    pub fn should_interrupt(&self) -> bool {
        self.time_out && self.irq
    }
}

impl LoadStore for Timer {
    fn load_byte(&self, addr: u32) -> u8 {
        let part = addr / 4;
        let i = addr % 4;

        match part {
            0 => {
                if i == 0 {
                    let mut res: u8 = 0;

                    if self.time_out {
                        res |= 1 << 0
                    }

                    if self.running {
                        res |= 1 << 1
                    }

                    res
                } else {
                    0
                }
            }
            1 => {
                if i == 0 {
                    let mut res: u8 = 0;

                    if self.irq {
                        res |= 1 << 0
                    }

                    if self.cont {
                        res |= 1 << 1
                    }

                    res
                } else {
                    0
                }
            }
            _ => 0,
        }
    }

    fn store_byte(&mut self, addr: u32, byte: u8) {
        let part = addr / 4;
        let i = addr % 4;

        match part {
            0 => {
                if i == 0 {
                    let old_time_out = self.time_out;
                    self.time_out = byte & 1 == 1;

                    if old_time_out && !self.time_out {
                        self.clock_start = Instant::now();
                    }
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
                match i {
                    0 => self.set_part_period(i, byte),
                    1 => self.set_part_period(i, byte),
                    _ => {}
                }
            }
            3 => {
                // Upper 16 bits of the state
                match i {
                    0 => self.set_part_period(i + 2, byte),
                    1 => self.set_part_period(i + 2, byte),
                    _ => {}
                }
            }
            _ => unreachable!(),
        };
    }
}

impl std::fmt::Debug for Timer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Timer {{ Running: {}, IRQ: {}, Duration: {:?} }}",
            self.running, self.irq, self.period_duration
        )
    }
}

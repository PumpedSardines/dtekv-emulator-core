use crate::{exception, io, Data};

const SDRAM_LOWER_ADDR: u32 = 0x00000000;
const SDRAM_HIGHER_ADDR: u32 = 0x3ffffff + 1;
const HEX_DISPLAY_LOWER_ADDR: u32 = 0x04000050;
const HEX_DISPLAY_HIGHER_ADDR: u32 = 0x04000050 + 0x10 * 5 + 4;
const SWITCH_LOWER_ADDR: u32 = 0x04000010;
const SWITCH_HIGHER_ADDR: u32 = 0x400001c + 4;
const UART_LOWER_ADDR: u32 = 0x04000040;
const UART_HIGHER_ADDR: u32 = 0x04000044 + 4;
const BUTTON_LOWER_ADDR: u32 = 0x040000d0;
const BUTTON_HIGHER_ADDR: u32 = 0x040000dc + 4;
const TIMER_LOWER_ADDR: u32 = 0x4000020;
const TIMER_HIGHER_ADDR: u32 = 0x400003c + 4;
const VGA_DMA_LOWER_ADDR: u32 = 0x4000100;
const VGA_DMA_HIGHER_ADDR: u32 = 0x400010c + 4;
const VGA_LOWER_ADDR: u32 = 0x08000000;
const VGA_HIGHER_ADDR: u32 = 0x08000000 + 320 * 240;

#[derive(Debug)]
pub struct Bus {
    pub mem: io::Memory,
    pub hex_display: io::HexDisplay,
    pub switch: io::Switch,
    pub uart: io::Uart,
    pub button: io::Button,
    pub vga: io::Vga,
    pub timer: io::Timer,
}

impl Bus {
    pub fn new() -> Self {
        Bus {
            mem: io::Memory::new(),
            hex_display: io::HexDisplay::new(),
            switch: io::Switch::new(),
            uart: io::Uart::new(),
            button: io::Button::new(),
            vga: io::Vga::new(),
            timer: io::Timer::new(),
        }
    }

    /// If an interrupt signal is pending, returns the cause
    pub fn should_interrupt(&self) -> Option<u32> {
        if self.switch.should_interrupt() {
            Some(exception::SWITCH_INTERRUPT);
        } else if self.button.should_interrupt() {
            Some(exception::BUTTON_INTERRUPT);
        } else if self.timer.should_interrupt() {
            Some(exception::TIMER_INTERRUPT);
        }
        None
    }

    pub fn load_at<K: Into<u8>, T: IntoIterator<Item = K>>(&mut self, offset: u32, bin: T)  {
        for (i, byte) in bin.into_iter().enumerate() {
            self.mem.store_byte(offset + i as u32, byte.into()).unwrap();
        }
    }


    pub fn clock(&mut self) {
        // Clock all io devices
        self.timer.clock();
    }
}

impl Data<()> for Bus {
    fn load_byte(&self, addr: u32) -> Result<u8, ()> {
        match addr {
            SDRAM_LOWER_ADDR..SDRAM_HIGHER_ADDR => self.mem.load_byte(addr),
            HEX_DISPLAY_LOWER_ADDR..HEX_DISPLAY_HIGHER_ADDR => {
                self.hex_display.load_byte(addr - HEX_DISPLAY_LOWER_ADDR)
            }
            SWITCH_LOWER_ADDR..SWITCH_HIGHER_ADDR => {
                self.switch.load_byte(addr - SWITCH_LOWER_ADDR)
            }
            UART_LOWER_ADDR..UART_HIGHER_ADDR => self.uart.load_byte(addr - UART_LOWER_ADDR),
            BUTTON_LOWER_ADDR..BUTTON_HIGHER_ADDR => {
                self.button.load_byte(addr - BUTTON_LOWER_ADDR)
            }
            VGA_LOWER_ADDR..VGA_HIGHER_ADDR => self.vga.load_byte(addr - VGA_LOWER_ADDR),
            VGA_DMA_LOWER_ADDR..VGA_DMA_HIGHER_ADDR => {
                unimplemented!("Fetching VGA DMA not implemented")
            }
            TIMER_LOWER_ADDR..TIMER_HIGHER_ADDR => self.timer.load_byte(addr - TIMER_LOWER_ADDR),
            _ => Err(()),
        }
    }

    fn store_byte(&mut self, addr: u32, byte: u8) -> Result<(), ()> {
        match addr {
            SDRAM_LOWER_ADDR..SDRAM_HIGHER_ADDR => self.mem.store_byte(addr, byte),
            HEX_DISPLAY_LOWER_ADDR..HEX_DISPLAY_HIGHER_ADDR => self
                .hex_display
                .store_byte(addr - HEX_DISPLAY_LOWER_ADDR, byte),
            SWITCH_LOWER_ADDR..SWITCH_HIGHER_ADDR => {
                self.switch.store_byte(addr - SWITCH_LOWER_ADDR, byte)
            }
            UART_LOWER_ADDR..UART_HIGHER_ADDR => self.uart.store_byte(addr - UART_LOWER_ADDR, byte),
            BUTTON_LOWER_ADDR..BUTTON_HIGHER_ADDR => {
                self.button.store_byte(addr - BUTTON_LOWER_ADDR, byte)
            }
            VGA_LOWER_ADDR..VGA_HIGHER_ADDR => self.vga.store_byte(addr - VGA_LOWER_ADDR, byte),
            VGA_DMA_LOWER_ADDR..VGA_DMA_HIGHER_ADDR => {
                Ok(()) // Ignore writes to VGA DMA
            }
            TIMER_LOWER_ADDR..TIMER_HIGHER_ADDR => {
                self.timer.store_byte(addr - TIMER_LOWER_ADDR, byte)
            }
            _ => Err(()),
        }
    }
}

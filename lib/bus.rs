use crate::{Button, Data, HexDisplay, LoadStore, Memory, Switch, Timer, Uart, Vga};

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

#[derive(Debug, Clone)]
pub struct Bus {
    pub mem: Memory,
    pub hex_display: HexDisplay,
    pub switch: Switch,
    pub uart: Uart,
    pub button: Button,
    pub vga: Vga,
    pub timer: Timer,
}

impl Data for Bus {
    fn load_halfword(&self, addr: u32) -> u16 {
        u16::from_be_bytes([self.load_byte(addr + 1), self.load_byte(addr + 0)])
    }

    fn load_word(&self, addr: u32) -> u32 {
        u32::from_be_bytes([
            self.load_byte(addr + 3),
            self.load_byte(addr + 2),
            self.load_byte(addr + 1),
            self.load_byte(addr + 0),
        ])
    }

    fn store_halfword(&mut self, addr: u32, halfword: u16) {
        let bytes = halfword.to_be_bytes();
        self.store_byte(addr + 0, bytes[1]);
        self.store_byte(addr + 1, bytes[0]);
    }

    fn store_word(&mut self, addr: u32, word: u32) {
        let bytes = word.to_be_bytes();
        self.store_byte(addr + 0, bytes[3]);
        self.store_byte(addr + 1, bytes[2]);
        self.store_byte(addr + 2, bytes[1]);
        self.store_byte(addr + 3, bytes[0]);
    }
}

impl LoadStore for Bus {
    fn load_byte(&self, addr: u32) -> u8 {
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
            VGA_DMA_LOWER_ADDR..VGA_DMA_HIGHER_ADDR => unimplemented!("Fetching VGA DMA not implemented"),
            TIMER_LOWER_ADDR..TIMER_HIGHER_ADDR => {
                self.timer.load_byte(addr - TIMER_LOWER_ADDR)
            },
            // TODO: Implement this
            _ => panic!("Invalid address: {:#010x}", addr),
        }
    }

    fn store_byte(&mut self, addr: u32, byte: u8) {
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
            VGA_DMA_LOWER_ADDR..VGA_DMA_HIGHER_ADDR => {},
            TIMER_LOWER_ADDR..TIMER_HIGHER_ADDR => {
                self.timer.store_byte(addr - TIMER_LOWER_ADDR, byte)
            },
            // TODO: Implement this
            _ => panic!("Invalid address: {:#010x}", addr),
        }
    }
}

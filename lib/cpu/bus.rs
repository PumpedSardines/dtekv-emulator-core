use std::{cell::RefCell, rc::Rc};

use crate::{
    io::{self, Device},
    Data,
};

#[derive(Debug)]
pub struct Bus<
    SDRam: io::Device<()>,
    Button: io::Device<()>,
    HexDisplay: io::Device<()>,
    LedStrip: io::Device<()>,
    Switch: io::Device<()>,
    Timer: io::Device<()>,
    Uart: io::Device<()>,
    VgaBuffer: io::Device<()>,
    VgaDma: io::Device<()>,
> {
    pub sdram: SDRam,
    pub button: Button,
    pub hex_display: HexDisplay,
    pub led_strip: LedStrip,
    pub switch: Switch,
    pub timer: Timer,
    pub uart: Uart,
    pub vga_buffer: VgaBuffer,
    pub vga_dma: VgaDma,
}

impl<
        SDRam: io::Device<()>,
        Button: io::Device<()>,
        HexDisplay: io::Device<()>,
        LedStrip: io::Device<()>,
        Switch: io::Device<()>,
        Timer: io::Device<()>,
        Uart: io::Device<()>,
        VgaBuffer: io::Device<()>,
        VgaDma: io::Device<()>,
    > io::Device<()>
    for Bus<SDRam, Button, HexDisplay, LedStrip, Switch, Timer, Uart, VgaBuffer, VgaDma>
{
    fn clock(&mut self) {
        self.sdram.clock();
        self.button.clock();
        self.hex_display.clock();
        self.led_strip.clock();
        self.switch.clock();
        self.timer.clock();
        self.uart.clock();
        self.vga_buffer.clock();
        self.vga_dma.clock();
    }
}

impl<
        SDRam: io::Device<()>,
        Button: io::Device<()>,
        HexDisplay: io::Device<()>,
        LedStrip: io::Device<()>,
        Switch: io::Device<()>,
        Timer: io::Device<()>,
        Uart: io::Device<()>,
        VgaBuffer: io::Device<()>,
        VgaDma: io::Device<()>,
    > io::Interruptable
    for Bus<SDRam, Button, HexDisplay, LedStrip, Switch, Timer, Uart, VgaBuffer, VgaDma>
{
    fn interrupt(&self) -> Option<u32> {
        if let Some(int) = self.timer.interrupt() {
            return Some(int);
        } else if let Some(int) = self.button.interrupt() {
            return Some(int);
        } else if let Some(int) = self.switch.interrupt() {
            return Some(int);
        } else if let Some(int) = self.led_strip.interrupt() {
            return Some(int);
        } else if let Some(int) = self.hex_display.interrupt() {
            return Some(int);
        } else if let Some(int) = self.sdram.interrupt() {
            return Some(int);
        } else if let Some(int) = self.uart.interrupt() {
            return Some(int);
        } else if let Some(int) = self.vga_buffer.interrupt() {
            return Some(int);
        } else if let Some(int) = self.vga_dma.interrupt() {
            return Some(int);
        }
        None
    }
}

impl<
        SDRam: io::Device<()>,
        Button: io::Device<()>,
        HexDisplay: io::Device<()>,
        LedStrip: io::Device<()>,
        Switch: io::Device<()>,
        Timer: io::Device<()>,
        Uart: io::Device<()>,
        VgaBuffer: io::Device<()>,
        VgaDma: io::Device<()>,
    > Data<()>
    for Bus<SDRam, Button, HexDisplay, LedStrip, Switch, Timer, Uart, VgaBuffer, VgaDma>
{
    fn load_byte(&self, addr: u32) -> Result<u8, ()> {
        match addr {
            io::SDRAM_LOWER_ADDR..=io::SDRAM_HIGHER_ADDR => self.sdram.load_byte(addr),
            io::BUTTON_LOWER_ADDR..=io::BUTTON_HIGHER_ADDR => self.button.load_byte(addr),
            io::HEX_DISPLAY_LOWER_ADDR..=io::HEX_DISPLAY_HIGHER_ADDR => self.hex_display.load_byte(addr),
            io::LED_STRIP_LOWER_ADDR..=io::LED_STRIP_HIGHER_ADDR => self.led_strip.load_byte(addr),
            io::SWITCH_LOWER_ADDR..=io::SWITCH_HIGHER_ADDR => self.switch.load_byte(addr),
            io::TIMER_LOWER_ADDR..=io::TIMER_HIGHER_ADDR => self.timer.load_byte(addr),
            io::UART_LOWER_ADDR..=io::UART_HIGHER_ADDR => self.uart.load_byte(addr),
            io::VGA_BUFFER_LOWER_ADDR..=io::VGA_BUFFER_HIGHER_ADDR => self.vga_buffer.load_byte(addr),
            io::VGA_DMA_LOWER_ADDR..=io::VGA_DMA_HIGHER_ADDR => self.vga_dma.load_byte(addr),
            _ => Err(()),
        }
    }

    fn store_byte(&mut self, addr: u32, byte: u8) -> Result<(), ()> {
        match addr {
            io::SDRAM_LOWER_ADDR..=io::SDRAM_HIGHER_ADDR => self.sdram.store_byte(addr, byte),
            io::BUTTON_LOWER_ADDR..=io::BUTTON_HIGHER_ADDR => self.button.store_byte(addr, byte),
            io::HEX_DISPLAY_LOWER_ADDR..=io::HEX_DISPLAY_HIGHER_ADDR => self.hex_display.store_byte(addr, byte),
            io::LED_STRIP_LOWER_ADDR..=io::LED_STRIP_HIGHER_ADDR => self.led_strip.store_byte(addr, byte),
            io::SWITCH_LOWER_ADDR..=io::SWITCH_HIGHER_ADDR => self.switch.store_byte(addr, byte),
            io::TIMER_LOWER_ADDR..=io::TIMER_HIGHER_ADDR => self.timer.store_byte(addr, byte),
            io::UART_LOWER_ADDR..=io::UART_HIGHER_ADDR => self.uart.store_byte(addr, byte),
            io::VGA_BUFFER_LOWER_ADDR..=io::VGA_BUFFER_HIGHER_ADDR => self.vga_buffer.store_byte(addr, byte),
            io::VGA_DMA_LOWER_ADDR..=io::VGA_DMA_HIGHER_ADDR => self.vga_dma.store_byte(addr, byte),
            _ => Err(()),
        }
    }

    fn load_halfword(&self, addr: u32) -> Result<u16, ()> {
        match addr {
            io::SDRAM_LOWER_ADDR..=io::SDRAM_HIGHER_ADDR => self.sdram.load_halfword(addr),
            io::BUTTON_LOWER_ADDR..=io::BUTTON_HIGHER_ADDR => self.button.load_halfword(addr),
            io::HEX_DISPLAY_LOWER_ADDR..=io::HEX_DISPLAY_HIGHER_ADDR => self.hex_display.load_halfword(addr),
            io::LED_STRIP_LOWER_ADDR..=io::LED_STRIP_HIGHER_ADDR => self.led_strip.load_halfword(addr),
            io::SWITCH_LOWER_ADDR..=io::SWITCH_HIGHER_ADDR => self.switch.load_halfword(addr),
            io::TIMER_LOWER_ADDR..=io::TIMER_HIGHER_ADDR => self.timer.load_halfword(addr),
            io::UART_LOWER_ADDR..=io::UART_HIGHER_ADDR => self.uart.load_halfword(addr),
            io::VGA_BUFFER_LOWER_ADDR..=io::VGA_BUFFER_HIGHER_ADDR => self.vga_buffer.load_halfword(addr),
            io::VGA_DMA_LOWER_ADDR..=io::VGA_DMA_HIGHER_ADDR => self.vga_dma.load_halfword(addr),
            _ => Err(()),
        }
    }

    fn store_halfword(&mut self, addr: u32, halfword: u16) -> Result<(), ()> {
        match addr {
            io::SDRAM_LOWER_ADDR..=io::SDRAM_HIGHER_ADDR => self.sdram.store_halfword(addr, halfword),
            io::BUTTON_LOWER_ADDR..=io::BUTTON_HIGHER_ADDR => self.button.store_halfword(addr, halfword),
            io::HEX_DISPLAY_LOWER_ADDR..=io::HEX_DISPLAY_HIGHER_ADDR => self.hex_display.store_halfword(addr, halfword),
            io::LED_STRIP_LOWER_ADDR..=io::LED_STRIP_HIGHER_ADDR => self.led_strip.store_halfword(addr, halfword),
            io::SWITCH_LOWER_ADDR..=io::SWITCH_HIGHER_ADDR => self.switch.store_halfword(addr, halfword),
            io::TIMER_LOWER_ADDR..=io::TIMER_HIGHER_ADDR => self.timer.store_halfword(addr, halfword),
            io::UART_LOWER_ADDR..=io::UART_HIGHER_ADDR => self.uart.store_halfword(addr, halfword),
            io::VGA_BUFFER_LOWER_ADDR..=io::VGA_BUFFER_HIGHER_ADDR => self.vga_buffer.store_halfword(addr, halfword),
            io::VGA_DMA_LOWER_ADDR..=io::VGA_DMA_HIGHER_ADDR => self.vga_dma.store_halfword(addr, halfword),
            _ => Err(()),
        }
    }

    fn load_word(&self, addr: u32) -> Result<u32, ()> {
        match addr {
            io::SDRAM_LOWER_ADDR..=io::SDRAM_HIGHER_ADDR => self.sdram.load_word(addr),
            io::BUTTON_LOWER_ADDR..=io::BUTTON_HIGHER_ADDR => self.button.load_word(addr),
            io::HEX_DISPLAY_LOWER_ADDR..=io::HEX_DISPLAY_HIGHER_ADDR => self.hex_display.load_word(addr),
            io::LED_STRIP_LOWER_ADDR..=io::LED_STRIP_HIGHER_ADDR => self.led_strip.load_word(addr),
            io::SWITCH_LOWER_ADDR..=io::SWITCH_HIGHER_ADDR => self.switch.load_word(addr),
            io::TIMER_LOWER_ADDR..=io::TIMER_HIGHER_ADDR => self.timer.load_word(addr),
            io::UART_LOWER_ADDR..=io::UART_HIGHER_ADDR => self.uart.load_word(addr),
            io::VGA_BUFFER_LOWER_ADDR..=io::VGA_BUFFER_HIGHER_ADDR => self.vga_buffer.load_word(addr),
            io::VGA_DMA_LOWER_ADDR..=io::VGA_DMA_HIGHER_ADDR => self.vga_dma.load_word(addr),
            _ => Err(()),
        }
    }

    fn store_word(&mut self, addr: u32, word: u32) -> Result<(), ()> {
        match addr {
            io::SDRAM_LOWER_ADDR..=io::SDRAM_HIGHER_ADDR => self.sdram.store_word(addr, word),
            io::BUTTON_LOWER_ADDR..=io::BUTTON_HIGHER_ADDR => self.button.store_word(addr, word),
            io::HEX_DISPLAY_LOWER_ADDR..=io::HEX_DISPLAY_HIGHER_ADDR => self.hex_display.store_word(addr, word),
            io::LED_STRIP_LOWER_ADDR..=io::LED_STRIP_HIGHER_ADDR => self.led_strip.store_word(addr, word),
            io::SWITCH_LOWER_ADDR..=io::SWITCH_HIGHER_ADDR => self.switch.store_word(addr, word),
            io::TIMER_LOWER_ADDR..=io::TIMER_HIGHER_ADDR => self.timer.store_word(addr, word),
            io::UART_LOWER_ADDR..=io::UART_HIGHER_ADDR => self.uart.store_word(addr, word),
            io::VGA_BUFFER_LOWER_ADDR..=io::VGA_BUFFER_HIGHER_ADDR => self.vga_buffer.store_word(addr, word),
            io::VGA_DMA_LOWER_ADDR..=io::VGA_DMA_HIGHER_ADDR => self.vga_dma.store_word(addr, word),
            _ => Err(()),
        }
    }
}

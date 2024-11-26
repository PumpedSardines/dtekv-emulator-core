use std::{cell::RefCell, rc::Rc};

/// Test larger programs using the emulator to ensure the CPU is working correctly
use dtekv_emulator_core::*;

#[test]
fn test_hex_display() {
    // Program that stores the bit mask for the number 9 in the hex display

    let bus = cpu::Bus {
        sdram: io::SDRam::new(),
        button: io::Button::new(),
        hex_display: io::HexDisplay::new(),
        led_strip: io::LEDStrip::new(),
        switch: io::Switch::new(),
        timer: io::Timer::new(),
        uart: io::Uart::new(),
        vga_buffer: io::SDRam::new(),
        vga_dma: io::VgaDma::new(),
    };

    let mut cpu = cpu::Cpu::new_with_bus(bus);
    let bin: Vec<u32> = vec![
        0x09000293, // li t0, 144
        0x04000337, // lui t1, 0x4000
        0x05030313, // add t1, t1, 80 # 4000050 <end+0x4000040>
        0x00532023, // sw t0, 0(t1)
        // 00000010 <end>:
        0x0000006f, // j 10 <end>
    ];
    for (i, instr) in bin.iter().enumerate() {
        cpu.bus.store_word(i as u32 * 4, *instr).unwrap();
    }
    // Roughly the amount of cycles needed to calculate 8 factorial with the above program
    for _ in 0..10 {
        cpu.clock();
    }

    assert_eq!(cpu.bus.hex_display.get(0), 144);
}

#[test]
fn test_switch_display() {
    // Program that stores the bit mask for the number 9 in the hex display

    let bus = cpu::Bus {
        sdram: io::SDRam::new(),
        button: io::Button::new(),
        hex_display: io::HexDisplay::new(),
        led_strip: io::LEDStrip::new(),
        switch: io::Switch::new(),
        timer: io::Timer::new(),
        uart: io::Uart::new(),
        vga_buffer: io::SDRam::new(),
        vga_dma: io::VgaDma::new(),
    };

    let mut cpu = cpu::Cpu::new_with_bus(bus);
    let bin: Vec<u32> = vec![
        0x04000337, // lui t1, 0x4000
        0x01030313, // add t1, t1,16 # 4000010 <end+0x4000004>
        0x00032283, // lw t0, 0(t1)
        // 0000000c <end>:
        0x0000006f, // j c <end>
    ];
    for (i, instr) in bin.iter().enumerate() {
        cpu.bus.store_word(i as u32 * 4, *instr).unwrap();
    }

    {
        cpu.bus.switch.set(0, true);
        cpu.bus.switch.set(2, true);
    }
    // Roughly the amount of cycles needed to calculate 8 factorial with the above program
    for _ in 0..10 {
        cpu.clock();
    }
    assert_eq!(cpu.regs.get(5), 5);
}

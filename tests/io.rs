use std::{cell::RefCell, rc::Rc};

/// Test larger programs using the emulator to ensure the CPU is working correctly
use dtekv_emulator_core::*;
use peripheral::MemoryMapped;

#[test]
fn test_hex_display() {
    // Program that stores the bit mask for the number 9 in the hex display

    let mut bus = peripheral::Bus::new();
    let hex_display = Rc::new(RefCell::new(peripheral::HexDisplay::new()));
    let sdram = Rc::new(RefCell::new(peripheral::SDRam::new()));
    bus.attach_device(
        (peripheral::HEX_DISPLAY_LOWER_ADDR, peripheral::HEX_DISPLAY_HIGHER_ADDR),
        Box::new(hex_display.clone()),
    );
    bus.attach_device(
        (peripheral::SDRAM_LOWER_ADDR, peripheral::SDRAM_HIGHER_ADDR),
        Box::new(sdram.clone()),
    );

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

    let hex_display = hex_display.borrow();
    assert_eq!(hex_display.get(0), 144);
}

#[test]
fn test_switch_display() {
    // Program that stores the bit mask for the number 9 in the hex display

    let mut bus = peripheral::Bus::new();
    let switch = Rc::new(RefCell::new(peripheral::Switch::new()));
    let sdram = Rc::new(RefCell::new(peripheral::SDRam::new()));
    bus.attach_device(
        (peripheral::SWITCH_LOWER_ADDR, peripheral::SWITCH_HIGHER_ADDR),
        Box::new(switch.clone()),
    );
    bus.attach_device(
        (peripheral::SDRAM_LOWER_ADDR, peripheral::SDRAM_HIGHER_ADDR),
        Box::new(sdram.clone()),
    );

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
        let mut switch = switch.borrow_mut();
        switch.set(0, true);
        switch.set(2, true);
    }
    // Roughly the amount of cycles needed to calculate 8 factorial with the above program
    for _ in 0..10 {
        cpu.clock();
    }
    assert_eq!(cpu.regs.get(register::Register::T0), 5);
}

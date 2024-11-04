/// Test larger programs using the emulator to ensure the CPU is working correctly
use dtekv_emulator::*;

#[test]
fn test_hex_display() {
    // Program that stores the bit mask for the number 9 in the hex display

    let mut cpu = Cpu::new();
    let bin: Vec<u32> = vec![
        0x09000293, // li t0, 144
        0x04000337, // lui t1, 0x4000
        0x05030313, // add t1, t1, 80 # 4000050 <end+0x4000040>
        0x00532023, // sw t0, 0(t1)
        // 00000010 <end>:
        0x0000006f, // j 10 <end>
    ];
    for (i, instr) in bin.iter().enumerate() {
        cpu.bus.store_word(i as u32 * 4, *instr);
    }
    // Roughly the amount of cycles needed to calculate 8 factorial with the above program
    for _ in 0..10 {
        cpu.clock();
    }

    assert_eq!(cpu.bus.hex_display.get_display(0), 144);
}

#[test]
fn test_switch_display() {
    // Program that stores the bit mask for the number 9 in the hex display

    let mut cpu = Cpu::new();
    let bin: Vec<u32> = vec![
        0x04000337, // lui t1, 0x4000
        0x01030313, // add t1, t1,16 # 4000010 <end+0x4000004>
        0x00032283, // lw t0, 0(t1)
        // 0000000c <end>:
        0x0000006f, // j c <end>
    ];
    for (i, instr) in bin.iter().enumerate() {
        cpu.bus.store_word(i as u32 * 4, *instr);
    }

    cpu.bus.switch.set_switch(0, true);
    cpu.bus.switch.set_switch(2, true);
    // Roughly the amount of cycles needed to calculate 8 factorial with the above program
    for _ in 0..10 {
        cpu.clock();
    }
    assert_eq!(cpu.regs.get(5), 5);
}

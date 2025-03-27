use std::{cell::RefCell, rc::Rc};

use debug_console::DebugConsole;
/// Test larger programs using the emulator to ensure the CPU is working correctly
use dtekv_emulator_core::*;
use memory_mapped::MemoryMapped;

fn new_cpu() -> cpu::Cpu<peripheral::Bus> {
    let mut bus = peripheral::Bus::new();

    bus.attach_device(
        (peripheral::SDRAM_LOWER_ADDR, peripheral::SDRAM_HIGHER_ADDR),
        Box::new(peripheral::SDRam::new()),
    );

    cpu::Cpu::new_with_bus(bus)
}

#[test]
fn test_factorial_riscv_program() {
    // Factorial program using only ADDI, ADD, and BEQ, result stored in x7

    let mut cpu = new_cpu();
    let bin: Vec<u8> = vec![
        // 00000000 <factorial_loop-0x8>:
        0x00800293, // li t0,8
        0x00100393, // li t2,1
        // 00000008 <factorial_loop>:
        0x02028263, // beqz t0,2c <end>
        0xfff28293, // add t0,t0,-1
        0x00028313, // mv t1,t0
        0x00038213, // mv tp,t2
        // 00000018 <mul_loop>:
        0x00030863, // beqz t1,28 <mul_end>
        0x004383b3, // add t2,t2,tp
        0xfff30313, // add t1,t1,-1
        0xfe000ae3, // beqz zero,18 <mul_loop>
        // 00000028 <mul_end>:
        0xfe0000e3, // beqz zero,8 <factorial_loop>
        // 0000002c <end>:
        0x00000063, // beqz zero,2c <end>
    ]
    .into_iter()
    .map(|x: u32| u32::to_le_bytes(x))
    .flatten()
    .collect();

    cpu.store_at(0, bin).unwrap();
    // Roughly the amount of cycles needed to calculate 8 factorial with the above program
    for _ in 0..200 {
        cpu.clock();
    }
    assert_eq!(cpu.regs.get(register::Register::T2), 40320);
}

#[test]
fn test_sieves_c_program() {
    let debug_console = Rc::new(RefCell::new(DebugConsole::new()));
    let mut cpu = new_cpu().with_debug_console(debug_console.clone());
    let bin = *include_bytes!("../roms/sieves/O0/sieves.rom");
    cpu.store_at(0, bin).unwrap();
    cpu.generate_instruction_cache();

    for _ in 0..2800 {
        cpu.clock();
    }

    fn is_prime(n: u32) -> bool {
        if n < 2 {
            return false;
        }
        for i in 2..n {
            if n % i == 0 {
                return false;
            }
        }
        true
    }

    {
        let mut db = debug_console.borrow_mut();
        let is_empty = db.is_empty();
        assert!(is_empty, "{:?}", db.pop().unwrap());
    }
    for i in 0..100 {
        assert_eq!(
            cpu.bus.load_byte(0x10000 + i).unwrap(),
            if is_prime(i) { 1 } else { 0 }
        );
    }
}

#[test]
fn test_sieves_c_program_o3() {
    let mut cpu = new_cpu();
    let bin = *include_bytes!("../roms/sieves/O3/sieves.rom");
    cpu.store_at(0, bin).unwrap();
    cpu.generate_instruction_cache();

    for _ in 0..2000 {
        cpu.clock();
    }

    fn is_prime(n: u32) -> bool {
        if n < 2 {
            return false;
        }
        for i in 2..n {
            if n % i == 0 {
                return false;
            }
        }
        true
    }

    for i in 0..100 {
        assert_eq!(
            cpu.bus.load_byte(0x10000 + i).unwrap(),
            if is_prime(i) { 1 } else { 0 }
        );
    }
}

#[test]
fn writing_and_running() {
    // This program will write a program into memory, then run that memory

    let bin: Vec<u8> = vec![
        // 00000000 <_start>:
        0x00000013, // nop
        0x00000013, // nop
        0x002002b7, // lui	t0,0x200
        0x29328293, // add	t0,t0,659 # 200293 <__stack_size+0x100293>
        0x10000313, // li	t1,256
        0x00532023, // sw	t0,0(t1)
        0x040012b7, // lui	t0,0x4001
        0xe1328293, // add	t0,t0,-493 # 4000e13 <__stack_size+0x3f00e13>
        0x10400313, // li	t1,260
        0x00532023, // sw	t0,0(t1)
        0x000e02b7, // lui	t0,0xe0
        0x36728293, // add	t0,t0,871 # e0367 <__heap_size+0xdfb67>
        0x10800313, // li	t1,264
        0x00532023, // sw	t0,0(t1)
        0x10000313, // li	t1,256
        0x00030f67, // jalr	t5,t1
        // 00000040 <end>:
        0x0000006f, // j	40 <end>
    ]
    .into_iter()
    .map(|x| u32::to_le_bytes(x))
    .flatten()
    .collect();

    let mut cpu = new_cpu();
    cpu.reset();

    cpu.store_at(0, bin).unwrap();
    cpu.generate_instruction_cache();

    for _ in 0..1000 {
        cpu.clock();
    }

    assert_eq!(cpu.regs.get(register::Register::T0), 2);
}

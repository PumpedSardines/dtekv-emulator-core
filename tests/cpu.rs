/// Test larger programs using the emulator to ensure the CPU is working correctly
use dtekv_emulator_core::*;

#[test]
fn test_factorial_riscv_program() {
    // Factorial program using only ADDI, ADD, and BEQ, result stored in x7

    let mut cpu = cpu::Cpu::new_with_bus(io::SDRam::new());
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
    .map(|x: u32| [x as u8, (x >> 8) as u8, (x >> 16) as u8, (x >> 24) as u8])
    .flatten()
    .collect();
    cpu.store_at(0, bin).unwrap();
    // Roughly the amount of cycles needed to calculate 8 factorial with the above program
    for _ in 0..200 {
        cpu.clock();
    }
    assert_eq!(cpu.regs.get(7), 40320);
}

#[test]
fn test_sieves_c_program() {
    let mut cpu = cpu::Cpu::new_with_bus(io::SDRam::new());
    // Set stack pointer somewhere
    cpu.regs.set(2, 0x2000);
    // Testing the following C program is compiled to RISC-V assembly
    /*
    char *sieves = (char *)0x100;

    void prime_sieves() {
      int i, j;
      for (i = 2; i < 100; i++) {
        sieves[i] = 1;
      }
      for (i = 2; i < 100; i++) {
        if (sieves[i]) {
          for (j = i + i; j < 100; j += i) {
            sieves[j] = 0;
          }
        }
      }
    }

    inline __attribute__((always_inline))
    int main() {
      prime_sieves();
      return 0;
    }


    void _start(void) __attribute__((section("._start")));
    void _start() {
      main();
      while (1);
    }

    */

    let bin: Vec<u32> = vec![
        // 00000000 <_start>:
        0xff010113, // add	x2,x2,-16
        0x00112623, // sw	x1,12(x2)
        0x00812423, // sw	x8,8(x2)
        0x01010413, // add	x8,x2,16
        0x008000ef, // jal	18 <prime_sieves>
        0x0000006f, // j	14 <_start+0x14>
        // 00000018 <prime_sieves>:
        0xfe010113, // add	x2,x2,-32
        0x00812e23, // sw	x8,28(x2)
        0x02010413, // add	x8,x2,32
        0x00200793, // li	x15,2
        0xfef42623, // sw	x15,-20(x8)
        0x0240006f, // j	50 <prime_sieves+0x38>
        0x0e402703, // lw	x14,228(x0) # e4 <sieves>
        0xfec42783, // lw	x15,-20(x8)
        0x00f707b3, // add	x15,x14,x15
        0x00100713, // li	x14,1
        0x00e78023, // sb	x14,0(x15)
        0xfec42783, // lw	x15,-20(x8)
        0x00178793, // add	x15,x15,1
        0xfef42623, // sw	x15,-20(x8)
        0xfec42703, // lw	x14,-20(x8)
        0x06300793, // li	x15,99
        0xfce7dce3, // bge	x15,x14,30 <prime_sieves+0x18>
        0x00200793, // li	x15,2
        0xfef42623, // sw	x15,-20(x8)
        0x0600006f, // j	c4 <prime_sieves+0xac>
        0x0e402703, // lw	x14,228(x0) # e4 <sieves>
        0xfec42783, // lw	x15,-20(x8)
        0x00f707b3, // add	x15,x14,x15
        0x0007c783, // lbu	x15,0(x15)
        0x04078063, // beqz	x15,b8 <prime_sieves+0xa0>
        0xfec42783, // lw	x15,-20(x8)
        0x00179793, // sll	x15,x15,0x1
        0xfef42423, // sw	x15,-24(x8)
        0x0240006f, // j	ac <prime_sieves+0x94>
        0x0e402703, // lw	x14,228(x0) # e4 <sieves>
        0xfe842783, // lw	x15,-24(x8)
        0x00f707b3, // add	x15,x14,x15
        0x00078023, // sb	x0,0(x15)
        0xfe842703, // lw	x14,-24(x8)
        0xfec42783, // lw	x15,-20(x8)
        0x00f707b3, // add	x15,x14,x15
        0xfef42423, // sw	x15,-24(x8)
        0xfe842703, // lw	x14,-24(x8)
        0x06300793, // li	x15,99
        0xfce7dce3, // bge	x15,x14,8c <prime_sieves+0x74>
        0xfec42783, // lw	x15,-20(x8)
        0x00178793, // add	x15,x15,1
        0xfef42623, // sw	x15,-20(x8)
        0xfec42703, // lw	x14,-20(x8)
        0x06300793, // li	x15,99
        0xf8e7dee3, // bge	x15,x14,68 <prime_sieves+0x50>
        0x00000013, // nop
        0x00000013, // nop
        0x01c12403, // lw	x8,28(x2)
        0x02010113, // add	x2,x2,32
        0x00008067, // ret
        // 000000e4 <sieves>:
        0x0100,
    ];

    for (i, instr) in bin.iter().enumerate() {
        cpu.store_word(i as u32 * 4, *instr).unwrap();
    }
    cpu.generate_instruction_cache();
    // The amount of cycles needed to calculate 8 factorial with the above program with some
    // extra cycles to ensure the program has finished
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

    for i in 0..100 {
        assert_eq!(
            cpu.bus.load_byte(0x100 + i).unwrap(),
            if is_prime(i) { 1 } else { 0 }
        );
    }
}

#[test]
fn test_sieves_c_program_o3() {
    let mut cpu = cpu::Cpu::new_with_bus(io::SDRam::new());
    // Set stack pointer somewhere
    cpu.regs.set(2, 0x2000);
    // Testing the following C program is compiled to RISC-V assembly with the 03 optimization flag
    /*
    char *sieves = (char *)0x100;

    void prime_sieves() {
      int i, j;
      for (i = 2; i < 100; i++) {
        sieves[i] = 1;
      }
      for (i = 2; i < 100; i++) {
        if (sieves[i]) {
          for (j = i + i; j < 100; j += i) {
            sieves[j] = 0;
          }
        }
      }
    }

    inline __attribute__((always_inline))
    int main() {
      prime_sieves();
      return 0;
    }


    void _start(void) __attribute__((section("._start")));
    void _start() {
      main();
      while (1);
    }

    */

    let bin: Vec<u32> = vec![
        // 00000000 <_start>:
        0xff010113, // add	x2,x2,-16
        0x00112623, // sw	x1,12(x2)
        0x008000ef, // jal	10 <prime_sieves>
        0x0000006f, // j	c <_start+0xc>
        // 00000010 <prime_sieves>:
        0x00200793, // li	x15,2
        0x00100593, // li	x11,1
        0x06400693, // li	x13,100
        0x0a402703, // lw	x14,164(x0) # a4 <sieves>
        0x00f70733, // add	x14,x14,x15
        0x00b70023, // sb	x11,0(x14)
        0x00178793, // add	x15,x15,1
        0xfed798e3, // bne	x15,x13,1c <prime_sieves+0xc>
        0x00600513, // li	x10,6
        0x00400593, // li	x11,4
        0x00200693, // li	x13,2
        0x06300813, // li	x16,99
        0x06400893, // li	x17,100
        0x0140006f, // j	58 <prime_sieves+0x48>
        0x00168693, // add	x13,x13,1
        0x00258593, // add	x11,x11,2
        0x00350513, // add	x10,x10,3
        0x05168663, // beq	x13,x17,a0 <prime_sieves+0x90>
        0x0a402783, // lw	x15,164(x0) # a4 <sieves>
        0x00d78733, // add	x14,x15,x13
        0x00074703, // lbu	x14,0(x14)
        0xfe0702e3, // beqz	x14,48 <prime_sieves+0x38>
        0xfeb860e3, // bltu	x16,x11,48 <prime_sieves+0x38>
        0x00b787b3, // add	x15,x15,x11
        0x00078023, // sb	x0,0(x15)
        0x00050793, // mv	x15,x10
        0xfca868e3, // bltu	x16,x10,48 <prime_sieves+0x38>
        0x0a402703, // lw	x14,164(x0) # a4 <sieves>
        0x00f70733, // add	x14,x14,x15
        0x00070023, // sb	x0,0(x14)
        0x00d787b3, // add	x15,x15,x13
        0xfef858e3, // bge	x16,x15,7c <prime_sieves+0x6c>
        0x00168693, // add	x13,x13,1
        0x00258593, // add	x11,x11,2
        0x00350513, // add	x10,x10,3
        0xfb169ee3, // bne	x13,x17,58 <prime_sieves+0x48>
        0x00008067, // ret
        // 000000a4 <sieves>:
        0x0100,
    ];

    for (i, instr) in bin.iter().enumerate() {
        cpu.store_word(i as u32 * 4, *instr).unwrap();
    }
    cpu.generate_instruction_cache();
    // The amount of cycles needed to calculate 8 factorial with the above program with some
    // extra cycles to ensure the program has finished
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
            cpu.bus.load_byte(0x100 + i).unwrap(),
            if is_prime(i) { 1 } else { 0 }
        );
    }
}

#[test]
fn writing_and_running() {
    // This program will write a program into memory, then run that memory

    let bin: Vec<u32> = vec![
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
    ];

    let mut cpu = cpu::Cpu::new_with_bus(io::SDRam::new());
    cpu.reset();

    for (i, instr) in bin.iter().enumerate() {
        cpu.bus.store_word(i as u32 * 4, *instr).unwrap();
    }

    cpu.generate_instruction_cache();

    // The amount of cycles needed to calculate 8 factorial with the above program with some
    // extra cycles to ensure the program has finished
    for _ in 0..50 {
        println!("{:?}", cpu.pc);
        cpu.clock();
    }

    assert_eq!(cpu.regs.get(5), 2);
}

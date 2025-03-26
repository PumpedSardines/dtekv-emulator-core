use criterion::{black_box, criterion_group, criterion_main, Criterion};
use dtekv_emulator_core::*;
use peripheral::MemoryMapped;

fn new_cpu() -> cpu::Cpu<peripheral::Bus> {
    let mut bus = peripheral::Bus::new();

    bus.attach_device(
        (peripheral::SDRAM_LOWER_ADDR, peripheral::SDRAM_HIGHER_ADDR),
        Box::new(peripheral::SDRam::new()),
    );

    cpu::Cpu::new_with_bus(bus)
}

fn create_sieves_cpu() -> cpu::Cpu<peripheral::Bus> {
    let mut cpu = new_cpu();
    // Set stack pointer somewhere
    cpu.regs.set(register::Register::SP, 0x2000);
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

    let bin: Vec<u8> = vec![
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
    ]
    .into_iter()
    .map(|x| u32::to_le_bytes(x))
    .flatten()
    .collect();

    cpu.store_at(0, bin).unwrap();

    cpu
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("sieves", |b| {
        let mut cpu = create_sieves_cpu();
        b.iter(|| {
            black_box({
                for _ in 0..2800 {
                    cpu.clock();
                }
            })
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

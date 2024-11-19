# Quirks:

There are some quirks that the dtekv board has that the standard risc-v implementation doesn't.

- When an external interrupt happens, the program counter is still incremented before being stored into MEPC
- When an internal interrupt happens, the program counter is **NOT** incremented before being stored into MEPC
- Accessing memory out of bounds is very inconsistent and returns different things depending on seemingly random stuff. It looks like the chip is trying to respond with 0xDEADBEEF, but this is not always the case, especially when accessing halfwords and bytes. I've decided that accessing memory out of bounds is undefined behavior and return OxDE when accessing a byte, 0xDEAD when accessing a halfword and 0xDEADBEEF when accessing a word. Storing does nothing and just ignores the error.
- The VGA DMA works by swapping back and front buffer. The value is clamped, min address is 0x08000000. however, when specifying something larger we just fill the vga buffer with 0
- Interrupts ALWAYS move to address 0
- Program counter ALWAYS starts at 4
- CSRWI takes a bit to set instead of a bit mask
- ECALL always calls from machine mode
- Only three exceptions exist:
    - 0: Instruction address misaligned
    - 2: Illegal instruction
    - 11: Ecall machine mode

# Quirks:

There are some quirks that the dtekv board has that the standard risc-v implementation doesn't.

- When an external interrupt happens, the program counter is still incremented before being stored into MEPC
- When an internal interrupt happens, the program counter is **NOT** incremented before being stored into MEPC
- Interrupts ALWAYS move to address 0
- Program counter ALWAYS starts at 4
- CSRWI takes a bit to set instead of a bit mask
- ECALL always calls from machine mode
- Only three exceptions exist:
    - 0: Instruction address misaligned
    - 2: Illegal instruction
    - 11: Ecall machine mode

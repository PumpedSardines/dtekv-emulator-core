# DTEKV emulator

## Quirks:

There are some quirkes that the dtekv board has that the standard risc-v implementation doesn't.

- When an external interrupt happens, the program counter is still incremented before being stored into MEPC
- CSRWI takes a bit to set instead of a bit mask
- ECALL always calls from machine mode
- Only three exceptions exist:
    - 0: Instruction address misaligned
    - 2: Illegal instruction
    - 11: Ecall machine mode

## Supported Risc-V instructions:

- [X] LUI
- [X] AUIPC
- [X] JAL
- [X] JALR
- [X] BEQ
- [X] BNE
- [X] BLT
- [X] BGE
- [X] BLTU
- [X] BGEU
- [X] LB
- [X] LH
- [X] LW
- [X] LBU
- [X] LHU
- [X] SB
- [X] SH
- [X] SW
- [X] ADDI
- [X] SLTI
- [X] SLTIU
- [X] XORI
- [X] ORI
- [X] ANDI
- [X] SLLI
- [X] SRLI
- [X] SRAI
- [X] ADD
- [X] SUB
- [X] SLL
- [X] SLT
- [X] SLTU
- [X] XOR
- [X] SRL
- [X] SRA
- [X] OR
- [X] AND
- [X] CSRRW
- [X] CSRRS
- [X] CSRRC
- [ ] CSRRWI
- [X] CSRRSI
- [ ] CSRRCI
- [X] MRET
- [X] ECALL
- [X] MUL
- [X] MULH
- [X] MULHU
- [X] MULHSU
- [-] DIV
- [-] DIVU
- [-] REM
- [-] REMU

## Supported CSR

- mstatus
- mcause
- mepc
- mie
- mpie

## Supported Modes

Only machine mode is supported


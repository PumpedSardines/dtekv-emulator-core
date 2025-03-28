<div align="center">
    <h1>DTEK-V emulator core</h1>
    <p>Containing the library for emulating the chip students at KTH taking the IS1500 course use</p>
</div>

## Implemented IO devices:

- Button
- Switch
- Timer
- VGA output
- Hex displays
- LED
- Button interrupts
- Switch interrupts
- Timer interrupts

## Supported Risc-V instructions:

- [x] LUI
- [x] AUIPC
- [x] JAL
- [x] JALR
- [x] BEQ
- [x] BNE
- [x] BLT
- [x] BGE
- [x] BLTU
- [x] BGEU
- [x] LB
- [x] LH
- [x] LW
- [x] LBU
- [x] LHU
- [x] SB
- [x] SH
- [x] SW
- [x] ADDI
- [x] SLTI
- [x] SLTIU
- [x] XORI
- [x] ORI
- [x] ANDI
- [x] SLLI
- [x] SRLI
- [x] SRAI
- [x] ADD
- [x] SUB
- [x] SLL
- [x] SLT
- [x] SLTU
- [x] XOR
- [x] SRL
- [x] SRA
- [x] OR
- [x] AND
- [x] CSRRW
- [x] CSRRS
- [x] CSRRC
- [ ] CSRRWI
- [x] CSRRSI
- [ ] CSRRCI
- [x] MRET
- [x] ECALL
- [x] MUL
- [x] MULH
- [x] MULHU
- [x] MULHSU
- [X] DIV
- [X] DIVU
- [X] REM
- [X] REMU

## Supported Modes

Only machine mode is supported

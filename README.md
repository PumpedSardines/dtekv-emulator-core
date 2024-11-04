<div align="center">
    <h1>DTEK-V emulator</h1>
    <p>An emulator for the chip that the KTH students have to use in IS1500</p>
</div>

![A screenshot of the game](./assets/screenshot.png)

<div align="center">
    <p><i>A preview of my final project running in the emulator</i></p>
</div>

## How to use:

This project can take the binary you upload to the dtek-v board and run it in an emulated environment, making it easier to develop your programs without having to upload to the chip every time you want to debug.

Currently you have to change the code to run your binary. Im working on a way to run any code, if you want to add this submit a PR :) 

## Supported IO devices:

- Button
- Switch
- VGA output
- Hex displays
- Button interrupts
- Switch interrupts

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


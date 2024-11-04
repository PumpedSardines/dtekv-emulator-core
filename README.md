<div align="center">
    <h1>DTEK-V emulator</h1>
    <p>An emulator for the chip that the KTH students taking the IS1500 course have to use</p>
</div>

![A screenshot of the emulator](./assets/example.png)

<div align="center">
    <p><i>A preview of my project running in the emulator</i></p>
</div>
<br />
<br />

## How to use:

This project can take the binary you upload to the dtek-v board and run it in an emulated environment, making it easier to develop your programs without having to upload to the chip every time you want to debug.

Currently the code compile the file "dtekv-riscv.bin" into the binary when compiling. Replace your binary with that file and run `cargo run` in the terminal.
In the future i want to take an argument in instead of compiling the binary.

## Supported IO devices:

- Button
- Switch
- VGA output
- Hex displays
- Button interrupts
- Switch interrupts

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

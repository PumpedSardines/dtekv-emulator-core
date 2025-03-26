.section .text
.align 2
.globl _start
_start:
  nop // Program loads at 0x4, 0x0 is IR address
  la sp, _stack_end
  la gp, __global_pointer
  call prime_sieves
loop: j loop

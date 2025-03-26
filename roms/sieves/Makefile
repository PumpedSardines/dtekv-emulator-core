SRC_DIR ?= ./
OBJ_DIR ?= ./
SOURCES ?= $(shell find $(SRC_DIR) -name '*.c' -or -name '*.S')
OBJECTS ?= $(addsuffix .o, $(basename $(notdir $(SOURCES))))
LINKER ?= $(SRC_DIR)/linker.ld

TOOLCHAIN ?= riscv64-unknown-elf-
CFLAGS ?= -Wall -nostdlib -mabi=ilp32 -march=rv32imzicsr

build: clean O0/sieves.rom O3/sieves.rom clean

main-O0.elf: 
	$(TOOLCHAIN)gcc -c $(CFLAGS) $(SOURCES)
	$(TOOLCHAIN)ld -o $@ -T $(LINKER) $(filter-out boot.o, $(OBJECTS))

main-O3.elf: 
	$(TOOLCHAIN)gcc -c $(CFLAGS) -O3 $(SOURCES)
	$(TOOLCHAIN)ld -o $@ -T $(LINKER) $(filter-out boot.o, $(OBJECTS))

O0/sieves.rom: main-O0.elf
	mkdir -p O0
	$(TOOLCHAIN)objcopy --output-target binary $< $@
	$(TOOLCHAIN)objdump -D $< > O0/sieves.asm.txt

O3/sieves.rom: main-O3.elf
	mkdir -p O3
	$(TOOLCHAIN)objcopy --output-target binary $< $@
	$(TOOLCHAIN)objdump -D $< > O3/sieves.asm.txt

clean:
	rm -f *.o *.elf *.bin *.txt

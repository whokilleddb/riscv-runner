ASM_CC=riscv64-unknown-elf-gcc
OBJCPY=riscv64-unknown-elf-objcopy
ASM_CFLAGS=-Wl,-Ttext=0x0 -O0 -nostdlib -fverbose-asm
ASM_SRC_DIR=asm
ASM_BUILD_DIR=target/asm
ASM_SRC_FILE=addi_add_mul_sub.s

TARGET=riscv

default: rusty asm;

rusty: src/*.rs 
	cargo build
	cargo build --release

asm:  $(ASM_SRC_DIR)/$(ASM_SRC_FILE)
	@mkdir -p $(ASM_BUILD_DIR)
	$(ASM_CC) $(ASM_CFLAGS) -o $(ASM_BUILD_DIR)/$(TARGET) $(ASM_SRC_DIR)/$(ASM_SRC_FILE)
	$(OBJCPY) -O binary $(ASM_BUILD_DIR)/$(TARGET) $(ASM_BUILD_DIR)/$(TARGET).bin

.PHONY: clean default asm

clean:
	cargo clean
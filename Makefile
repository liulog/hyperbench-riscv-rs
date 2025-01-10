TARGET		    := riscv64gc-unknown-none-elf
MODE		    := debug
HYPERBENCH_ELF	:= target/$(TARGET)/$(MODE)/hyperbench-riscv-rs
HYPERBENCH_BIN	:= target/$(TARGET)/$(MODE)/hyperbench-riscv-rs.bin
CPUS		    := 1

OBJDUMP         := rust-objdump --arch-name=riscv64
OBJCOPY         := rust-objcopy --binary-architecture=riscv64

BOOTLOADER      := bootloader/rustsbi-qemu.bin

# QEMUPATH	    ?= ~/software/qemu/qemu-7.1.0/build/
# QEMU 		    := $(QEMUPATH)qemu-system-riscv64
QEMU 		    := qemu-system-riscv64
QEMUOPTS        := --machine virt -m 3G -bios $(BOOTLOADER) -nographic -kernel $(HYPERBENCH_BIN)


build:
	cargo build

$(HYPERBENCH_BIN): build
	$(OBJCOPY) $(HYPERBENCH_ELF) --strip-all -O binary $@


qemu: $(HYPERBENCH_BIN)
	$(QEMU) $(QEMUOPTS)

clean:
	rm $(HYPERBENCH_BIN)
	rm $(HYPERBENCH_ELF)



Z_FLAGS=-Z build-std=core,compiler_builtins \
         -Z build-std-features=compiler-builtins-mem

TARGET=x86_64-HLeos.json

KERNEL=bootimage-HLeOs.bin
KERNEL_DIR=target/x86_64-HLeos/debug/$(KERNEL)

.PHONY : all, run

all : $(TARGET)
	cargo bootimage $(Z_FLAGS) --target $(TARGET)

run : $(KERNEL_DIR)
	qemu-system-x86_64 -drive format=raw,file=$(KERNEL_DIR) -nographic

gdb : $(KERNEL_DIR)
	qemu-system-x86_64 -drive format=raw,file=$(KERNEL_DIR) -nographic -s -S

Z_FLAGS=-Z build-std=core,compiler_builtins \
         -Z build-std-features=compiler-builtins-mem

TARGET=x86_64-HLeos.json

WORKING_DIR=/home/asss8593/lab/HLeOs/HLeOs

KERNEL=bootimage-HLeOs.bin

KERNEL_DIR=$(WORKING_DIR)/target/x86_64-HLeos/debug/$(KERNEL)

.PHONY : all, run, gdb, clean

all : $(TARGET)
	cargo bootimage $(Z_FLAGS) --target $(TARGET)

run : $(KERNEL_DIR)
	sudo xauth add `xauth -f /home/asss8593/.Xauthority list|tail -1`
	sudo qemu-system-x86_64 -drive format=raw,file=$(KERNEL_DIR) -enable-kvm

gdb : $(KERNEL_DIR)
	sudo xauth add `xauth -f /home/asss8593/.Xauthority list|tail -1`
	sudo qemu-system-x86_64 -drive format=raw,file=$(KERNEL_DIR) -enable-kvm -s -S

clean :
	@rm -rf ./target

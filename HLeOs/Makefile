Z_FLAGS=-Z build-std=core,compiler_builtins \
         -Z build-std-features=compiler-builtins-mem

TARGET=x86_64-HLeos.json

WORKING_DIR=/home/asss8593/lab/HLeOs/HLeOs

IMAGE=bootimage-HLeOs.bin
KERNEL=HLeOs 

IMAGE_DIR=$(WORKING_DIR)/target/x86_64-HLeos/debug/$(IMAGE)
KERNEL_DIR=$(WORKING_DIR)/target/x86_64-HLeos/debug/$(KERNEL)

GDB_SCRIPT=./gdb_init_real_mode.txt
BOOTLOADER_DIR=$(WORKING_DIR)/target/bootimage/bootloader/x86_64-bootloader/release/bootloader

.PHONY : all, run, gdb, gdb-i386, clean debug-loader debug

all : $(TARGET)
	cargo bootimage $(Z_FLAGS) --target $(TARGET)

run : $(IMAGE_DIR)
	sudo xauth add `xauth -f /home/asss8593/.Xauthority list|tail -1`
	sudo qemu-system-x86_64 -drive format=raw,file=$(IMAGE_DIR) -enable-kvm

gdb : $(IMAGE_DIR)
	sudo xauth add `xauth -f /home/asss8593/.Xauthority list|tail -1`
	sudo qemu-system-x86_64 -drive format=raw,file=$(IMAGE_DIR) -enable-kvm -s -S

gdb-i386 : $(IMAGE_DIR)
	sudo xauth add `xauth -f /home/asss8593/.Xauthority list|tail -1`
	sudo qemu-system-i386 -drive format=raw,file=$(IMAGE_DIR) -enable-kvm -s -S

debug-loader : $(GDB_SCRIPT)
	gdb -ix $(GDB_SCRIPT) \
		-ex 'symbol $(BOOTLOADER_DIR)' \
		-ex 'set tdesc filename target.xml' \
		-ex 'target remote localhost:1234' \
		-ex 'hb *0x7c00' \
		-ex 'continue'
debug :
	gdb -ex 'file $(KERNEL_DIR)' \
		-ex 'target remote:1234' \
		-ex 'hb _start' \
		-ex 'continue'

clean :
	@rm -rf ./target

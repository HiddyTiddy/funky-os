CURR_ID := $(shell id -u)

RELEASE = 0

.PHONY: all clean
all: bootimage
	#bootsector kernel
	# cat bin/bs.bin target/x86_64-funky_os/release/os > bin/os.bin
	# qemu-system-x86_64 bin/os.bin
ifeq ($(RELEASE), 1)
	qemu-system-x86_64 target/x86_64-funky_os/release/bootimage-os.bin
else
	qemu-system-x86_64 target/x86_64-funky_os/debug/bootimage-os.bin
endif


sandwich:
ifeq ($(CURR_ID), 0)
	nasm -f bin bootsector/bootsector.asm -o bs.bin
	qemu-system-x86_64 bin/bs.bin
else
	$(error no)
endif

bootsector: force
	nasm -f bin bootsector/bootsector.asm -o bin/bs.bin

kernel: kernel_entry
	cargo build --release

kernel_entry:
	nasm kernel_entry.asm -f elf64 -o bin/kernel_entry.o

bootimage:
ifeq ($(RELEASE), 1)
	cargo bootimage --release
else
	cargo bootimage
endif

force:

CURR_ID := $(shell id -u)

.PHONY: all clean
all: bootsector
	qemu-system-x86_64 bin/bs.bin


sandwich:
ifeq ($(CURR_ID), 0)
	nasm -f bin bootsector/bootsector.asm -o bs.bin
	qemu-system-x86_64 bin/bs.bin
else
	$(error no)
endif

bootsector: force
	nasm -f bin bootsector/bootsector.asm -o bin/bs.bin

force:

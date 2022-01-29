CURR_ID := $(shell id -u)

sandwich:
ifeq ($(CURR_ID), 0)
	nasm -f bin bootsector/bootsector.asm -o bs.bin
	qemu-system-x86_64 bs.bin
else
	$(error no)
endif

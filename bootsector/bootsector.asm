[org 0x7c00] ; always add offset of 0x7c00 to references

mov ah, 0x0e ; tty mode

mov bx, buongiorno
call print

jmp $

%include "bootsector/print.asm"

buongiorno:
    db "buongiorno", 0

times 510 - ($-$$) db 0
dw 0xaa55


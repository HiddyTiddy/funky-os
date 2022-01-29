mov ah, 0x0e ; tty mode

mov al, "1"
int 0x10
mov bx, buongiorno
add bx, 0x7c00
mov al, [bx]
int 0x10

jmp $
buongiorno:
    db "buongiorno"

times 510 - ($-$$) db 0
dw 0xaa55


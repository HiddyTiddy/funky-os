print_hex: ; data in dx
    pusha
    mov cx, 0

print_hex_start:
    cmp cx, 4
    je print_hex_done

    mov ax, dx
    and ax, 0xf
    add al, 0x30 ; al + '0'
    cmp al, 0x39
    jle print_hex_p2
    add al, 0x27 ; 'a' - 'f'
    ;add al, 0x7 ; 'A' - 'F'

print_hex_p2:
    mov bx, print_hex_out + 5
    sub bx, cx
    mov [bx], al
    ror dx, 4 ; interesting. why does shl not work

    add cx, 1
    jmp print_hex_start

print_hex_done:
    mov bx, print_hex_out
    call print
    popa
    ret

print_hex_out:
    db "0x0000", 0 ; smart

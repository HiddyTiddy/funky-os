[bits 32]
print_hex32: ; data in edx
    pusha
    mov ecx, 0

print_hex32_start:
    cmp ecx, 8
    je print_hex32_done

    mov eax, edx ; 30
    and eax, 0xf
    add eax, 0x30 ; al + '0'
    cmp eax, 0x39
    jle print_hex32_p2
    add eax, 0x27 ; 'a' - 'f'
    ;add al, 0x7 ; 'A' - 'F'

print_hex32_p2:
    mov ebx, print_hex32_out + 5
    sub ebx, ecx
    mov [ebx], eax
    ror edx, 4 ; interesting. why does shl not work

    add ecx, 1
    jmp print_hex32_start

print_hex32_done:
    mov ebx, print_hex32_out
    ;call print32
    popa
    ret

print_hex32_out:
    db "0x00000000", 0 ; smart

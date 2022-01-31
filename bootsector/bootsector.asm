[org 0x7c00] ; always add offset of 0x7c00 to references
KERNEL_OFFSET equ 0x1000
; mov ah, 0x0e ; tty mode
mov [BOOT_DRIVE], dl; 0x0080
call print_hex
mov bp, 0x9000; stack pointer mov sp,bp


mov bx, buongiorno
call print
call println

call load_kernel
call switch_32

jmp $ ; unreachable!()

%include "bootsector/print.asm"
%include "bootsector/print_hex.asm"
%include "bootsector/disk_load.asm"
%include "bootsector/switch_32.asm"
%include "bootsector/print32.asm"
%include "bootsector/gdt.asm"

[bits 16]
load_kernel:
    mov bx, MSG_LOAD_KERNEL
    call print
    call println

    mov bx, KERNEL_OFFSET
    mov dh, 2
    mov dl, [BOOT_DRIVE]
    call disk_load

    mov dx, [KERNEL_OFFSET]
    call print_hex
    call println
    mov dx, [KERNEL_OFFSET + 8]
    call print_hex
    call println

    ret


[bits 32]
BEGIN_PM:
    mov ebx, MSG_PROT_MODE
    call print32
    mov ebx, KERNEL_OFFSET
    call print32
    call KERNEL_OFFSET
    jmp $

buongiorno:
    db "buongiorno", 0xa, 0

BOOT_DRIVE db 0
MSG_LOAD_KERNEL db "loading kernel into memory", 0
MSG_PROT_MODE db "Landed in 32-bit Protected Mode", 0


times 510 - ($-$$) db 0
dw 0xaa55


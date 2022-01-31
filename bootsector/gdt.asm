gdt_start:
    dd 0x0
    dd 0x0

gdt_code:
    dw 0xffff
    dw 0x0
    db 0x0
    db 0b10011010
    db 0b11001111
    db 0x0


gdt_data:
    dw 0xffff
    dw 0x0
    db 0x0
    db 0b10010010
    db 0b11001111
    db 0x0

gdt_end:
gdt_descriptor:
    dw gdt_end - gdt_start - 1 ; size (16 bit), always one less of its true size
    dd gdt_start ; address (32 bit)

; define some constants for later use
CODE_SEG equ gdt_code - gdt_start
DATA_SEG equ gdt_data - gdt_start

disk_load: ; dh = num sectors
    pusha

    push dx
    mov ah, 0x02; 0x02 = read
    mov al, dh
    mov cl, 0x02; sector

    mov ch, 0x00; cylinder
    mov dh, 0x00; head number

    int 0x13
    jc disk_load_disk_error

    pop dx
    cmp al, dh
    jne disk_load_sectors_error
    popa
    ret


disk_load_disk_error:
    mov bx, DISK_ERROR
    call print

    ; call println

    mov dh, ah; ah = error code, dl = disk drive
    call print_hex
    jmp $

disk_load_sectors_error:
    mov bx, SECTORS_ERROR
    call print
disk_load_loop:
    jmp $


DISK_ERROR: db "Disk read error: ", 0
SECTORS_ERROR: db "Incorrect number of sectors read", 0

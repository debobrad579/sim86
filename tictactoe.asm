main:
	%define stdin 00h
	%define stdout 01h

    mov ax, 0x1000
    mov ss, ax
    mov sp, 0xFA00

    mov bx, 0x1000

    mov [bx+0], 1
    mov [bx+3], 1
    mov [bx+6], 1
    mov [bx+1], 2
    mov [bx+4], 2

    call print_board

    hlt

print_board:
    push ax
    push di

    print_loop_start:
        mov al, [bx+di]

        cmp al, 0
        je print_empty

        cmp al, 1
        je print_x

        cmp al, 2
        je print_o

        print_loop_mid:

        cmp di, 2
        je print_newline

        cmp di, 5
        je print_newline

        cmp di, 8
        je print_newline

        print_loop_end:

        inc di
        cmp di, 9
        jne print_loop_start

    pop di
    pop ax
    ret

print_empty:
    mov al, '-'
    out stdout, al
    jmp print_loop_mid

print_x:
    mov al, 'X'
    out stdout, al
    jmp print_loop_mid

print_o:
    mov al, 'O'
    out stdout, al
    jmp print_loop_mid

print_newline:
    mov al, 0x0A
    out stdout, al
    jmp print_loop_end

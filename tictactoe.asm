main:
    %define stdin 00h
    %define stdout 01h
    %define newline 0x0A
    %define backspace 0x08

    %define X 1
    %define O -1
    %define Empty 0

    mov ax, 0x1000
    mov ss, ax
    mov sp, 0xFA00

    mov bx, 0x1000

    game_loop:
        mov ax, 'Mo'
        out stdout, ax
        mov ax, 've'
        out stdout, ax
        mov ax, ' ('
        out stdout, ax
        mov ax, '1-'
        out stdout, ax
        mov ax, '9)'
        out stdout, ax
        mov ax, ': '
        out stdout, ax

        in al, stdin

        cmp al, 'q'
        je halt

        out stdout, al

        cbw
        mov di, ax
        sub di, '1'

        cmp di, 0
        jl delete_game_loop_text

        cmp di, 8
        jg delete_game_loop_text

        cmp [bx+di], 0
        jne delete_game_loop_text

        call get_turn

        mov [bx+di], dl

        mov al, newline
        out stdout, al

        call print_board

        jmp game_loop

    delete_game_loop_text:
        mov cx, 13
        delete_loop:
            mov al, backspace
            out stdout, al
            loop delete_loop
        jmp game_loop

halt:
    mov al, newline
    out stdout, al
    hlt

get_turn:
    push di
    push cx

    mov di, 0

    get_turn_loop:
        mov al, [bx+di]

        cmp al, 0
        jne inc_cl

        get_turn_loop_end:

        inc di
        cmp di, 9
        jne get_turn_loop

    test cl, 1
    je ret_X
    jne ret_O

    inc_cl:
        inc cl
        jmp get_turn_loop_end

    ret_X:
        mov dl, X

        pop cx
        pop di
        ret

    ret_O:
        mov dl, O

        pop cx
        pop di
        ret

print_board:
    push ax
    push di

    mov di, 0

    print_loop_start:
        mov al, [bx+di]

        cmp al, Empty
        je print_empty

        cmp al, X
        je print_X

        cmp al, O
        je print_O

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

    print_X:
        mov al, 'X'
        out stdout, al
        jmp print_loop_mid

    print_O:
        mov al, 'O'
        out stdout, al
        jmp print_loop_mid

    print_newline:
        mov al, newline
        out stdout, al
        jmp print_loop_end

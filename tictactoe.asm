main:
    %define stdin 00h
    %define stdout 01h
    %define newline 0x0A
    %define backspace 0x08

    %define X 2
    %define O 1
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

        out stdout, al

        mov ah, 0
        mov di, ax
        sub di, '1'

        cmp di, 0
        jl delete_game_loop_text

        cmp di, 8
        jg delete_game_loop_text

        cmp [bx+di], 0
        jne delete_game_loop_text

        call get_turn

        mov [bx+di], dx

        mov al, newline
        out stdout, al

        call print_board

        jmp game_loop

    hlt

    delete_game_loop_text:
        mov cx, 13
        delete_loop:
            mov al, backspace
            out stdout, al
            loop delete_loop
        jmp game_loop

get_turn:
    push di
    push cx

    mov di, 0

    get_turn_loop:
        mov al, [bx+di]

        cmp al, 0
        jne inc_cx

        get_turn_loop_end:

        inc di
        cmp di, 9
        jne get_turn_loop

    cmp cx, 0
    je ret_X

    cmp cx, 1
    je ret_O

    cmp cx, 2
    je ret_X

    cmp cx, 3
    je ret_O

    cmp cx, 4
    je ret_X

    cmp cx, 5
    je ret_O

    cmp cx, 6
    je ret_X

    cmp cx, 7
    je ret_O

    cmp cx, 8
    je ret_X

    cmp cx, 9
    je ret_O

    inc_cx:
        inc cx
        jmp get_turn_loop_end

    ret_X:
        mov dx, X

        pop cx
        pop di
        ret

    ret_O:
        mov dx, O

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

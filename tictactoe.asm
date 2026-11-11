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
    mov sp, 0xFF00

    mov ax, 0x2000
    mov ds, ax

    game_loop:
        call get_result

        cmp dh, 1
        je gameover

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

        cmp [di], 0
        jne delete_game_loop_text

        call get_turn

        mov [di], dl

        mov al, newline
        out stdout, al

        call print_board

        jmp game_loop

    halt:
        mov al, newline
        out stdout, al
        hlt

    gameover:
        cmp dl, X
        je X_wins

        cmp dl, O
        je O_wins

        cmp dl, Empty
        je game_drawn

    X_wins:
        mov ax, 'X '
        out stdout, ax
        mov ax, 'Wi'
        out stdout, ax
        mov ax, 'ns'
        out stdout, ax
        mov al, '!'
        out stdout, al
        hlt

    O_wins:
        mov ax, 'O '
        out stdout, ax
        mov ax, 'Wi'
        out stdout, ax
        mov ax, 'ns'
        out stdout, ax
        mov al, '!'
        out stdout, al
        hlt

    game_drawn:
        mov ax, 'Ga'
        out stdout, ax
        mov ax, 'me'
        out stdout, ax
        mov ax, ' D'
        out stdout, ax
        mov ax, 'ra'
        out stdout, ax
        mov ax, 'wn'
        out stdout, ax
        mov al, '!'
        out stdout, al
        hlt

    delete_game_loop_text:
        mov cx, 13
        delete_loop:
            mov al, backspace
            out stdout, al
            loop delete_loop
        jmp game_loop

get_result:
    mov dl, Empty
    mov dh, 0
    ret

get_turn:
    push di
    push cx

    mov di, 0
    xor cl, cl

    get_turn_loop:
        mov al, [di]

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
        mov al, [di]

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

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

    .loop:
        call get_result

        cmp dh, 1
        jne .no_gameover
            cmp dl, X
            jne .not_X_wins
                mov ax, 'X '
                out stdout, ax
                mov ax, 'Wi'
                out stdout, ax
                mov ax, 'ns'
                out stdout, ax
                mov al, '!'
                out stdout, al
                hlt
            .not_X_wins:

            cmp dl, O
            jne .not_O_wins
                mov ax, 'X '
                out stdout, ax
                mov ax, 'Wi'
                out stdout, ax
                mov ax, 'ns'
                out stdout, ax
                mov al, '!'
                out stdout, al
                hlt
            .not_O_wins:

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
        .no_gameover:

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
        jne .no_quit
            mov al, newline
            out stdout, al
            hlt
        .no_quit:

        out stdout, al

        cbw
        mov di, ax
        sub di, '1'

        cmp di, 0
        jl .delete_text

        cmp di, 8
        jg .delete_text

        cmp [di], 0
        jne .delete_text

        call get_turn

        mov [di], dl

        mov al, newline
        out stdout, al

        call print_board

        jmp .loop

        .delete_text:
            mov cx, 13
            .delete_loop:
                mov al, backspace
                out stdout, al
                loop .delete_loop
            jmp .loop

get_result:
    mov dl, Empty
    mov dh, 0
    ret

get_turn:
    push di
    push cx

    mov di, 0
    xor cl, cl

    .loop:
        mov al, [di]

        cmp al, 0
        je .no_inc_cl
            inc cl
        .no_inc_cl:

        inc di
        cmp di, 9
        jne .loop

    test cl, 1
    je .ret_X
    jne .ret_O

    .ret_X:
        mov dl, X

        pop cx
        pop di
        ret

    .ret_O:
        mov dl, O

        pop cx
        pop di
        ret

print_board:
    push ax
    push di

    mov di, 0

    .loop:
        mov al, [di]

        cmp al, Empty
        jne .no_print_empty
            mov al, '-'
            out stdout, al
        .no_print_empty:

        cmp al, X
        jne .no_print_X
            mov al, 'X'
            out stdout, al
        .no_print_X:

        cmp al, O
        jne .no_print_O
            mov al, 'O'
            out stdout, al
        .no_print_O:

        cmp di, 2
        je .print_newline

        cmp di, 5
        je .print_newline

        cmp di, 8
        je .print_newline

        .loop_end:

        inc di
        cmp di, 9
        jne .loop

    pop di
    pop ax
    ret

    .print_newline:
        mov al, newline
        out stdout, al
        jmp .loop_end

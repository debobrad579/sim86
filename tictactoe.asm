bits 16

main:
    %define stdin 00h
    %define stdout 01h
    %define newline 0x0A
    %define carriage_return 0x0D

    %define X 1
    %define O -1
    %define Empty 0

    mov ax, 0x1000
    mov ss, ax
    mov sp, 0xFFFF

    mov ax, 0x2000
    mov ds, ax

    .play_as_loop:
        mov ax, 'Pl'
        out stdout, ax
        mov ax, 'ay'
        out stdout, ax
        mov ax, ' a'
        out stdout, ax
        mov ax, 's '
        out stdout, ax

        in al, stdin

        cmp al, 'q'
        jne .no_play_quit
            hlt
        .no_play_quit:

        cmp al, 'X'
        je .play_as_X
        cmp al, 'x'
        je .play_as_X

        cmp al, 'O'
        je .play_as_O
        cmp al, 'o'
        je .play_as_O

        mov al, carriage_return
        out stdout, al
        jmp .play_as_loop
        
    .play_as_X:
    mov [9], X
    jmp .loop
    
    .play_as_O:
    mov [9], O

    .loop:
        mov al, newline
        out stdout, al
        call print_board

        mov ax, 0x5B1B
        out stdout, ax

        mov ax, 0x4134
        out stdout, ax

        mov al, carriage_return
        out stdout, al

        call get_result

        cmp dh, 1
        jne .no_gameover
            jmp gameover
        .no_gameover:

        call get_turn
        cmp dl, [9]
        je .play

        mov ax, 'Th'
        out stdout, ax
        mov ax, 'in'
        out stdout, ax
        mov ax, 'ki'
        out stdout, ax
        mov ax, 'ng'
        out stdout, ax
        mov ax, '..'
        out stdout, ax
        mov al, '.'
        out stdout, al
        mov al, newline
        out stdout, al

        call minimax
        mov di, bx
        call make_move

        call cursor_up
        jmp .loop

        .play:

        mov ax, 'Yo'
        out stdout, ax
        mov ax, 'ur'
        out stdout, ax
        mov ax, ' T'
        out stdout, ax
        mov ax, 'ur'
        out stdout, ax
        mov al, 'n'
        out stdout, al
        mov ax, '  '
        out stdout, ax
        mov al, newline
        out stdout, al

        mov di, 0
        
        .input_loop:
            in al, stdin

            cmp al, newline
            jne .no_enter
                cmp di, 3
                jl .finish_enter
                    call cursor_up
                    cmp di, 6
                    jl .finish_enter
                        call cursor_up
                .finish_enter:
                call make_move
                call cursor_up
                jmp .loop
            .no_enter:

            cmp al, 'q'
            jne .no_quit
                mov al, newline
                out stdout, al
                hlt
            .no_quit:

            cmp al, 'h'
            jne .no_left
                cmp di, 0
                je .no_left
                cmp di, 3
                je .no_left
                cmp di, 6
                je .no_left
                call cursor_left
                dec di
            .no_left:

            cmp al, 'j'
            jne .no_down
                cmp di, 6
                jge .no_down
                call cursor_down
                add di, 3
            .no_down:

            cmp al, 'k'
            jne .no_up
                cmp di, 2
                jle .no_up
                call cursor_up
                sub di, 3
            .no_up:

            cmp al, 'l'
            jne .no_right
                cmp di, 2
                je .no_right
                cmp di, 5
                je .no_right
                cmp di, 8
                je .no_right
                call cursor_right
                inc di
            .no_right:

            jmp .input_loop

gameover:
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
        jmp .end
    .not_X_wins:
    cmp dl, O
    jne .not_O_wins
        mov ax, 'O '
        out stdout, ax
        mov ax, 'Wi'
        out stdout, ax
        mov ax, 'ns'
        out stdout, ax
        mov al, '!'
        out stdout, al
        jmp .end
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

    .end:
    mov ax, '  '
    out stdout, ax
    mov ax, '  '
    out stdout, ax
    mov al, newline
    out stdout, al

    call print_board
    hlt

minimax:
    push ax
    push di

    call get_turn
    cmp dl, O
    je .O_preloop

    mov al, -2
    mov di, 8

    .loop:
        cmp [di], Empty
        jne .continue
        
        call make_move
        call min_value
        call unmake_move

        cmp dl, al
        jle .continue

        mov al, dl
        mov bx, di

        .continue:
        dec di
        jns .loop

    jmp .return

    .O_preloop:

    mov al, 2
    mov di, 8

    .O_loop:
        cmp [di], Empty
        jne .O_continue
        
        call make_move
        call max_value
        call unmake_move

        cmp dl, al
        jge .O_continue

        mov al, dl
        mov bx, di

        .O_continue:
        dec di
        jns .O_loop

    .return:
    pop di
    pop ax
    ret

max_value:
    push ax
    push di

    call get_result
    cmp dh, 1
    je .return

    mov al, -2
    mov di, 8

    .loop:
        cmp [di], Empty
        jne .continue
        
        call make_move
        call min_value
        call unmake_move

        cmp dl, al
        jle .continue

        mov al, dl

        .continue:
        dec di
        jns .loop

    mov dl, al

    .return:
    pop di
    pop ax
    ret

min_value:
    push ax
    push di

    call get_result
    cmp dh, 1
    je .return

    mov al, 2
    mov di, 8

    .loop:
        cmp [di], Empty
        jne .continue
        
        call make_move
        call max_value
        call unmake_move

        cmp dl, al
        jge .continue

        mov al, dl

        .continue:
        dec di
        jns .loop

    mov dl, al

    .return:
    pop di
    pop ax
    ret

make_move:
    call get_turn
    mov [di], dl
    ret
    
unmake_move:
    mov [di], Empty
    ret

get_result:
    push di
    push cx

    xor dh, dh

    mov di, 8

    .loop_horizontal:
        mov dl, [di]
        cmp dl, Empty
        je .continue_horizontal
        cmp dl, [di-1]
        jne .continue_horizontal
        cmp dl, [di-2]
        jne .continue_horizontal
            mov dh, 1
            jmp .return
        .continue_horizontal:

        sub di, 3
        jns .loop_horizontal

    mov di, 2

    .loop_vertical:
        mov dl, [di]
        cmp dl, Empty
        je .continue_vertical
        cmp dl, [di+3]
        jne .continue_vertical
        cmp dl, [di+6]
        jne .continue_vertical
            mov dh, 1
            jmp .return
        .continue_vertical:

        dec di
        jns .loop_vertical

    mov dl, [0]
    cmp dl, Empty
    je .continue_diagonal
    cmp dl, [4]
    jne .continue_diagonal
    cmp dl, [8]
    jne .continue_diagonal
        mov dh, 1
        jmp .return
    .continue_diagonal:

    mov dl, [2]
    cmp dl, Empty
    je .continue_to_full
    cmp dl, [4]
    jne .continue_to_full
    cmp dl, [6]
    jne .continue_to_full
        mov dh, 1
        jmp .return
    .continue_to_full:

    xor cl, cl
    mov di, 8

    .loop_full:
        mov dl, [di]

        cmp dl, Empty
        je .no_inc_cl
            inc cl
        .no_inc_cl:
        
        dec di
        jns .loop_full

    cmp cl, 9
    jne .game_ongoing

    mov dl, Empty
    mov dh, 1
    jmp .return

    .game_ongoing:
    
    mov dl, Empty
        
    .return:

    pop cx
    pop di
    ret
    
get_turn:
    push di
    push cx

    xor cl, cl
    mov di, 8

    .loop:
        mov dl, [di]

        cmp dl, Empty
        je .no_inc_cl
            inc cl
        .no_inc_cl:

        dec di
        jns .loop

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

cursor_left:
    push ax

    mov ax, 0x5B1B
    out stdout, ax
    mov ax, 0x4431
    out stdout, ax

    pop ax
    ret

cursor_down:
    push ax
    
    mov ax, 0x5B1B
    out stdout, ax
    mov ax, 0x4231
    out stdout, ax

    pop ax
    ret

cursor_up:
    push ax

    mov ax, 0x5B1B
    out stdout, ax
    mov ax, 0x4131
    out stdout, ax

    pop ax
    ret

cursor_right:
    push ax

    mov ax, 0x5B1B
    out stdout, ax
    mov ax, 0x4331
    out stdout, ax

    pop ax
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

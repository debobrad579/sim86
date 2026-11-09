push word [bp + si]
push word [3000]
push word [bx + di - 30]
push cx
push ax
push dx
push cs

pop word [bp + si]
pop word [3]
pop word [bx + di - 3000]
pop sp
pop di
pop si
pop ds

call [39201]
call [bp - 100]
call sp
call ax
call [bp+si-0x3a]
call far [bp+si-0x3a]
call 11804
call 123:456

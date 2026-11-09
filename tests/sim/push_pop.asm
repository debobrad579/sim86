mov sp, 1000

mov ax, 1
mov bx, 2
mov cx, 3
mov dx, 4

push ax
push bx
push cx
push dx

pop ax
pop bx
pop cx
pop dx

hlt

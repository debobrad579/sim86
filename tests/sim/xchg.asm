bits 16

mov ax, 10
mov bx, 20
mov cx, 30
mov dx, 40

xchg ax, bx
xchg cx, dx
xchg ax, cx
xchg bx, dx

hlt

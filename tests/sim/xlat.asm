bits 16

mov ax, 0x1000
mov ds, ax

mov bx, 0x0200
mov byte [0x0204], 0x0055
mov al, 0x0004

xlat

hlt

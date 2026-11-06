bits 16

mov byte [1000], 1
mov byte [1001], 2
mov byte [1002], 3
mov byte [1003], 4

mov bx, 1000
mov byte [bx + 2], 10

mov bl, byte [1000]
mov cl, byte [1001]
mov dl, byte [1002]
mov al, byte [1003]

hlt

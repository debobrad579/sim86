bits 16

; Signed displacements
mov ax, [bx + di - 37]
mov [si - 300], cx
mov dx, [bx - 32]
mov cx, [bp + si - 128]
mov [bx + 256], dx

; Explicit sizes
mov [bp + di], byte 7
mov [di + 901], word 347
mov [bx + si], byte 25
mov [bp - 64], word 1024

; Direct address
mov bp, [5]
mov bx, [3458]
mov cx, [1024]
mov dx, [65535]

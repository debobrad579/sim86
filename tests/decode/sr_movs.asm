bits 16

mov ds, ax
mov es, bx
mov ss, cx

mov ax, ds
mov bx, es
mov cx, ss
mov dx, cs

mov ds, [1000h]
mov es, [2000h]
mov ss, [3000h]

mov [1000h], ds
mov [2000h], es
mov [3000h], ss

mov ds, [bx]
mov es, [si]
mov ss, [bp]
mov ds, [di]

mov ds, [bx+si]
mov es, [bx+di]
mov ss, [bp+si]
mov ds, [bp+di]

mov ds, [bx+10h]
mov es, [bp+20h]
mov ss, [si+30h]

mov al, cs:[bx + si]
mov bx, ds:[bp + di]
mov dx, es:[bp]
mov ah, ss:[bx + si + 4]

mov cx, ds:[bx + di]
mov al, es:[bp + si + 8]
mov si, ss:[bx]
mov dl, cs:[bp + di + 2]
mov ax, ds:[si + 6]
mov bh, es:[bx + si + 10]

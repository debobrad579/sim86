bits 16

; 512x256 RGBA image = 524,288 bytes
; Placed at physical 0x10000 (segment 0x1000), 16 chunks of 32 kb each

mov si, 0x1000         ; si = segment of the current chunk
mov bp, 16             ; bp = chunks left
mov ax, 0              ; ah = green, steps up once per chunk

chunk_loop:
	mov ds, si
	mov bx, 0          ; pixel pointer within this chunk
	mov dx, 255        ; dl = blue, fades down within each chunk
	mov di, 16         ; di = rows left in this chunk

	row_loop:
		mov cx, 512    ; cl = red, wraps around mid-row
		col_loop:
			mov byte [bx + 0], cl  ; Red
			mov byte [bx + 1], ah  ; Green
			mov byte [bx + 2], dl  ; Blue
			mov byte [bx + 3], 255 ; Alpha
			add bx, 4
			loop col_loop

		sub dl, 15     ; blue gets darker each row
		sub di, 1
		jnz row_loop

	add ah, 15         ; next band is greener
	add si, 0x800      ; next 32 KB chunk
	sub bp, 1
	jnz chunk_loop

hlt

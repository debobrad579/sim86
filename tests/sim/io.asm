main:
	mov di, 10_000
	mov sp, 50_000

	mov ax, 'Ty'
	out 01h, ax
	mov ax, 'pe'
	out 01h, ax
	mov ax, ': '
	out 01h, ax

	mov cx, 10

	loop_start:
		in al, 00h
		out 01h, al
		cmp al, 08h
		jz backspace
		cmp al, 0Ah
		jnz loop_start

	hlt

backspace:
	push ax
	mov al, " "
	out 01h, al
	mov al, 08h
	out 01h, al
	pop ax
	jmp loop_start

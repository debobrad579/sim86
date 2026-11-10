main:
	%define newline 0Ah
	%define backspace 08h
	%define stdin 00h
	%define stdout 01h

	mov di, 10_000
	mov sp, 50_000

	mov ax, 'Ty'
	out stdout, ax
	mov ax, 'pe'
	out stdout, ax
	mov ax, ': '
	out stdout, ax

	mov cx, 6

	loop_start:
		in al, stdin
		cmp al, backspace
		jz delete
		out stdout, al
		inc cx
		loop_mid:
		cmp al, newline
		jnz loop_start

	hlt

delete:
	cmp cx, 7
	jns delete_character
	jmp loop_start

delete_character:
	push ax
	mov al, backspace
	out stdout, al
	mov al, " "
	out stdout, al
	mov al, backspace
	out stdout, al
	pop ax
	dec cx
	jmp loop_mid

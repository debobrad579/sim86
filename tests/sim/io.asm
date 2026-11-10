main:
	%define newline 0Ah
	%define backspace 08h
	%define stdin 00h
	%define stdout 01h

	mov bx, 10_000
	mov sp, 50_000

	mov ax, 'Na'
	out stdout, ax
	mov ax, 'me'
	out stdout, ax
	mov ax, ': '
	out stdout, ax

	loop_start:
		in al, stdin
		cmp al, backspace
		jz delete
		out stdout, al
		inc di
		mov [bx+di], al
		loop_mid:
		cmp al, newline
		jnz loop_start

	mov dx, di
	mov di, 0

	mov ax, 'He'
	out stdout, ax
	mov ax, 'll'
	out stdout, ax
	mov ax, 'o,'
	out stdout, ax
	mov al, ' '
	out stdout, al

	loop2_start:
		inc di
		mov al, [bx+di]
		out stdout, al
		cmp di, dx
		jnz loop2_start

	hlt

delete:
	cmp di, 1
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
	dec di
	jmp loop_mid

main:
	mov sp, 40_000

	mov ax, 1
	mov bx, 2

	call func

	pushf
	test ax, bx
	popf

	hlt

func:
	push ax
	push bx
	mov ax, 100
	mov bx, 50
	cmp bx, ax
	pop bx
	pop ax
	ret

bits 16

add cx, [bp]
add dx, [bx + si]
add [bp + di + 5000], ah
add [bx], al
add sp, 392
add si, 5
add ax, 1000
add ah, 30
add al, 9
add cx, bx
add ch, al

adc cx, [bp]
adc dx, [bx + si]
adc [bp + di + 5000], ah
adc [bx], al
adc sp, 392
adc si, 5
adc ax, 1000
adc ah, 30
adc al, 9
adc cx, bx
adc ch, al

sub cx, [bp]
sub dx, [bx + si]
sub [bp + di + 5000], ah
sub [bx], al
sub sp, 392
sub si, 5
sub ax, 1000
sub ah, 30
sub al, 9
sub cx, bx
sub ch, al

sbb cx, [bp]
sbb dx, [bx + si]
sbb [bp + di + 5000], ah
sbb [bx], al
sbb sp, 392
sbb si, 5
sbb ax, 1000
sbb ah, 30
sbb al, 9
sbb cx, bx
sbb ch, al

cmp bx, cx
cmp dh, [bp + 390]
cmp [bp + 2], si
cmp bl, 20
cmp byte [bx], 34
cmp ax, 23909

and al, ah
and ch, cl
and bp, si
and di, sp
and al, 93
and ax, 20392
and [bp + si + 10], ch
and [bx + di + 1000], dx
and bx, [bp]
and cx, [4384]
and byte [bp - 39], 239
and word [bx + si - 4332], 10328

test bx, cx
test dh, [bp + 390]
test [bp + 2], si
test bl, 20
test byte [bx], 34
test ax, 23909

or al, ah
or ch, cl
or bp, si
or di, sp
or al, 93
or ax, 20392
or [bp + si + 10], ch
or [bx + di + 1000], dx
or bx, [bp]
or cx, [4384]
or byte [bp - 39], 239
or word [bx + si - 4332], 10328

xor al, ah
xor ch, cl
xor bp, si
xor di, sp
xor al, 93
xor ax, 20392
xor [bp + si + 10], ch
xor [bx + di + 1000], dx
xor bx, [bp]
xor cx, [4384]
xor byte [bp - 39], 239
xor word [bx + si - 4332], 10328

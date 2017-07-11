BITS 64
loop:
xchg bx, bx
mov rax, 0xDEADBEEF
int 80
jmp loop
BITS 64
mov rax, 0xDEADBEEF
int 80
loop:
jmp loop
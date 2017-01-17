global gdt64
global gdt64_code
global gdt64_data
global gdt64_pointer

section .rodata
bits 64
gdt64:
	dq 0								; zero entry
gdt64_code: equ $ - gdt64
	dq (1 << 44) | (1 << 47) | (1 << 41) | (1 << 43) | (1 << 53)	; code segment
gdt64_pointer:
	dw $ - gdt64 - 1
	dq gdt64

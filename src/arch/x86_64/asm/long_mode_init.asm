global long_mode_start

extern setup_SSE
extern rust_main

section .text
bits 64
long_mode_start:
	call setup_SSE
	call rust_main

	mov rax, 0x2f592f412f4b2f4f
	mov qword [0xb8000], rax
	hlt

global long_mode_start

extern setup_SSE
extern higher_half_start

section .text
bits 64
long_mode_start:
	call setup_SSE

	; Setup higher half stack
	mov rsp, stack_top

	; Jump to higher half
	mov rax, higher_half_start
	jmp rax

section .stack
stack_bottom:
	times 4096*4 db 0
stack_top:

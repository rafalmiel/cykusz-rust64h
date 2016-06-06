global long_mode_start

extern setup_SSE
extern higher_half_start
extern unmap_lower_half

section .text
bits 64
long_mode_start:
	call setup_SSE

	; Setup higher half stack
	mov rsp, stack_top

	; Jump to higher half
	mov rax, higher_half_start
	jmp rax

	hlt

section .stack
stack_bottom:
	resb 4096*2
stack_top:

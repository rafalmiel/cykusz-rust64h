section .text
bits 64
global switch_to
switch_to:
	mov rax, rdi	; old stack ptr
	mov rdx, rsi	; new stack ptr

	pushfq			; push regs to current ctx
	push rbp
	push rbx
	push r12
	push r13
	push r14
	push r15

	mov [rax], rsp	; update old ctx ptr with current stack ptr
	mov rsp, rdx	; switch to new stack

	pop r15
	pop r14
	pop r13
	pop r12
	pop rbx
	pop rbp
	popfq

	ret

global read_rsp
read_rsp:
	mov rax, rsp
	ret
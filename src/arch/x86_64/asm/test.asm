global test_cpuid
global test_long_mode
global test_multiboot

extern error

section .text
bits 32

test_cpuid:
	pushfd			; Store the FLAGS-register.
	pop eax			; Restore the A-register
	mov ecx, eax		; Set the C-register to the A-register
	xor eax, 1 << 21	; Flip the ID-bit, which is bit 21
	push eax		; Store the A-register
	popfd			; Restore the FLAGS-register
	pushfd			; Store the FLAGS-register
	pop eax			; Restore the A-register
	push ecx		; Store the C-register
	popfd			; Restore the FLAGS-register
	xor eax, ecx		; Do a XOR-operation on the A-register and the C-register
	jz .no_cpuid		; The zero flag is set, no CPUID
	ret			; CPUID is available to use
.no_cpuid:
	mov al, "1"
	jmp error

test_long_mode:
	mov eax, 0x80000000	; Set the A-register to 0x80000000
	cpuid			; CPU identification
	cmp eax, 0x80000001	; Compare the A-register with 0x80000001
	jb .no_long_mode	; It is less, there is no long mode
	mov eax, 0x80000001	; Set the A-register to 0x80000001
	cpuid			; CPU identification
	test edx, 1 << 29	; Test if the LM-bit, which is bit 29, is set in the D-register
	jz .no_long_mode	; They aren't, there is no long mode
	ret
.no_long_mode:
	mov al, "2"
	jmp error

test_multiboot:
	cmp eax, 0x36d76289
	jne .no_multiboot
	ret
.no_multiboot:
	mov al, "0"
	jmp error

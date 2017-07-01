global start
global gdt64_code_offset
global error

extern long_mode_start
extern test_multiboot
extern test_cpuid
extern test_long_mode
extern setup_page_tables
extern enable_paging

section .text
bits 32
start:
  cli

	mov esp, boot_stack_top
	mov edi, ebx       ;Multiboot address

	call test_multiboot
	call test_cpuid
	call test_long_mode

	call setup_page_tables
	call enable_paging

	lgdt [gdt64.pointer]

	jmp gdt64.code:long_mode_start

error:
	mov dword [0xb8000], 0x4f524f45 ; ER
	mov dword [0xb8004], 0x4f3a4f52 ; R:
	mov dword [0xb8008], 0x4f204f20 ;
	mov byte  [0xb8008], al		; err code
	hlt

section .bss
boot_stack_bottom:
	resb 512
boot_stack_top:

; lower half gdt
section .rodata
bits 64
gdt64:
	dq 0														    ; zero entry
.code: equ $ - gdt64
	dw 0			; Limit (low)
	dw 0			; Base (low)
	db 0			; Base (middle)
	db 10011011b	; Access (Pr Privl=0 1 Ex Dc=0 RW Ac)
	db 00100000b	; Flags (64bit) Limit (high)
	db 0			; Base (high)
.pointer:
	dw $ - gdt64 - 1
	dq gdt64
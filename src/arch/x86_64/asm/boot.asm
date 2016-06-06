global start
global gdt64_code_offset
global error

extern long_mode_start
extern gdt64
extern gdt64_code
extern gdt64_data
extern gdt64_pointer
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

	lgdt [gdt64_pointer]

	; update selectors
	mov ax, gdt64_data
	mov ss, ax
	mov ds, ax
	mov es, ax

	jmp gdt64_code:long_mode_start

error:
	mov dword [0xb8000], 0x4f524f45 ; ER
	mov dword [0xb8004], 0x4f3a4f52 ; R:
	mov dword [0xb8008], 0x4f204f20 ;
	mov byte  [0xb8008], al		; err code
	hlt

section .bss
boot_stack_bottom:
	resb 4096
boot_stack_top:

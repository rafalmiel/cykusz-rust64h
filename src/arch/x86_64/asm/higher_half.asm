extern x86_64_rust_main
extern __p4_table

global higher_half_start

section .text
bits 64

higher_half_start:
  ; Unmap lower half
  mov rax, 0
  mov [__p4_table], rax

  ; switch to higher half gdt
  mov rax, gdt64_pointer_hh
  lgdt [rax]

  ; update selectors
  mov ax, 0
  mov ss, ax
  mov ds, ax
  mov es, ax
  mov fs, ax
  mov gs, ax

  ; Reload page table
  mov rax, cr3
  mov cr3, rax

  ; Jump to rust code
  call x86_64_rust_main

.loop:
  hlt
  jmp $

; We set 40th access bit on so that bochs will not try to write to readonly page
; causing segfault
section .rodata
bits 64
gdt64_hh:
	dq 0                                                          ; zero entry
gdt64_code_hh: equ $ - gdt64_hh
	dq (1 << 44) | (1 << 47) | (1 << 43) | (1 << 40) | (1 << 53)  ; code segment
gdt64_pointer_hh:
	dw $ - gdt64_hh - 1
	dq gdt64_hh

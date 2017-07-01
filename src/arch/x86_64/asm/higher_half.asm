extern x86_64_rust_main
extern __p4_table
extern gdt64.pointer
extern gdt64.data

global higher_half_start

section .text
bits 64

higher_half_start:
  ; Unmap lower half
  mov rax, 0
  mov [__p4_table], rax

  ; switch to higher half gdt
  mov rax, gdt64.pointer
  lgdt [rax]

  ; update selectors
  mov ax, gdt64.data
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

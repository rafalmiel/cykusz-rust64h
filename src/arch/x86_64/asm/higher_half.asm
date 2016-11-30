extern x86_64_rust_main
extern __p4_table

global higher_half_start

section .text
bits 64

higher_half_start:
  ; Unmap lower half
  mov rax, 0
  mov [__p4_table], rax

  ; Reload page table
  mov rax, cr3
  mov cr3, rax

  ; Jump to rust code
  call x86_64_rust_main

  .loop:
    hlt
    jmp $

extern rust_main
extern p4_table

global higher_half_start

section .text
bits 64

higher_half_start:
  ; Unmap lower half
  mov eax, 0
  mov [p4_table], eax

  ; Reload page table
  mov rax, cr3
  mov cr3, rax

  ; Jump to rust code
  call rust_main

extern x86_64_rust_main
extern __p4_table
extern gdt64.pointer
extern gdt64.data
extern gdt64

global higher_half_start

global switch_to_user

section .text
higher_half_start:
	; Setup higher half stack
    mov rsp, stack_top

    ; Unmap lower half
    mov rax, 0
    mov [__p4_table], rax

    ; switch to higher half gdt
    mov rax, gdt64.pointer
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
    mov rsi, stack_top
    mov rdx, gdt64
    call x86_64_rust_main

.loop:
    hlt
    jmp $

switch_to_user:
    cli

    mov ax,0x23
    mov ds,ax
    mov es,ax
    mov fs,ax
    mov gs,ax

    xchg bx, bx
    mov rax, 0x400000
    mov rbx, rsp
	push 0x23
	push rbx
	push 0x200
	push 0x1B
	push rax
	iretq

section .stack
stack_bottom:
	times 4096*4 db 0
stack_top:
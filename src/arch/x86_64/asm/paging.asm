global setup_page_tables
global enable_paging
global __p4_table

section .text
bits 32

; kernel phys mem start 0xffff800000000000
; kernel code mem start 0xffffff0000000000
; kernel heap mem start 0xfffff80000000000

; ecx : offset
; ebx : page addr
map_2g_pages:
	push eax
	push ebx
	push ecx
	push edx

	mov ebx, [esp + 4*5]
	mov ecx, [esp + 4*6]

	mov edx, 0
.map:
	mov eax, 0x200000
	push edx
	mul edx
	pop edx

	add eax, ecx
	or eax, 0b10000011

	mov [ebx + edx * 8], eax

	push eax
	mov eax, [esp + 4*8]
	mov [ebx + edx * 8 + 4], eax
	pop eax

	inc edx
	cmp edx, 512
	jne .map

	pop edx
	pop ecx
	pop ebx
	pop eax

	ret

setup_page_tables:
	; map first P4 entry to P3 table
	mov eax, __p3_table
	or eax, 0b011		; present + writable
	mov [__p4_table], eax

	; Entry for higher half kernel
	mov eax, __p3_table_high
	or eax, 0b11 ; present + writable
	mov [__p4_table + 510 * 8], eax

	; Entry for physical mem kernel mapping at 0xffff800000000000
	mov eax, __p3_table_phys
	or eax, 0b011 ; present + writable
	mov [__p4_table + 256 * 8], eax

	mov eax, __p2_table
	or eax, 0b11
	mov [__p3_table], eax

	mov eax, __p2_table_high
	or eax, 0b11		; present + present + writable
	mov [__p3_table_high], eax

	push 0
	push 0
	push __p2_table
	call map_2g_pages
	add esp, 4*3

	push 0
	push 0
	push __p2_table_high
	call map_2g_pages
	add esp, 4*3

	mov ecx, 0
map_p3_table_phys:
	mov eax, 0x40000000
	mul ecx

	push edx
	push eax

	mov ebx, __p2_table_phys
	mov eax, ecx
	mov edx, 4096
	mul edx
	add ebx, eax
	push ebx
	or ebx, 0b11

	mov [__p3_table_phys + 8*ecx], ebx

	xchg bx, bx
	call map_2g_pages
	add esp, 4*3

	inc ecx
	cmp ecx, 16
	jne map_p3_table_phys
	ret

enable_paging:
	; load P4 to cr3 register (cpu uses this to access the P4 table)
	mov eax, __p4_table
	mov cr3, eax

	; enable PAE-flag in cr4 (Physical Address Extension)
	mov eax, cr4
	or eax, 1 << 5
	mov cr4, eax

	; set the long mode bit in the EFER MSR (model specific register)
	mov ecx, 0xC0000080
	rdmsr
	or eax, 1 << 8
	wrmsr

	; enable paging in the cr0 register
	mov eax, cr0
	or eax, 1 << 31
	or eax, 1 << 16
	mov cr0, eax

	xchg bx, bx
	ret

section .bss
align 4096
__p4_table:
	resb 4096
__p3_table:
	resb 4096
__p3_table_phys:
	resb 4096
__p3_table_high:
	resb 4096
__p2_table:
	resb 4096
__p2_table_phys:
    resb 16*4096
__p2_table_high:
	resb 4096

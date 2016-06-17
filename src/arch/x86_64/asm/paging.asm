global setup_page_tables
global enable_paging
global p4_table

section .text
bits 32

setup_page_tables:
	; map first P4 entry to P3 table
	mov eax, p3_table
	or eax, 0b11		; present + writable
	mov [p4_table], eax

	; Entry for higher half kernel
	mov eax, p3_table_high
	or eax, 0b11 ; huge present + writable
	mov [p4_table + 510 * 8], eax

	; Entry for physical mem kernel mapping at 0xffff800000000000
	mov eax, p3_table_phys
	or eax, 0b11 ; huge present + writable
	mov [p4_table + 256 * 8], eax

	; Recursive page table mapping
;	mov eax, p4_table
;	or eax, 0b11 ; present + writable
;	mov [p4_table + 511 * 8], eax

	;map first P3 entry to 1 GB huge page
	mov eax, 0
	or eax, 0b10000011		; Huge table + present + writable
	mov [p3_table], eax

	;map first P3 high table entry to 1GB huge page
	mov eax, 0
	or eax, 0b10000011		; Huge table + present + writable
	mov [p3_table_high], eax

	; Map all P3 table phys tables to 1 GB
	mov ecx, 0
.map_p3_table_phys:
	mov eax, 0x40000000	; 1GB
	mul ecx
	or eax, 0b10000011	; Huge table + present + writable
	mov [p3_table_phys + ecx * 8], eax
	inc ecx
	cmp ecx, 512
	jne .map_p3_table_phys

	ret

enable_paging:
	; load P4 to cr3 register (cpu uses this to access the P4 table)
	mov eax, p4_table
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

	ret

section .bss
align 4096
p4_table:
	resb 4096
p3_table:
	resb 4096
p3_table_high:
	resb 4096
p3_table_phys:
	resb 4096

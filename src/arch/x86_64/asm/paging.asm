global setup_page_tables
global enable_paging
global unmap_lower_half
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
	or eax, 0b11 ; present + writable
	mov [p4_table + 256 * 8], eax

	; Recursive page table mapping
	mov eax, p4_table
	or eax, 0b11 ; present + writable
	mov [p4_table + 511 * 8], eax

	;map first P3 entry to P2 table
	mov eax, p2_table
	or eax, 0b11		; present + writable
	mov [p3_table], eax

	;map first P3 entry to P2 table high (higher half)
	mov eax, p2_table_high
	or eax, 0b11		; present + writable
	mov [p3_table_high], eax

	; map each P2 entry to a huge 2MiB page
	mov ecx, 0
.map_p2_table:
	; map ecx-tx P2 entry to a huge page that starts at address 2MiB*ecx
	mov eax, 0x200000		; 2MiB
	mul ecx				; start address of ecx-th page
	or eax, 0b10000111		; preset + writable + huge
	mov [p2_table + ecx * 8], eax 	; map ecx-th entry

	inc ecx
	cmp ecx, 512
	jne .map_p2_table

	mov ecx, 0
.map_p2_table_high:
	; map ecx-tx P2 high entry to a huge page that starts at address 2MiB*ecx
	mov eax, 0x200000		; 2MiB
	mul ecx				; start address of ecx-th page
	or eax, 0b10000111		; preset + writable + huge
	mov [p2_table_high + ecx * 8], eax 	; map ecx-th entry

	inc ecx
	cmp ecx, 512
	jne .map_p2_table_high
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

unmap_lower_half:
	mov eax, 0x0
	mov [p4_table], eax
	mov eax, p4_table
	mov cr3, eax

section .bss
align 4096
p4_table:
	resb 4096
p3_table:
	resb 4096
p3_table_high:
	resb 4096
p2_table:
	resb 4096
p2_table_high:
	resb 4096

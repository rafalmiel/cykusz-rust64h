global gdt64
global gdt64.code
global gdt64.data
global gdt64.pointer

; higher half gdt
; We set 40th access bit on so that bochs will not try to write to readonly page
; causing segfault
; http://wiki.osdev.org/Global_Descriptor_Table
section .rodata
bits 64
gdt64:
	dq 0								; zero entry
.code: equ $ - gdt64
	dw 0			; Limit (low)
	dw 0			; Base (low)
	db 0			; Base (middle)
	db 10011011b	; Access (Pr Privl=0 1 Ex Dc=0 RW Ac)
	db 00100000b	; Flags (64bit) Limit (high)
	db 0			; Base (high)
.data: equ $ - gdt64
	dw 0			; Limit (low)
	dw 0			; Base (low)
	db 0			; Base (middle)
	db 10010010b	; Access (Pr Privl=0 1 Ex=0 Dc=0 RW Ac=0)
	db 00100000b	; Flags (64bit) Limit (high)
	db 0			; Base (high)
.pointer:
	dw $ - gdt64 - 1
	dq gdt64

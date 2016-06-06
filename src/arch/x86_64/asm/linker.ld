ENTRY(start)

SECTIONS {
	. = 1M;

	__kernel_boot_start = .;
	.boot :
	{
		KEEP(*(.multiboot_header))
		build/arch/x86_64/asm/boot.o(.text* .rodata* .data* .bss*)
		build/arch/x86_64/asm/gdt.o(.text* .rodata* .data* .bss*)
		build/arch/x86_64/asm/paging.o(.text* .rodata* .data* .bss*)
		build/arch/x86_64/asm/sse.o(.text* .rodata* .data* .bss*)
		build/arch/x86_64/asm/test.o(.text* .rodata* .data* .bss*)
		build/arch/x86_64/asm/long_mode_init.o(.text* .rodata* .data* .bss*)
	}

	. = ALIGN(4K);
	__kernel_text_start = .;

	.text(0xFFFF800000000000 + __kernel_text_start) : AT(__kernel_text_start)
	{
		build/arch/x86_64/asm/higher_half.o(.text* .rodata* .data* .bss*)
    *(.rodata .rodata.*)
		*(.text .text.*)
	}

	. = ALIGN(4K) - 0xFFFF800000000000;
	__kernel_data_start = .;

	.data(0xFFFF800000000000 + __kernel_data_start) : AT(__kernel_data_start)
	{
		*(.data .data.*)
		. = ALIGN(4K);
	}

	. = ALIGN(4K) - 0xFFFF800000000000;
	__kernel_bss_start = .;

	.bss(0xFFFF800000000000 + __kernel_bss_start) : AT(__kernel_bss_start)
  {
			build/arch/x86_64/asm/long_mode_init.o(.stack)
      *(.bss .bss.*)
      . = ALIGN(4K);
  }

	__kernel_bss_end = .;
}

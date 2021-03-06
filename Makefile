arch ?= x86_64
kernel := build/kernel-$(arch).bin
iso := build/os-$(arch).iso

linker_script := src/arch/$(arch)/asm/linker.ld
grub_cfg := src/arch/$(arch)/asm/grub.cfg
assembly_source_files := $(wildcard src/arch/$(arch)/asm/*.asm)
assembly_object_files := $(patsubst src/arch/$(arch)/asm/%.asm, \
		build/arch/$(arch)/asm/%.o, $(assembly_source_files))

target ?= $(arch)-unknown-none-gnu
rust_os := target/$(target)/debug/libcykusz_rust64h.a

.PHONY: all clean run iso

all: $(kernel)

program/program : program/program.asm
	@nasm -f bin $< -o $@

clean:
	cargo clean
	find build -name *.o | xargs rm

purge: clean
	rm -r build

run: $(iso)
	qemu-system-x86_64 -drive format=raw,file=$(iso) -no-reboot -m 64 -smp cpus=4
debug: $(iso)
	qemu-system-x86_64 -drive format=raw,file=$(iso) -no-reboot -s -S
gdb:
	#@rust-os-gdb/bin/rust-gdb "build/kernel-x86_64.bin" -ex "target remote :1234"
	@rust-gdb "build/kernel-x86_64.bin" -ex "target remote :1234"
kdbg:
	@kdbg -r localhost:1234 "build/kernel-x86_64.bin"
bochs: $(iso)
	bochs -f bochsrc.txt -q

iso: $(iso)

$(iso): $(kernel) $(grub_cfg) program/program
	mkdir -p build/isofiles/boot/grub
	cp $(kernel) build/isofiles/boot/kernel.bin
	cp $(grub_cfg) build/isofiles/boot/grub
	cp program/program build/isofiles/boot/program
	grub-mkrescue -d /usr/lib/grub/i386-pc/ -o $(iso) build/isofiles 2> /dev/null

$(kernel): cargo $(rust_os) $(assembly_object_files) $(linker_script)
	ld -n --gc-sections -T $(linker_script) -o $(kernel) $(assembly_object_files) $(rust_os)

build:
	./update_core_nightly.sh ../rust

cargo:
	RUSTFLAGS="-L ./build -Z no-landing-pads"  xargo build --target $(target) --verbose

# compile assembly files
build/arch/$(arch)/asm/%.o: src/arch/$(arch)/asm/%.asm
	mkdir -p $(shell dirname $@)
	nasm -felf64 $< -o $@

#![feature(lang_items, asm,unique)]
#![feature(const_fn)]
#![feature(associated_type_defaults)]
#![no_std]
#![allow(dead_code)]

extern crate rlibc;
extern crate spin;
#[macro_use]
extern crate x86;

mod arch;

#[macro_use]
mod vga;
mod mboot2;
mod util;

fn phys_to_virt(addr: u64) -> u64 {
    addr + 0xFFFF_8000_0000_0000
}

#[no_mangle]
pub extern "C" fn rust_main(multiboot_addr: u64) {
    unsafe { x86::tlb::flush_all() };

    vga::clear_screen();

    let mboot_info = unsafe { mboot2::load(phys_to_virt(multiboot_addr)) };

    println!("Hello world, multiboot addr: 0x{:x} total_size: 0x{:x}!", multiboot_addr, mboot_info.size);

    for tag in mboot_info.tags() {
        println!("tag type: {}, size: {}", tag.typ, tag.size);
    }

    let mem = mboot_info.memory_map_tag().unwrap();

    println!("mem tag size: {}, ver: {}, es: {}", mem.size, mem.entry_ver, mem.entry_size);

    for e in mem.entries() {
        println!("Mem entry: base_addr: 0x{:x}  len: 0x{:x}, type: {}", e.base_addr, e.length, e.typ);
    }
}

#[cfg(not(test))]
#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[cfg(not(test))]
#[lang = "panic_fmt"]
extern "C" fn panic_fmt(fmt: core::fmt::Arguments, file: &str, line: u32) -> ! {
    println!("\n\nPANIC in {} at line {}:", file, line);
    println!("    {}", fmt);

    loop {}
}

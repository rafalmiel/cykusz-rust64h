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

#[no_mangle]
pub extern "C" fn rust_main() {
    unsafe { x86::tlb::flush_all() };

    vga::clear_screen();

    println!("Hello world!");

    unsafe {
        loop {
            asm!("hlt");
        }
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

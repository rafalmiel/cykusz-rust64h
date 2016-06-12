#![feature(lang_items, asm,unique)]
#![feature(const_fn)]
#![feature(associated_type_defaults)]
#![no_std]
#![allow(dead_code)]

extern crate rlibc;
extern crate spin;
#[macro_use]
extern crate x86;
#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate lazy_static;

#[macro_use]
mod vga;

pub mod arch;

mod mboot2;
mod util;

#[no_mangle]
pub extern "C" fn rust_main() {
    println!("In rust main!");

    loop {}
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

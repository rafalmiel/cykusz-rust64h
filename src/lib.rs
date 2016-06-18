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
pub mod mm;

mod mboot2;
mod util;

#[no_mangle]
pub extern "C" fn rust_main() {
    println!("In rust main!");

    for _ in 1..10 {
        let a = arch::mm::phys::allocate();
        let b = arch::mm::phys::allocate();
        let c = arch::mm::phys::allocate();

        if let Some(ref f) = a {
            println!("Allocated: 0x{:x}", f.address());
        }
        if let Some(ref f) = b {
            println!("Allocated: 0x{:x}", f.address());
        }
        if let Some(ref f) = c {
            println!("Allocated: 0x{:x}", f.address());
        }

        if let Some(f) = arch::mm::phys::allocate() {
            println!("Allocated: 0x{:x}", f.address());
        }
        if let Some(f) = arch::mm::phys::allocate() {
            println!("Allocated: 0x{:x}", f.address());
        }
        if let Some(f) = arch::mm::phys::allocate() {
            println!("Allocated: 0x{:x}", f.address());
        }
        if let Some(f) = arch::mm::phys::allocate() {
            println!("Allocated: 0x{:x}", f.address());
        }
    }

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

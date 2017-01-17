#![feature(lang_items, asm,unique)]
#![feature(const_fn)]
#![feature(associated_type_defaults)]
#![feature(pub_restricted)]
#![no_std]
#![allow(dead_code)]
#![feature(alloc, collections, step_by)]

extern crate hole_list_allocator;
extern crate alloc;
#[macro_use]
extern crate collections;

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
mod mm;

mod mboot2;
mod util;

#[no_mangle]
pub extern "C" fn notify_alloc(addr: *const u8) {
    println!("Calling from allocator! 0x{:x}", addr as usize);
}

#[no_mangle]
pub extern "C" fn notify_dealloc(addr: *const u8) {
    println!("Calling from deallocator! 0x{:x}", addr as usize);
}

#[no_mangle]
pub extern "C" fn request_more_mem(from: *const u8, size: usize) {
    let mut from = from as usize;
    let to = from + size;

    while from < to {
        arch::mm::virt::map(from);

        from += arch::mm::PAGE_SIZE;
    }

}

pub fn rust_main() {
    println!("In rust main!");

    // for _ in 1..1 {
    //     let a = arch::mm::phys::allocate();
    //     let b = arch::mm::phys::allocate();
    //     let c = arch::mm::phys::allocate();
    //
    //     if let Some(ref f) = a {
    //         println!("Allocated: 0x{:x}", f.address());
    //     }
    //     if let Some(ref f) = b {
    //         println!("Allocated: 0x{:x}", f.address());
    //     }
    //     if let Some(ref f) = c {
    //         println!("Allocated: 0x{:x}", f.address());
    //     }
    //
    //     if let Some(f) = arch::mm::phys::allocate() {
    //         println!("Allocated: 0x{:x}", f.address());
    //     }
    //     if let Some(f) = arch::mm::phys::allocate() {
    //         println!("Allocated: 0x{:x}", f.address());
    //     }
    //     if let Some(f) = arch::mm::phys::allocate() {
    //         println!("Allocated: 0x{:x}", f.address());
    //     }
    //     if let Some(f) = arch::mm::phys::allocate() {
    //         println!("Allocated: 0x{:x}", f.address());
    //     }
    // }

    {
        use alloc::boxed::Box;
        let mut heap_test = Box::new(42);

        Box::new(42);

        let a = vec![1,2,3];

        for i in a {
            print!("{} ", i);
        }

        Box::new(42);

        *heap_test = 33;
    }

    println!("Allocated on heap!");

    loop {}
}

#[cfg(not(test))]
#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[cfg(not(test))]
#[no_mangle]
#[lang = "panic_fmt"]
pub extern "C" fn panic_fmt(fmt: core::fmt::Arguments, file: &str, line: u32) -> ! {
    println!("\n\nPANIC in {} at line {}:", file, line);
    println!("    {}", fmt);

    loop {}
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn _Unwind_Resume() -> ! {
    loop {}
}

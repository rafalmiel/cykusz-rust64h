#![feature(lang_items, asm, unique)]
#![feature(const_fn)]
#![feature(associated_type_defaults)]
#![feature(naked_functions)]
#![no_std]
#![allow(dead_code)]
#![feature(iterator_step_by, inclusive_range_syntax)]
//alloc features
#![feature(alloc)]
#![feature(allocator_api)]
#![feature(global_allocator)]


#[macro_use]
extern crate alloc;

extern crate rlibc;
extern crate spin;
#[macro_use]
extern crate x86;
#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate lazy_static;

extern crate linked_list_allocator;

#[macro_use]
mod vga;

pub mod arch;
mod mm;

mod mboot2;
mod util;

//global allocator needs to be placed in root module? otherwise does not compile
#[global_allocator]
static mut HEAP: arch::mm::heap::LockedHeap = arch::mm::heap::LockedHeap::empty();

#[no_mangle]
pub fn notify_alloc(_addr: *const u8) {
    //println!("Calling from allocator! 0x{:x}", addr as usize);
}

#[no_mangle]
pub fn notify_dealloc(_addr: *const u8) {
    //println!("Calling from deallocator! 0x{:x}", addr as usize);
}

#[no_mangle]
pub fn log(l: &str) {
    print!("{}", l);
}

#[no_mangle]
pub fn logln(l: &str) {
    println!("{}", l);
}


#[no_mangle]
pub extern "C" fn logn(n: usize) {
    print!("0x{:x}", n);
}

extern "C" {
    fn switch_to_user();
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

        unsafe {
            asm!("xchg %bx, %bx");
        }


        let mut heap_test = Box::new(42);

        Box::new(42);

        let a = vec![1,2,3];

        for i in a {
            print!("{} ", i);
        }

        Box::new(42);

        *heap_test = 33;
    }

    println!("Allocated on heap! {}", 3);

    //loop{}

    vga::clear_screen();

    ::arch::int::fire_timer();

    unsafe {
        switch_to_user();
    }

    loop{}

    arch::task::init();
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

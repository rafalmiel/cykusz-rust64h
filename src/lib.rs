#![no_std]

#![feature(asm)]
#![feature(const_fn)]
#![feature(iterator_step_by)]
#![feature(lang_items)]
#![feature(step_trait)]
#![feature(unique)]
#![feature(alloc)]
#![feature(allocator_api)]
#![feature(global_allocator)]
#![feature(const_unsafe_cell_new)]
#![feature(const_ptr_null_mut)]

#![allow(dead_code)]

extern crate rlibc;
extern crate spin;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate x86;
#[macro_use]
extern crate bitflags;
extern crate linked_list_allocator;

extern crate alloc;

#[macro_use]
mod kernel;

#[macro_use]
pub mod newtype;

#[macro_use]
pub mod arch;

mod drivers;
mod util;

#[global_allocator]
static mut HEAP: kernel::mm::heap::LockedHeap = kernel::mm::heap::LockedHeap::empty();

fn task() {
    for _ in 0..10000 {

    }

    //println!("[ TASK ] About to finish task 1")
}

fn task2() {
    for _ in 0..10000 {
        
    }

    //println!("[ TASK ] About to finish task 2")
}

pub fn rust_main() {

    kernel::mm::init();

    kernel::sched::init();

    println!("[ OK ] Initialised scheduler");

    kernel::sched::create_kernel_task(task);
    kernel::sched::create_kernel_task(task2);

    kernel::sched::create_user_task(
        unsafe { ::core::mem::transmute::<usize, fn() -> ()>(0x400000) }, 
        0x600000, 4096);

    kernel::int::fire_timer();
    kernel::int::enable_interrupts();
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

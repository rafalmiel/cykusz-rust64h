#[macro_use]
pub mod print;

pub mod mm;
pub mod cpuio;
pub mod output;
pub mod acpi;
pub mod pic;
pub mod int;
pub mod sync;
#[macro_use]
pub mod task;
pub mod gdt;
pub mod user;

use kernel::mm::*;
use drivers::multiboot2;

#[no_mangle]
pub extern "C" fn x86_64_rust_main(multiboot_addr: PhysAddr, stack_top: VirtAddr, gdt64: VirtAddr) 
{
    output::clear();

    println!("[ OK ] Initialised long mode");

    let mboot = unsafe { &multiboot2::load(multiboot_addr.to_mapped()) };

    mm::init(mboot);

    println!("[ OK ] Initialised memory");

    int::init();

    gdt::init(stack_top, gdt64);

    println!("[ OK ] Initialised TSS");

    user::init(mboot);

    println!("[ OK ] Initialised user program");

    ::rust_main();
}
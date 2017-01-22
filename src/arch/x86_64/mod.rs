pub mod cpuio;
pub mod mm;
pub mod int;
pub mod apic;

use vga;
use mboot2;

use arch::mm::PhysAddr;

use self::mm::phys_to_physmap;

#[no_mangle]
pub extern "C" fn x86_64_rust_main(multiboot_addr: PhysAddr) {
    vga::clear_screen();

    println!("Loading mboot at addr 0x{:x}", phys_to_physmap(multiboot_addr));

    let mboot_info = unsafe { mboot2::load(phys_to_physmap(multiboot_addr)) };

    mm::init(&mboot_info);

    int::init();
    int::disable_pic();

    apic::init();

    unsafe {int::idt::test()};

    //apic::fire_timer();

    ::rust_main();
}

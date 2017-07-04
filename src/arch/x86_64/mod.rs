pub mod cpuio;
pub mod mm;
pub mod int;
pub mod apic;
pub mod pic;
pub mod task;
pub mod sync;

use vga;
use mboot2;

use arch::mm::PhysAddr;

use self::mm::phys_to_physmap;

#[no_mangle]
pub extern "C" fn x86_64_rust_main(multiboot_addr: PhysAddr, stack_top: PhysAddr) {
    unsafe {
        asm!("xchg %bx, %bx");
    }
    vga::clear_screen();

    println!("Loading mboot at addr 0x{:x}", phys_to_physmap(multiboot_addr));

    let mboot_info = unsafe { mboot2::load(phys_to_physmap(multiboot_addr)) };

    mm::init(&mboot_info);

    int::init();

    if let Some(mtag) = mboot_info.modules_tags().next() {
        //let string = unsafe { ::core::str::from_utf8_unchecked(mtag.name) };

        println!("Module start: 0x{:x}", mtag.mod_start);
        println!("Module end  : 0x{:x}", mtag.mod_end);

        unsafe {
            asm!("xchg %bx, %bx");
        }

        // Copy user program to user mapping!
        mm::virt::map_flags(0x400000, mm::virt::entry::USER | mm::virt::entry::WRITABLE);

        for (i, ptr) in (mtag.mod_start..mtag.mod_end).enumerate() {
            unsafe {
                println!("0x{:x} {}, 0x{:x}", ptr, i, *((phys_to_physmap(mtag.mod_start as usize + i)) as *mut u8));
                *((0x400000 as usize + i) as *mut u8) = *((phys_to_physmap(mtag.mod_start as usize + i)) as *mut u8);
                println!("0x{:x} {}, 0x{:x}", ptr, i, *((0x400000 as usize + i) as *mut u8));
            }
        }

        unsafe {
            asm!("xchg %bx, %bx");
        }
    }

    unsafe {
        int::idt::test() ;
    }

    //int::fire_timer();

    ::rust_main();
}

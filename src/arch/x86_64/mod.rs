pub mod cpuio;
pub mod mm;

use vga;
use mboot2;

pub const VIRT : u64 = 0xFFFFFF0000000000;
pub const PHYSMAP : u64 = 0xFFFF800000000000;

pub fn phys_to_virt(addr: u64) -> u64 {
    if addr < VIRT {
        addr + VIRT
    } else {
        addr
    }
}

pub fn virt_to_phys(addr: u64) -> u64 {
    if addr >= VIRT {
        addr - VIRT
    } else {
        addr
    }
}

pub fn phys_to_physmap(addr: u64) -> u64 {
    if addr < PHYSMAP {
        addr + PHYSMAP
    } else {
        addr
    }
}

pub fn physmap_to_phys(addr: u64) -> u64 {
    if addr >= PHYSMAP {
        addr - PHYSMAP
    } else {
        addr
    }
}

#[no_mangle]
pub extern "C" fn x86_64_rust_main(multiboot_addr: u64) {
    vga::clear_screen();

    let mboot_info = unsafe { mboot2::load(phys_to_virt(multiboot_addr)) };

    mm::init(& mboot_info);

    ::rust_main();
}

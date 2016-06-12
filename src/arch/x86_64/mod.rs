pub mod cpuio;

use x86;
use vga;
use mboot2;

pub const VIRT : u64 = 0xFFFFFF0000000000;

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

#[no_mangle]
pub extern "C" fn x86_64_rust_main(multiboot_addr: u64) {
    unsafe { x86::tlb::flush_all() };

    vga::clear_screen();

    let mboot_info = unsafe { mboot2::load(phys_to_virt(multiboot_addr)) };

    for tag in mboot_info.tags() {
        println!("tag type: {}, size: {}", tag.typ, tag.size);
    }

    let mem = mboot_info.memory_map_tag().unwrap();

    for e in mem.entries() {
        println!("Mem entry: base_addr: 0x{:x}  len: 0x{:x}, type: {}", e.base_addr, e.length, e.typ);
    }

    let elf = mboot_info.elf_tag().unwrap();

    for s in elf.sections() {
        println!("Elf typ: {}, flags: 0x{:x}, addr: 0x{:x}, size: 0x{:x}", s.typ, s.flags, s.addr, s.size);
    }

    println!("Kernel start: 0x{:x}", virt_to_phys(mboot_info.kernel_start_addr()));
    println!("Kernel end  : 0x{:x}", virt_to_phys(mboot_info.kernel_end_addr()));

    println!("mboot2 start: 0x{:x}", virt_to_phys(multiboot_addr));
    println!("mboot2 end  : 0x{:x}", virt_to_phys(multiboot_addr + mboot_info.size as u64));

    ::rust_main();
}

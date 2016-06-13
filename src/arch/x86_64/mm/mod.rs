pub mod phys;

use mboot2;
use arch::virt_to_phys;

pub type PhysAddr = u64;
pub type VirtAddr = u64;
pub const PAGE_SIZE: PhysAddr = 4096;

pub fn init(mboot_info: & mboot2::Info) {
    let mem = mboot_info.memory_map_tag().unwrap();
    let multiboot_addr = mboot_info as *const _ as PhysAddr;

    for tag in mboot_info.tags() {
        println!("tag type: {}, size: {}", tag.typ, tag.size);
    }

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

    phys::init(mem.entries(),
               virt_to_phys(mboot_info.kernel_start_addr()),
               virt_to_phys(mboot_info.kernel_end_addr()),
               virt_to_phys(multiboot_addr),
               virt_to_phys(multiboot_addr + mboot_info.size as u64));
}

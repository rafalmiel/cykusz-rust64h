pub mod entry;
mod page;
mod table;

use arch::mm::*;
use arch::mm::phys::Frame;
use self::table::Table;

const PAGE_SIZE: usize = 4096;

fn p4_table_addr() -> MappedAddr {
    unsafe {
        phys_to_physmap(::x86::shared::control_regs::cr3() as PhysAddr)
    }
}

pub fn map_flags(virt: VirtAddr, flags: entry::Entry) {
    let page = page::Page::new(virt);
    let p4_addr = physmap_to_phys(p4_table_addr());

    Table::new_at_frame_mut(
        &Frame::new(p4_addr)
    )
        .alloc_next_level_flags(page.p4_index(), flags)
        .alloc_next_level_flags(page.p3_index(), flags)
        .alloc_next_level_flags(page.p2_index(), flags)
        .alloc_set_flags(page.p1_index(), flags);

    unsafe {
        ::x86::shared::tlb::flush(p4_addr);
    }
}

pub fn map(virt: VirtAddr) {
    map_flags(virt, entry::WRITABLE);
}

pub fn unmap(virt: VirtAddr) {
    let page = page::Page::new(virt);
    let p4_addr = physmap_to_phys(p4_table_addr());

    if let Some(p1) = Table::new_at_frame_mut(
        &Frame::new(p4_addr)
    )
        .next_level_mut(page.p4_index())
        .and_then(|t| t.next_level_mut(page.p3_index())
        .and_then(|t| t.next_level_mut(page.p2_index()))) {

        p1.unmap(page.p1_index());

        unsafe {
            ::x86::shared::tlb::flush_all();
        };
    } else {
        println!("ERROR: virt addr 0x{:x} cannot be unmapped", virt);
    }
}

#[allow(unused)]
pub fn map_to(virt: VirtAddr, phys: PhysAddr) {
    let page = page::Page::new(virt);
    let p4_addr = physmap_to_phys(p4_table_addr());

    Table::new_at_frame_mut(
        &Frame::new(p4_addr)
    )
        .alloc_next_level(page.p4_index())
        .alloc_next_level(page.p3_index())
        .alloc_next_level(page.p2_index())
        .set(page.p1_index(), &Frame::new(phys));

    unsafe {
        ::x86::shared::tlb::flush(p4_addr);
    }
}

#[allow(unused)]
pub fn map_to_1gb(virt: VirtAddr, phys: PhysAddr) {
    let page = page::Page::new(virt);
    let p4_addr = physmap_to_phys(p4_table_addr());

    Table::new_at_frame_mut(
        &Frame::new(p4_addr)
    )
        .alloc_next_level(page.p4_index())
        .set_hugepage(page.p3_index(), &Frame::new(phys));

    unsafe {
        ::x86::shared::tlb::flush(p4_addr);
    }
}

fn enable_nxe_bit() {
    use x86::shared::msr::{IA32_EFER, rdmsr, wrmsr};

    let nxe_bit = 1 << 11;
    unsafe {
        let efer = rdmsr(IA32_EFER);
        wrmsr(IA32_EFER, efer | nxe_bit);
    }
}

fn enable_write_protect_bit() {
    use x86::shared::control_regs::{cr0, cr0_write, CR0_WRITE_PROTECT};

    unsafe { cr0_write(cr0() | CR0_WRITE_PROTECT) };
}

fn remap(mboot_info: &mboot2::Info) {
    let frame = ::arch::mm::phys::allocate().expect("Out of mem!");
    let table = Table::new_at_frame_mut(&frame);

    table.clear();

    for elf in mboot_info.elf_tag().unwrap().sections() {

        let s = ::util::align_down(virt_to_phys(elf.address() as VirtAddr) as usize, PAGE_SIZE);
        let e = ::util::align_up(virt_to_phys(elf.end_address() as VirtAddr) as usize, PAGE_SIZE);

        let mut flags = entry::Entry::empty();

        use ::mboot2::elf::ElfSectionFlags;

        if (elf.flags as usize & ElfSectionFlags::Allocated as usize) == ElfSectionFlags::Allocated as usize {
            flags.insert(entry::PRESENT);
        }
        if (elf.flags as usize & ElfSectionFlags::Writable as usize) == ElfSectionFlags::Writable as usize {
            flags.insert(entry::WRITABLE);
        }
        if (elf.flags as usize & ElfSectionFlags::Executable as usize) == 0 {
            flags.insert(entry::NO_EXECUTE);
        }

        println!("from 0x{:x} to 0x{:x} with flags 0x{:x}", s, e, flags.raw());

        for addr in (s..e).step_by(PAGE_SIZE) {
            let p = page::Page::new(phys_to_virt(addr));
            let f = Frame::new(addr);

            table.alloc_next_level(p.p4_index())
                 .alloc_next_level(p.p3_index())
                 .alloc_next_level(p.p2_index())
                 .set_flags(p.p1_index(), &f, flags);
        }
    }

    for idx in 0..512 {
        let physmap_page = page::Page::new(0xffff800000000000 + idx * 1024*1024*1024);

        table.alloc_next_level(physmap_page.p4_index())
             .set_hugepage(physmap_page.p3_index(), &Frame::new(idx * 1024*1024*1024));
    }

    unsafe {
        println!("Writing cr3 with value 0x{:x}", frame.address());

        enable_nxe_bit();
        enable_write_protect_bit();

        ::x86::shared::control_regs::cr3_write(frame.address() as usize);

        ::x86::shared::tlb::flush_all();
    }
}

pub fn init(mboot_info: &mboot2::Info) {
    remap(&mboot_info);
}

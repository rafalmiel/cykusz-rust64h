pub mod entry;
mod page;
mod table;

use arch::mm::*;
use arch::mm::phys::Frame;
use self::table::*;

const PAGE_SIZE: usize = 4096;

fn p4_table_addr() -> MappedAddr {
    unsafe {
        phys_to_physmap(::x86::shared::control_regs::cr3() as PhysAddr)
    }
}

fn table_map_flags(table: &mut P4Table, addr: VirtAddr, flags: entry::Entry) {
    let page = page::Page::new(addr);

    table.alloc_next_level(page.p4_index())
         .alloc_next_level(page.p3_index())
         .alloc_next_level(page.p2_index())
         .alloc_set_flags(page.p1_index(), flags);
}

fn table_map_frame_flags(table: &mut P4Table, virt: VirtAddr, phys: PhysAddr, flags: entry::Entry) {
    let page = page::Page::new(virt);

    table.alloc_next_level(page.p4_index())
         .alloc_next_level(page.p3_index())
         .alloc_next_level(page.p2_index())
         .set_flags(page.p1_index(), &Frame::new(phys), flags);
}

fn table_map_frame(table: &mut P4Table, virt: VirtAddr, phys: PhysAddr) {
    let page = page::Page::new(virt);

    table.alloc_next_level(page.p4_index())
         .alloc_next_level(page.p3_index())
         .alloc_next_level(page.p2_index())
         .set(page.p1_index(), &Frame::new(phys));
}

fn table_map_hugepage_frame(table: &mut P4Table, virt: VirtAddr, phys: PhysAddr) {
    let page = page::Page::new(virt);

    table.alloc_next_level(page.p4_index())
          .alloc_next_level(page.p3_index())
          .set_hugepage(page.p2_index(), &Frame::new(phys));
}

pub fn map_flags(virt: VirtAddr, flags: entry::Entry) {
    table_map_flags(
        P4Table::new_mut(&Frame::new(p4_table_addr())),
        virt,
        flags
    );

    unsafe {
        ::x86::shared::tlb::flush(virt);
    }
}

pub fn map(virt: VirtAddr) {
    map_flags(virt, entry::WRITABLE);
}

pub fn unmap(virt: VirtAddr) {
    let page = page::Page::new(virt);

    if let Some(p1) = Table::<Level4>::new_mut(
        &Frame::new(p4_table_addr())
    )
        .next_level_mut(page.p4_index())
        .and_then(|t| t.next_level_mut(page.p3_index())
        .and_then(|t| t.next_level_mut(page.p2_index()))) {

        p1.unmap(page.p1_index());

        unsafe {
            ::x86::shared::tlb::flush(virt);
        };
    } else {
        println!("ERROR: virt addr 0x{:x} cannot be unmapped", virt);
    }
}

#[allow(unused)]
pub fn map_to(virt: VirtAddr, phys: PhysAddr) {

    table_map_frame(
        P4Table::new_mut(&Frame::new(p4_table_addr())),
        virt,
        phys
    );

    unsafe {
        ::x86::shared::tlb::flush(virt);
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
    let table = P4Table::new_mut(&frame);

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
            table_map_frame_flags(table, phys_to_virt(addr), addr, flags);
        }
    }

    // Set physmap from previous mapping
    let orig = P4Table::new_mut(&Frame::new(p4_table_addr()));
    table.set_entry(256, orig.entry_at(256));

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

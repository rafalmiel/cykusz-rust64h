pub mod entry;
mod page;
mod table;

use kernel::mm::virt;
use kernel::mm::*;

use arch::mm::PAGE_SIZE;
use arch::mm::phys::Frame;
use self::table::*;

fn p4_table_addr() -> PhysAddr {
    unsafe {
        PhysAddr(::x86::shared::control_regs::cr3())
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

pub fn flush(virt: VirtAddr) {
    unsafe {
        ::x86::shared::tlb::flush(virt.0);
    }
}

pub fn flush_all() {
    unsafe {
        ::x86::shared::tlb::flush_all();
    }
}

pub fn map_flags(virt: VirtAddr, flags: virt::PageFlags) {
    P4Table::new_mut(
        &Frame::new(p4_table_addr())
    ).map_flags(virt, flags);

    flush(virt);
}

pub fn map(virt: VirtAddr) {
    map_flags(virt, virt::WRITABLE);
}

pub fn unmap(virt: VirtAddr) {
    P4Table::new_mut(
        &Frame::new(p4_table_addr())
    ).unmap(virt);

    flush(virt);
}

#[allow(unused)]
pub fn map_to(virt: VirtAddr, phys: PhysAddr) {
    P4Table::new_mut(&Frame::new(p4_table_addr())).map_to(virt, phys);

    flush(virt);
}

pub unsafe fn activate_table(table: &P4Table) {
    ::x86::shared::control_regs::cr3_write(table.phys_addr().0);
}

fn remap(mboot_info: &::drivers::multiboot2::Info) {
    let frame = ::arch::mm::phys::allocate().expect("Out of mem!");
    let table = P4Table::new_mut(&frame);

    table.clear();

    for elf in mboot_info.elf_tag().unwrap().sections() {

        let s = elf.address().align_down(PAGE_SIZE);
        let e = elf.end_address().align_up(PAGE_SIZE);

        let mut flags = virt::PageFlags::empty();

        use ::drivers::multiboot2::elf::ElfSectionFlags;

        if (elf.flags as usize & ElfSectionFlags::Allocated as usize) == 0 as usize {
            continue;
        }

        if (elf.flags as usize & ElfSectionFlags::Writable as usize) == ElfSectionFlags::Writable as usize {
            flags.insert(virt::WRITABLE);
        }
        if (elf.flags as usize & ElfSectionFlags::Executable as usize) == 0 {
            flags.insert(virt::NO_EXECUTE);
        }

        for addr in (s..e).step_by(PAGE_SIZE) {
            table.map_to_flags(addr, addr.to_phys(), flags);
        }
    }

    // Set physmap from previous mapping
    let orig = P4Table::new_mut(&Frame::new(p4_table_addr()));
    table.set_entry(256, orig.entry_at(256));

    unsafe {
        activate_table(&table);
    }
}

pub fn init(mboot_info: &::drivers::multiboot2::Info) {
    enable_nxe_bit();
    enable_write_protect_bit();

    println!("[ OK ] Enabled nxe and write protect");

    remap(&mboot_info);

    println!("[ OK ] Remapped kernel code");
}

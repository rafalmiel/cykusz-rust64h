mod entry;
mod page;
mod table;

use arch::mm::*;
use arch::mm::phys::Frame;
use self::table::Table;

const PAGE_SIZE: usize = 4096;

fn p4_table_addr() -> MappedAddr {
    unsafe {
        phys_to_physmap(::x86::controlregs::cr3() as PhysAddr)
    }
}

pub fn map(virt: VirtAddr) {
    let page = page::Page::new(virt);
    let p4_addr = physmap_to_phys(p4_table_addr());

    Table::new_at_frame_mut(
        &Frame::new(p4_addr)
    )
        .alloc_next_level(page.p4_index())
        .alloc_next_level(page.p3_index())
        .alloc_next_level(page.p2_index())
        .alloc(page.p1_index());

    unsafe {
        ::x86::tlb::flush(p4_addr);
    }
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
            ::x86::tlb::flush_all();
        };
    } else {
        println!("ERROR: virt addr 0x{:x} cannot be unmapped", virt);
    }
}

#[allow(unused)]
pub fn map_to(virt: VirtAddr, phys: PhysAddr) {

}

pub fn init() {
    println!("p4 addr: 0x{:x}", p4_table_addr());

    map(0x325000);

    unsafe {
        asm!("xchg %bx, %bx");
        *(0x325000 as *mut u64) = 33;
    };
}

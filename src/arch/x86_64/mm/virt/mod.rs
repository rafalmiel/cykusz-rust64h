mod entry;
mod page;
mod table;

use arch::mm::*;
use arch::mm::phys::Frame;
use self::table::Table;

const PAGE_SIZE: usize = 4096;

fn p4_table_addr() -> PhysAddr {
    unsafe {
        phys_to_physmap(::x86::controlregs::cr3() as PhysAddr)
    }
}

pub fn map(virt: VirtAddr) {
    let page = page::Page::new(virt);
    let p4_addr = p4_table_addr();

    Table::new_from_frame(
        Frame::new(p4_addr)
    )
        .next_level(page.p4_index())
        .next_level(page.p3_index())
        .next_level(page.p2_index())
        .next_level(page.p1_index());

    unsafe {
        ::x86::tlb::flush(p4_addr);
    }
}

#[allow(unused)]
pub fn unmap(virt: VirtAddr) {

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
    }
}

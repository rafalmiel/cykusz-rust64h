mod entry;
mod page;

use arch::mm::*;

const PAGE_SIZE: usize = 4096;

fn p4_table_addr() -> PhysAddr {
    unsafe {
        phys_to_physmap(::x86::controlregs::cr3() as PhysAddr)
    }
}

pub fn map(virt: VirtAddr) {
    let page = page::Page::new(virt);

    let mut e4 = unsafe {
        entry::Entry::from_addr(p4_table_addr() + page.p4_index() * 8)
    };

    println!("0x{:x}", e4.raw());

    if e4.is_unused() {
        let frame = phys::allocate().expect("Out of memory!");

        println!("MAP: Allocated frame addr 0x{:x}", frame.address());

        e4.set_frame(frame);
    }

    e4.set_flags(entry::PRESENT | entry::WRITABLE);
    unsafe { *(phys_to_physmap(p4_table_addr() + page.p4_index() * 8) as *mut PhysAddr) = e4.raw(); }

    unsafe {
        ::x86::tlb::flush_all();
    }

    println!("Getting {} e4 at 0x{:x} -> 0x{:x}", page.p4_index(), p4_table_addr() + page.p4_index() * 8, e4.raw());

    println!("e4 done");

    let mut e3 = unsafe {
        entry::Entry::from_addr(phys_to_physmap(e4.address()) + page.p3_index() * 8)
    };

    println!("0x{:x}", e3.raw());

    if e3.is_unused() {
        let frame = phys::allocate().expect("Out of memory!");

        println!("MAP: Allocated frame addr 0x{:x}", frame.address());

        e3.set_frame(frame);
    }

    e3.set_flags(entry::PRESENT | entry::WRITABLE);
    unsafe { *(phys_to_physmap(e4.address() + page.p3_index() * 8) as *mut PhysAddr) = e3.raw(); }

    unsafe {
        ::x86::tlb::flush_all();
    }

    println!("Getting {} e3 at 0x{:x} -> 0x{:x}", page.p3_index(), phys_to_physmap(e4.address()) + page.p3_index() * 8, e3.raw());

    println!("e3 done");

    let mut e2 = unsafe {
        entry::Entry::from_addr(phys_to_physmap(e3.address()) + page.p2_index() * 8)
    };

    println!("0x{:x}", e2.raw());

    if e2.is_unused() {
        let frame = phys::allocate().expect("Out of memory!");

        println!("MAP: Allocated frame addr 0x{:x}", frame.address());

        e2.set_frame(frame);
    }

    e2.set_flags(entry::PRESENT | entry::WRITABLE);
    unsafe { *(phys_to_physmap(e3.address() + page.p2_index() * 8) as *mut PhysAddr) = e2.raw(); }

    unsafe {
        ::x86::tlb::flush_all();
    }

    println!("Getting {} e2 at 0x{:x} -> 0x{:x}", page.p2_index(), phys_to_physmap(e3.address()) + page.p2_index() * 8, e2.raw());

    println!("e2 done");

    println!("Getting e1 at 0x{:x}", phys_to_physmap(e2.address()) + page.p1_index() * 8);

    let mut e1 = unsafe {
        entry::Entry::from_addr(phys_to_physmap(e2.address()) + page.p1_index() * 8)
    };

    println!("0x{:x}", e1.raw());

    if e1.is_unused() {
        let frame = phys::allocate().expect("Out of memory!");

        println!("MAP: Allocated frame addr 0x{:x}", frame.address());

        e1.set_frame(frame);
    }

    e1.set_flags(entry::PRESENT | entry::WRITABLE);
    unsafe { *(phys_to_physmap(e2.address() + page.p1_index() * 8) as *mut PhysAddr) = e1.raw(); }

    unsafe {
        ::x86::tlb::flush_all();
    }

    println!("Getting {} e1 at 0x{:x} -> 0x{:x}", page.p1_index(), phys_to_physmap(e2.address()) + page.p1_index() * 8, e1.raw());

    println!("e1 done");
}

pub fn init() {
    println!("p4 addr: 0x{:x}", p4_table_addr());

    let entry = unsafe {
        entry::Entry::from_addr(p4_table_addr() + 256 * 8)
    };

    map(0x5000);

    unsafe {asm!("xchg %bx, %bx")};

    unsafe {
        //loop{}
        *(0x5000 as *mut u64) = 33;
    }

    println!("Sec addr: 0x{:x}", entry.address());
}

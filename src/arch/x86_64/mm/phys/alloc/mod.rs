pub mod frame;

use spin::Mutex;

use arch::mm::PhysAddr;
use arch::mm::MappedAddr;
use arch::mm::phys_to_physmap;
use mboot2::memory::MemoryIter;
use mm;

use super::iter::PhysMemIterator;

const LIST_ADDR_INVALID: PhysAddr = 0xFFFF_FFFF_FFFF_FFFF;

const fn is_list_addr_valid(addr: PhysAddr) -> bool {
    addr != LIST_ADDR_INVALID
}

struct PhysAllocatorList {
    head:           PhysAddr,
}

static PHYS_LIST: Mutex<PhysAllocatorList> = Mutex::new(
    PhysAllocatorList {
        head: LIST_ADDR_INVALID,
    }
);

pub fn allocate() -> Option<mm::Frame> {
    let mut list = PHYS_LIST.lock();

    if is_list_addr_valid(list.head) {
        let ret = list.head;

        list.head = unsafe {
            *(phys_to_physmap(list.head) as *const PhysAddr)
        };

        unsafe {
            for i in 0..(4096 / 8) {
                *(phys_to_physmap(ret + i*8) as *mut PhysAddr) = 0;
            }
        }

        return Some(mm::Frame::new(ret));
    }

    None
}

fn deallocate(frame: &mm::Frame) {
    println!("Deallocating 0x{:x}", frame.address());
    let mut list = PHYS_LIST.lock();

    unsafe {
        *(frame.address_mapped() as *mut MappedAddr) = list.head;
    }

    list.head = frame.address();
}

pub fn init(mm_iter:        MemoryIter,
            kern_start:     PhysAddr,
            kern_end:       PhysAddr,
            mboot_start:    PhysAddr,
            mboot_end:      PhysAddr) {

    let iter = PhysMemIterator::new(mm_iter, kern_start, kern_end, mboot_start, mboot_end);

    println!("Initialising physical memory 0x{:x} 0x{:x} 0x{:x} 0x{:x}",
             kern_start, kern_end, mboot_start, mboot_end);

    let mut head: Option<PhysAddr> = None;
    let mut tail: Option<PhysAddr> = None;
    let mut max_cnt: u64 = 0;

    for el in iter {
        if let Some(p) = tail {
            let physmap = phys_to_physmap(p);

            let addr = physmap as *mut MappedAddr;

            unsafe {
                *addr = el;
            }
        }

        if head.is_none() {
            head = Some(el);
        }

        max_cnt += 1;
        tail = Some(el);
    }

    if let Some(p) = tail {
        let addr = phys_to_physmap(p) as *mut MappedAddr;

        unsafe {
            *addr = LIST_ADDR_INVALID;
        }

    }

    let mut l = PHYS_LIST.lock();

    if let Some(f) = head {
        println!("Init head to 0x{:x}", f);
        l.head = f;
    }

    println!("Physical memory initialisation complete after {} iterations", max_cnt);
}

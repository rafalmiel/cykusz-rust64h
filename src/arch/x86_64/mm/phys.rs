use spin::Mutex;

use arch::mm::PAGE_SIZE;
use arch::mm::PhysAddr;
use arch::phys_to_physmap;
use mboot2::memory::MemoryIter;
use mm;


impl Drop for ::mm::frame::Frame {
    fn drop(&mut self) {
        deallocate(self);
    }
}

const LIST_ADDR_INVALID: PhysAddr = 0xFFFF_FFFF_FFFF_FFFF;

const fn is_list_addr_valid(addr: PhysAddr) -> bool {
    addr != LIST_ADDR_INVALID
}

struct PhysAllocatorList {
    head:           PhysAddr,
    tail:           PhysAddr
}

static PHYS_LIST: Mutex<PhysAllocatorList> = Mutex::new(
    PhysAllocatorList {
        head: LIST_ADDR_INVALID,
        tail: LIST_ADDR_INVALID
    }
);

struct PhysMemIterator {
    current:        PhysAddr,
    mm_iter:        MemoryIter,
    mm_start:       PhysAddr,
    mm_end:         PhysAddr,
    kern_start:     PhysAddr,
    kern_end:       PhysAddr,
    mboot_start:    PhysAddr,
    mboot_end:      PhysAddr
}

fn not_contains(saddr: PhysAddr, start: PhysAddr, end: PhysAddr) -> bool {
    let eaddr = saddr + PAGE_SIZE;

    (saddr < start && eaddr < start) || (saddr >= end && eaddr >= end)
}

impl PhysMemIterator {
    pub fn new(mut mm_iter:     MemoryIter,
               kern_start:      PhysAddr,
               kern_end:        PhysAddr,
               mboot_start:     PhysAddr,
               mboot_end:       PhysAddr) -> PhysMemIterator {
        let ent = mm_iter.next().expect("Memory iterator needs at least one value");

        PhysMemIterator {
            current:        ent.base_addr,
            mm_iter:        mm_iter,
            mm_start:       ent.base_addr,
            mm_end:         ent.base_addr + ent.length,
            kern_start:     kern_start,
            kern_end:       kern_end,
            mboot_start:    mboot_start,
            mboot_end:      mboot_end
        }
    }

    fn is_valid(&self, addr: PhysAddr) -> bool {
        not_contains(addr, self.kern_start, self.kern_end)
        && not_contains(addr, self.mboot_start, self.mboot_end)
    }
}

impl Iterator for PhysMemIterator {
    type Item = PhysAddr;

    fn next(&mut self) -> Option<PhysAddr> {
        let c = self.current;

        if c >= self.mm_end {
            if let Some(e) = self.mm_iter.next() {
                self.mm_start = e.base_addr;
                self.mm_end = e.base_addr + e.length;
                self.current = self.mm_start;
                return self.next();
            } else {
                return None;
            }
        }

        if !self.is_valid(c) {
            self.current = self.current + PAGE_SIZE;
            return self.next();
        }

        self.current = self.current + PAGE_SIZE;

        Some(c)
    }
}

pub fn allocate() -> Option<mm::frame::Frame> {
    let mut list = PHYS_LIST.lock();

    if is_list_addr_valid(list.head) {
        let ret = list.head;

        if list.head != list.tail {
            let next_addr = unsafe {
                *(phys_to_physmap(list.head) as *const PhysAddr)
            };
            list.head = next_addr;
        } else {
            list.head = LIST_ADDR_INVALID;
            list.tail = LIST_ADDR_INVALID;
        }

        return Some(::mm::frame::Frame::new(ret));
    }

    None
}

fn deallocate(frame: &mm::frame::Frame) {
    let mut list = PHYS_LIST.lock();

    if is_list_addr_valid(list.tail) {
        unsafe {
            *(phys_to_physmap(list.tail) as *mut PhysAddr) = frame.address();
        }
        list.tail = frame.address();
    } else {
        list.head = frame.address();
        list.tail = frame.address();
    }
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

            let addr = physmap as *mut PhysAddr;

            unsafe {
                *addr = el;
            }
        }

        if head.is_none() {
            head = Some(el);
            println!("head is 0x{:x}", el);
        }

        max_cnt += 1;
        tail = Some(el);
    }

    if let Some(p) = tail {
        let addr = phys_to_physmap(p) as *mut PhysAddr;

        unsafe {
            *addr = LIST_ADDR_INVALID;
            println!("Value at 0x{:x} is 0x{:x}", addr as PhysAddr, *addr);
        }

    }

    let mut l = PHYS_LIST.lock();

    if let Some(f) = head {
        println!("Init head to 0x{:x}", f);
        l.head = f;
    }

    if let Some(f) = tail {
        println!("Init tail to 0x{:x}", f);
        l.tail = f;
    }

    println!("Physical memory initialisation complete after {} iterations", max_cnt);
}

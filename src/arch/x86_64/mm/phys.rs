use spin::Mutex;

use arch::mm::PAGE_SIZE;
use arch::mm::PhysAddr;
use arch::phys_to_physmap;
use mboot2::memory::MemoryIter;

static PHYS_HEAD: Mutex<Option<Frame>> = Mutex::new(None);
static PHYS_TAIL: Mutex<Option<Frame>> = Mutex::new(None);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Frame {
    number:        u64
}

impl Frame {
    pub fn new(address: PhysAddr) -> Frame {
        Frame {
            number: address / PAGE_SIZE
        }
    }

    pub fn address(&self) -> PhysAddr {
        self.number * PAGE_SIZE
    }

    pub fn end_address(&self) -> PhysAddr {
        self.number * PAGE_SIZE + PAGE_SIZE
    }

    pub fn number(&self) -> u64 {
        self.number
    }

    pub fn next(&self) -> Frame {
        Frame {
            number: self.number + 1
        }
    }

    fn clone(&self) -> Frame {
        Frame {
            number: self.number
        }
    }

    fn not_contains(&self, start: PhysAddr, end: PhysAddr) -> bool {
        let saddr = self.address();
        let eaddr = self.end_address();

        (saddr < start && eaddr < start) || (saddr >= end && eaddr >= end)
    }
}

struct PhysMemIterator {
    current:        Frame,
    mm_iter:        MemoryIter,
    mm_start:       PhysAddr,
    mm_end:         PhysAddr,
    kern_start:     PhysAddr,
    kern_end:       PhysAddr,
    mboot_start:    PhysAddr,
    mboot_end:      PhysAddr
}

impl PhysMemIterator {
    pub fn new(mut mm_iter:     MemoryIter,
               kern_start:      PhysAddr,
               kern_end:        PhysAddr,
               mboot_start:     PhysAddr,
               mboot_end:       PhysAddr) -> PhysMemIterator {
        let ent = mm_iter.next().expect("Memory iterator needs at least one value");

        PhysMemIterator {
            current:        Frame::new(ent.base_addr),
            mm_iter:        mm_iter,
            mm_start:       ent.base_addr,
            mm_end:         ent.base_addr + ent.length,
            kern_start:     kern_start,
            kern_end:       kern_end,
            mboot_start:    mboot_start,
            mboot_end:      mboot_end
        }
    }

    fn is_valid(&self, frame: &Frame) -> bool {
        frame.not_contains(self.kern_start, self.kern_end)
        && frame.not_contains(self.mboot_start, self.mboot_end)
    }
}

impl Iterator for PhysMemIterator {
    type Item = Frame;

    fn next(&mut self) -> Option<Frame> {
        let c = self.current.clone();

        if c >= Frame::new(self.mm_end) {
            if let Some(e) = self.mm_iter.next() {
                self.mm_start = e.base_addr;
                self.mm_end = e.base_addr + e.length;
                self.current = Frame::new(self.mm_start);
                return self.next();
            } else {
                return None;
            }
        }

        if !self.is_valid(&c) {
            self.current = self.current.next();
            return self.next();
        }

        self.current = self.current.next();

        Some(c)
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

    let mut prev: Option<Frame> = None;
    let mut head: Option<Frame> = None;
    let mut tail: Option<Frame> = None;
    let mut max_cnt: u64 = 0;

    for el in iter {
        if let Some(p) = prev {
            let physmap = phys_to_physmap(p.address());

            let addr = physmap as *mut PhysAddr;

            unsafe {
                *addr = el.address();
            }

            if max_cnt == 0 {
                head = Some(el.clone());
                println!("0x{:x}", el.address());
            }
            tail = Some(el.clone());
        }

        max_cnt += 1;
        prev = Some(el);
    }


    if let Some(p) = prev {
        let addr = phys_to_physmap(p.address()) as *mut PhysAddr;

        unsafe {
            *addr = 0xFFFF_FFFF_FFFF_FFFF;
            println!("Value at 0x{:x} is 0x{:x}", addr as PhysAddr, *addr);
        }

    }

    let mut h = PHYS_HEAD.lock();
    let mut t = PHYS_TAIL.lock();

    *h = head;
    *t = tail;

    println!("Physical memory initialisation complete after {} iterations", max_cnt);
}

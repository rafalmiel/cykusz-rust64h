use arch::mm::PhysAddr;
use mboot2::memory::MemoryIter;
use arch::mm::PAGE_SIZE;

pub struct PhysMemIterator {
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

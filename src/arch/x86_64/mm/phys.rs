use ::arch::mm::PhysAddr;
use ::mboot2::memory::MemoryIter;
use ::arch::mm::PAGE_SIZE;
use ::arch::phys_to_physmap;

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
        let addr_e = addr + PAGE_SIZE;

        (addr < self.kern_start || addr >= self.kern_end) &&
        (addr < self.mboot_start || addr >= self.mboot_end) &&
        (addr_e < self.kern_start || addr_e >= self.kern_end) &&
        (addr_e < self.mboot_start || addr_e >= self.mboot_end)
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

        if ! self.is_valid(c) {
            self.current += PAGE_SIZE;
            return self.next();
        }

        self.current += PAGE_SIZE;

        Some(c)
    }
}

pub fn init(mm_iter:        MemoryIter,
            kern_start:     PhysAddr,
            kern_end:       PhysAddr,
            mboot_start:    PhysAddr,
            mboot_end:      PhysAddr) {
    let iter = PhysMemIterator::new(mm_iter, kern_start, kern_end, mboot_start, mboot_end);

    println!("Initialising physical memory");

    let mut cnt = 0;

    let mut prev: Option<PhysAddr> = None;

    for (i, el) in iter.enumerate() {
        if let Some(p) = prev {
            let physmap = phys_to_physmap(p);

            let addr = physmap as *mut PhysAddr;

            unsafe {
                *addr = el;
                if i % 100 == 0 {
                    println!("Value at 0x{:x} is 0x{:x}", addr as PhysAddr, *addr);
                }
            }
        }

        cnt += 1;
        prev = Some(el);
    }

    if let Some(p) = prev {
        let addr = phys_to_physmap(p) as *mut PhysAddr;

        unsafe {
            *addr = 0xFFFF_FFFF_FFFF_FFFF;
            println!("Value at 0x{:x} is 0x{:x}", addr as PhysAddr, *addr);
        }

    }

    println!("Physical memory initialisation complete after {} iterations", cnt);
}

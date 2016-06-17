use arch::mm::PhysAddr;
use arch::mm::PAGE_SIZE;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Frame {
    number:         u64
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
}

impl Drop for Frame {
    fn drop(&mut self) {
        ::arch::mm::phys::deallocate(self);
    }
}

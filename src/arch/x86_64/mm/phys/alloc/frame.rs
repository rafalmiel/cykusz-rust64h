use arch::mm::PhysAddr;
use arch::mm::MappedAddr;
use arch::mm::PAGE_SIZE;

use arch::mm::phys_to_physmap;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Frame {
    number:         usize
}

impl Frame {
    // Restrict new static method to the mm submodule
    pub(arch::mm) fn new(address: PhysAddr) -> Frame {
        Frame {
            number: address / PAGE_SIZE
        }
    }

    pub fn clear(&mut self) {
        for a in (self.address_mapped()..self.address_mapped() + PAGE_SIZE).step_by(8) {
            unsafe {
                *(a as *mut u64) = 0;
            }
        }
    }

    pub fn address(&self) -> PhysAddr {
        self.number * PAGE_SIZE
    }

    pub fn address_mapped(&self) -> MappedAddr {
        phys_to_physmap(self.address())
    }

    pub fn end_address(&self) -> PhysAddr {
        self.number * PAGE_SIZE + PAGE_SIZE
    }

    pub fn number(&self) -> usize {
        self.number
    }

    pub fn next(&self) -> Frame {
        Frame {
            number: self.number + 1
        }
    }
}

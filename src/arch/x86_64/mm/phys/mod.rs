mod alloc;
mod iter;

pub use self::alloc::allocate;
pub use self::alloc::deallocate;
pub use self::alloc::frame::Frame;

use arch::mm::PhysAddr;
use mboot2::memory::MemoryIter;

pub fn init(mm_iter:        MemoryIter,
            kern_start:     PhysAddr,
            kern_end:       PhysAddr,
            mboot_start:    PhysAddr,
            mboot_end:      PhysAddr,
            modules_start:  PhysAddr,
            modules_end:    PhysAddr) {
    alloc::init(mm_iter, kern_start, kern_end, mboot_start, mboot_end, modules_start, modules_end);
}

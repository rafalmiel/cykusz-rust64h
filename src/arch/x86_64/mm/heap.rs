use arch::sync::{Mutex};
use linked_list_allocator::{Heap, align_up};
use alloc::allocator::{Alloc, Layout, AllocErr};
use core::ops::Deref;

pub const HEAP_START: usize = 0xfffff80000000000;
pub const HEAP_SIZE: usize = 1 * 4096; // 100 KiB / 25 pages
pub const HEAP_END: usize = HEAP_START + 4096 * 4096; // 4MB

fn request_more_mem(from: *const u8, size: usize) {
    //println!("Requesting more mem! 0x{:x} - size: 0x{:x}", from as usize, size);
    for addr in (from as usize..from as usize + size).step_by(::arch::mm::PAGE_SIZE) {
        ::arch::mm::virt::map(addr);
    }
}

pub struct LockedHeap(pub Mutex<Heap>);

impl LockedHeap {
    /// Creates an empty heap. All allocate calls will return `None`.
    pub const fn empty() -> LockedHeap {
        LockedHeap(Mutex::new(Heap::empty()))
    }
}

impl Deref for LockedHeap {
    type Target = Mutex<Heap>;

    fn deref(&self) -> &Mutex<Heap> {
        &self.0
    }
}

unsafe impl<'a> Alloc for &'a LockedHeap {
    unsafe fn alloc(&mut self, layout: Layout) -> Result<*mut u8, AllocErr> {
        let mut heap = self.0.lock();

        heap.alloc(layout.clone()).or_else(|_| {
            let top = heap.top();
            let req = align_up(layout.size(), 0x1000);

            if top + req > HEAP_END {
                panic!("Out of mem!");
            }

            request_more_mem(top as *const u8, req);

            heap.extend(req);

            heap.alloc(layout)
        })
    }

    unsafe fn dealloc(&mut self, ptr: *mut u8, layout: Layout) {
        self.0.lock().dealloc(ptr, layout)
    }
}

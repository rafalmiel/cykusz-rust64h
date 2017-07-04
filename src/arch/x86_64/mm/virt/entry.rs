use arch::mm::phys::Frame;
use arch::mm::MappedAddr;
use arch::mm::PhysAddr;

bitflags! {
    pub struct Entry: usize {
        const PRESENT       = 1 << 0;
        const WRITABLE      = 1 << 1;
        const USER          = 1 << 2;
        const WRT_THROUGH   = 1 << 3;
        const NO_CACHE      = 1 << 4;
        const ACCESSED      = 1 << 5;
        const DIRTY         = 1 << 6;
        const HUGE_PAGE     = 1 << 7;
        const GLOBAL        = 1 << 8;
        const NO_EXECUTE    = 1 << 63;
    }
}

impl Entry {
    pub fn new_empty() -> Entry {
        Entry {
            bits: 0
        }
    }

    pub unsafe fn from_addr(addr: MappedAddr) -> Entry {
        //println!("Dereferencing value at 0x{:x}", addr);
        Entry {
            bits: *(addr as *const MappedAddr)
        }
    }

    pub fn clear(&mut self) {
        self.bits = 0;
    }

    pub fn raw(&self) -> usize {
        self.bits
    }

    pub fn is_unused(&self) -> bool {
        self.bits == 0
    }

    pub fn address(&self) -> PhysAddr {
        self.bits as PhysAddr & 0x000fffff_fffff000
    }

    pub fn frame(&self) -> Option<Frame> {
        if self.contains(PRESENT) {
            Some(Frame::new(self.address()))
        } else {
            None
        }
    }

    pub fn set_frame_flags(&mut self, frame: &Frame, flags: Entry) {
        self.bits = frame.address();
        self.insert(flags);
    }

    pub fn set_frame(&mut self, frame: &Frame) {
        self.bits = frame.address();
    }

    pub fn set_flags(&mut self, flags: Entry) {
        self.insert(flags);
    }
}

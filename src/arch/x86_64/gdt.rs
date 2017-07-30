use x86::bits64::task::TaskStateSegment;

use kernel::mm::*;

#[derive(Copy, Clone, Debug)]
#[repr(packed)]
pub struct GdtEntry {
    pub limitl: u16,
    pub offsetl: u16,
    pub offsetm: u8,
    pub access: u8,
    pub flags_limith: u8,
    pub offseth: u8
}

const GDT_A_PRESENT: u8 = 1 << 7;
const GDT_A_RING_3: u8 = 3 << 5;
const GDT_A_TSS_AVAIL: u8 = 0x9;

impl GdtEntry {
    pub const fn new(offset: u32, limit: u32, access: u8, flags: u8) -> Self {
        GdtEntry {
            limitl: limit as u16,
            offsetl: offset as u16,
            offsetm: (offset >> 16) as u8,
            access: access,
            flags_limith: flags & 0xF0 | ((limit >> 16) as u8) & 0x0F,
            offseth: (offset >> 24) as u8
        }
    }

    pub fn set_offset(&mut self, offset: u32) {
        self.offsetl = offset as u16;
        self.offsetm = (offset >> 16) as u8;
        self.offseth = (offset >> 24) as u8;
    }

    pub fn set_limit(&mut self, limit: u32) {
        self.limitl = limit as u16;
        self.flags_limith = self.flags_limith & 0xF0 | ((limit >> 16) as u8) & 0x0F;
    }
}

static mut TSS: ::x86::bits64::task::TaskStateSegment = ::x86::bits64::task::TaskStateSegment::new();

pub fn init(stack_top: VirtAddr, gdt64: VirtAddr) {
    unsafe {
        TSS.rsp[0] = stack_top.0 as u64;

        let entry = &mut *((gdt64 + 8*5).0 as *mut GdtEntry);

        entry.access = GDT_A_PRESENT | GDT_A_RING_3 | GDT_A_TSS_AVAIL;
        entry.set_offset(&TSS as *const _ as usize as u32);
        entry.set_limit(::core::mem::size_of::<TaskStateSegment>() as u32);

        let up = (gdt64 + 8*6).0 as *mut GdtEntry as *mut u32;

        *up = ((&TSS as *const _ as usize) >> 32) as u32;

        ::x86::shared::task::load_tr(::x86::shared::segmentation::SegmentSelector::from_raw(5 << 3));
    }
}
pub mod cpuio;
pub mod mm;
pub mod int;
pub mod apic;
pub mod pic;
pub mod task;
pub mod sync;

use vga;
use mboot2;

use arch::mm::{PhysAddr, VirtAddr};

use self::mm::{phys_to_physmap, virt_to_phys};

pub const GDT_A_PRESENT: u8 = 1 << 7;
pub const GDT_A_RING_3: u8 = 3 << 5;
pub const GDT_A_TSS_AVAIL: u8 = 0x9;

use x86::bits64::task::TaskStateSegment;
static mut TSS: ::x86::bits64::task::TaskStateSegment = ::x86::bits64::task::TaskStateSegment::new();

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

#[no_mangle]
pub extern "C" fn x86_64_rust_main(multiboot_addr: PhysAddr, stack_top: VirtAddr, gdt64_tss: VirtAddr) {
    unsafe {
        println!("tss: 0x{:x} gdt64: 0x{:x}", virt_to_phys(&TSS as *const _ as usize), gdt64_tss);
        println!("stack top 0x{:x}", stack_top);
    }

    unsafe {
        TSS.rsp[0] = stack_top as u64;

        let entry = &mut *((gdt64_tss + 8*5) as *mut GdtEntry);

        entry.access = GDT_A_PRESENT | GDT_A_RING_3 | GDT_A_TSS_AVAIL;
        entry.set_offset(&TSS as *const _ as usize as u32);
        entry.set_limit(::core::mem::size_of::<TaskStateSegment>() as u32);

        let up = ((gdt64_tss + 8*6) as *mut GdtEntry as *mut u32);

        *up = ((&TSS as *const _ as usize) >> 32) as u32;

        ::x86::shared::task::load_tr(::x86::shared::segmentation::SegmentSelector::from_raw(5 << 3));

        println!("TSS set {:?}", entry);
        asm!("xchg %bx, %bx");
    }

    unsafe {
        asm!("xchg %bx, %bx");
    }
    vga::clear_screen();

    println!("Loading mboot at addr 0x{:x}", phys_to_physmap(multiboot_addr));

    let mboot_info = unsafe { mboot2::load(phys_to_physmap(multiboot_addr)) };

    mm::init(&mboot_info);

    int::init();

    if let Some(mtag) = mboot_info.modules_tags().next() {
        //let string = unsafe { ::core::str::from_utf8_unchecked(mtag.name) };

        println!("Module start: 0x{:x}", mtag.mod_start);
        println!("Module end  : 0x{:x}", mtag.mod_end);
        println!("Name: {}", mtag.name());

        unsafe {
            asm!("xchg %bx, %bx");
        }

        // Copy user program to user mapping!
        mm::virt::map_flags(0x400000, mm::virt::entry::USER | mm::virt::entry::WRITABLE);

        for (i, ptr) in (mtag.mod_start..mtag.mod_end).enumerate() {
            unsafe {
                println!("0x{:x} {}, 0x{:x}", ptr, i, *((phys_to_physmap(mtag.mod_start as usize + i)) as *mut u8));
                *((0x400000 as usize + i) as *mut u8) = *((phys_to_physmap(mtag.mod_start as usize + i)) as *mut u8);
                println!("0x{:x} {}, 0x{:x}", ptr, i, *((0x400000 as usize + i) as *mut u8));
            }
        }

        unsafe {
            asm!("xchg %bx, %bx");
        }
    }

    unsafe {
        int::idt::test() ;
    }

    //int::fire_timer();

    ::rust_main();
}

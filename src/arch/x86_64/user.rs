use kernel::mm::*;

use drivers::multiboot2;

pub fn init(mboot_info: &multiboot2::Info) {
    if let Some(mtag) = mboot_info.modules_tags().next() {
        //let string = unsafe { ::core::str::from_utf8_unchecked(mtag.name) };

        // Copy user program to user mapping!
        map_flags(VirtAddr(0x400000), ::kernel::mm::virt::PageFlags::USER | ::kernel::mm::virt::PageFlags::WRITABLE);

        // Allocate stack data
        map_flags(VirtAddr(0x600000), ::kernel::mm::virt::PageFlags::USER | ::kernel::mm::virt::PageFlags::WRITABLE);

        for (i, ptr) in (mtag.mod_start..mtag.mod_end).enumerate() {
            unsafe {
                VirtAddr(0x400000 as usize + i)
                    .store(
                        PhysAddr(ptr as usize).to_mapped().read::<u8>()
                    );
            }
        }
    }
}
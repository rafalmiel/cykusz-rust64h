use core::mem::size_of;

use arch::apic;

#[repr(packed, C)]
pub struct Rsdp {
    signature: [u8; 8],
    checksum: u8,
    oemid: [u8; 6],
    revision: u8,
    pub rsdt_address: u32,
}

impl Rsdp {
    pub unsafe fn is_valid(&self) -> bool {
        if &self.signature as &[u8] != b"RSD PTR " {
            false
        } else {
            println!("Found RSDP sig at 0x{:x}", self as *const _ as usize);

            println!("Checksum: 0x{:x}", self.checksum);
            println!("Revision: {}", self.revision);

            apic::util::checksum(self as *const _ as *const u8, size_of::<Rsdp>() as isize)
        }
    }

    pub unsafe fn find() -> Option<&'static Rsdp> {
        use arch::mm::phys_to_physmap;
        let ebda_address =
        unsafe {
            phys_to_physmap(*(phys_to_physmap(0x40E) as *const u16) as usize * 4)
        };
        let ebda_iter = (
            ebda_address as u64
            ..
            (ebda_address + 1024) as u64).step_by(0x10);

        for addr in ebda_iter {
            let ptr = &*(addr as *const Rsdp);

            if ptr.is_valid() {
                return Some(ptr);
            }
        }

        println!("Rsdp not found on ebda 0x{:x}", ebda_address);

        let iter = (
            phys_to_physmap(0xE0_000) as u64
            ..
            phys_to_physmap(0x10_0000) as u64).step_by(0x10);

        for addr in iter {
            let ptr = &*(addr as *const Rsdp);

            if ptr.is_valid() {
                return Some(ptr);
            }
        }

        println!("Rsdp not found on E0_000");

        None
    }
}

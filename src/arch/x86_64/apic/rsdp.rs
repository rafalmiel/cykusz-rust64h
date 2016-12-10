use core::mem::size_of;

use arch::apic;

#[repr(packed, C)]
pub struct Rsdp {
    signature: [u8; 8],
    checksum: u8,
    oemid: [u8; 6],
    revision: u8,
    pub rsdt_address: u64,
}

impl Rsdp {
    pub unsafe fn is_valid(&self) -> bool {
        if &self.signature as &[u8] != b"RSD PTR " {
            false
        } else {
            println!("SizeOf rsdp {}", size_of::<Rsdp>());
            apic::util::checksum(self as *const _ as *const u8, size_of::<Rsdp>() as isize)
        }
    }

    pub unsafe fn find() -> Option<&'static Rsdp> {
        use arch::mm::phys_to_physmap;

        let iter = (
            phys_to_physmap(0xE000) as u64
            ..
            phys_to_physmap(0x10_0000) as u64).step_by(0x10u64);

        // TODO: Check ebda address

        for addr in iter {
            let ptr = &*(addr as *const Rsdp);

            if ptr.is_valid() {
                return Some(ptr);
            }
        }

        None
    }
}

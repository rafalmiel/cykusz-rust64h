use core::mem::size_of;
use arch::mm::*;

use arch::acpi;

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
            // println!("Found RSDP sig at 0x{:x}", self as *const _ as usize);

            // println!("Checksum: 0x{:x}", self.checksum);
            // println!("Revision: {}", self.revision);

            acpi::util::checksum(self as *const _ as *const u8, size_of::<Rsdp>() as isize)
        }
    }

    pub unsafe fn find() -> Option<&'static Rsdp> {
        let ebda_address = PhysAddr((PhysAddr(0x40E as usize).to_mapped().read::<u16>()) as usize * 4).to_mapped();
        ebda_address.to_phys();
        let ebda_iter = (
            ebda_address
            ..
            (ebda_address + 1024)).step_by(0x10);

        for addr in ebda_iter {
            let ptr = addr.read_ref::<Rsdp>();

            if ptr.is_valid() {
                return Some(ptr);
            }
        }

        // println!("Rsdp not found on ebda {}", ebda_address);

        let iter = (
            PhysAddr(0xE0_000 as usize).to_mapped()
            ..
            PhysAddr(0x100_000 as usize).to_mapped()).step_by(0x10);

        for addr in iter {
            let ptr = addr.read_ref::<Rsdp>();

            if ptr.is_valid() {
                return Some(ptr);
            }
        }
        // println!("Rsdp not found on 0xE0000");

        None
    }
}

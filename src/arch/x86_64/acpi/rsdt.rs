use core::mem::size_of;

use arch::mm::PhysAddr;
use arch::mm::MappedAddr;

use arch::mm::phys_to_physmap;

const MATD_ENTRY_PROC_LOCAL_APIC: u8  = 0x0;
const MATD_ENTRY_PROC_IOAPIC: u8      = 0x1;
const MATD_ENTRY_INT_SRC_OVERRIDE: u8 =	0x2;

#[repr(packed, C)]
struct RSDTHeader {
    signature: [u8; 4],
    length: u32,
    revision: u8,
    checksum: u8,
    oem_id: [u8; 6],
    oem_table_id: [u8; 8],
    oem_revision: u32,
    creator_id: u32,
    creator_revision: u32
}

#[repr(packed, C)]
struct MATDHeader {
	rsdt: RSDTHeader,
	local_controller_address: u32,
	flags: u32
}

#[repr(packed, C)]
struct MATDEntry {
	typ: u8,
	length: u8
}

#[repr(packed, C)]
struct MATDEntryLocalApic {
    matd: MATDEntry,
	proc_id: u8,
	apic_id: u8,
	flags: u32
}

#[repr(packed, C)]
struct MATDEntryIOApic {
    matd: MATDEntry,
	ioapic_id: u8,
	reserved: u8,
	ioapic_address: u32,
	global_int_base: u32
}

#[repr(packed, C)]
struct MATDEntryIntSrc {
    matd: MATDEntry,
	bus_src: u8,
	irq_src: u8,
	global_sys_int: u32,
	flags: u16
}

pub struct Rsdt {
    matd: Option<&'static MATDHeader>,
    ioapic_address: Option<PhysAddr>
}

impl Rsdt {
    pub const fn new() -> Rsdt {
        Rsdt {
            matd: None,
            ioapic_address: None
        }
    }

    fn parse_matd(&mut self, matd_header: &'static MATDHeader) {
        self.matd = Some(matd_header);
        println!("local ctrl addr 0x{:x}, len: {}",
            self.local_controller_address().unwrap(),
            matd_header.rsdt.length);

        unsafe {
            let mut a = (matd_header as *const _ as *const u8).offset(size_of::<MATDHeader>() as isize);
            let limit: *const u8 = (a).offset(matd_header.rsdt.length as isize);

            while a < limit {
                let entry = &*(a as *const MATDEntry);

                //println!("Entry type {}, len: {}", entry.typ, entry.length);

                match entry.typ {
                    MATD_ENTRY_PROC_IOAPIC => {
                        println!("FOUND IOAPIC!");

                        let ioapic = &*(a as *const MATDEntryIOApic);
                        self.ioapic_address = Some(ioapic.ioapic_address as PhysAddr);
                    },
                    MATD_ENTRY_INT_SRC_OVERRIDE => {
                        let int = &*(a as *const MATDEntryIntSrc);
                        println!("int override {} -> {}", int.irq_src, int.global_sys_int);
                    },
                    _ => {}
                }

                a = a.offset(entry.length as isize);
            }

            println!("start: 0x{:x} end: 0x{:x}", matd_header as *const _ as usize, limit as usize);


        }
    }

    pub fn local_controller_address(&self) -> Option<MappedAddr> {
        self.matd.and_then(|addr| {
            Some(phys_to_physmap(addr.local_controller_address as PhysAddr))
        })
    }

    pub fn ioapic_address(&self) -> Option<MappedAddr> {
        self.ioapic_address.and_then(|addr| {
            Some(phys_to_physmap(addr))
        })
    }

    pub fn init(&mut self, rsdt_address: PhysAddr) {
        use super::util::checksum;
        unsafe {
            let rsdt_header = &*(phys_to_physmap(rsdt_address) as *const RSDTHeader);

            println!("{} {}",
                &rsdt_header.signature == b"RSDT",
                checksum(rsdt_header as *const _ as *const u8, rsdt_header.length as isize));

            if &rsdt_header.signature == b"RSDT"
               && checksum(rsdt_header as *const _ as *const u8,
                           rsdt_header.length as isize) {
                println!("RSDT Header length: {}, sizeof: {}", rsdt_header.length, size_of::<RSDTHeader>());

                let entries = (rsdt_header.length - size_of::<RSDTHeader>() as u32) / 4;

                println!("Entries: {}", entries);

                for i in 0..entries {
                    let entry = *((rsdt_header as *const _ as usize
                                   + size_of::<RSDTHeader>() + i as usize * 4) as *const u32);

                    let hdr = &*(phys_to_physmap(entry as PhysAddr) as *const RSDTHeader);

                    println!("LEN: {}", hdr.length);

                    if &hdr.signature == b"APIC"
                       && checksum(hdr as *const _ as *const u8, hdr.length as isize) {
                        println!("Found APIC");
                        self.parse_matd(&*(hdr as *const _ as *const MATDHeader));
                        println!("IOAPIC addr: 0x{:x}", self.ioapic_address().unwrap());
                    }
                }
            }
        }
    }
}

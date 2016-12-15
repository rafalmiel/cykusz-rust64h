use core::ptr::write_volatile;
use core::ptr::read_volatile;

use arch::mm::MappedAddr;

const REG_ID: u32 =	    0x00;
const REG_VER: u32 =    0x01;
const REG_ARB: u32 =    0x02;

const fn reg_redtbl_low(num: u32) -> u32 {
    0x10 + (2*num)
}

const fn reg_redtbl_high(num: u32) -> u32 {
    0x11 + (2*num)
}

struct RegId(u32);
struct RegVer(u32);

impl RegId {
    pub const fn id(&self) -> u32 {
        (self.0 >> 24) & 0b1111
    }
}

impl RegVer {
    pub const fn apic_version(&self) -> u32 {
        self.0 & 0b11111111
    }

    pub const fn max_red_entry(&self) -> u32 {
        (self.0 >> 16) & 0b11111111
    }
}

pub struct IOApic {
    ioapic_base: Option<MappedAddr>,
}

impl IOApic {
    fn read(&self, reg: u32) -> u32 {
        if let Some(base) = self.ioapic_base {
            unsafe {
                write_volatile::<u32>(
                    base as *mut u32,
                    reg
                );

                return read_volatile::<u32>(
                    (base + 0x10) as *const u32
                );
            }
        } else {
            panic!("IOApic module not initialised");
        }
    }

    fn write(&self, reg: u32, value: u32) {
        if let Some(base) = self.ioapic_base {
            unsafe {
                write_volatile::<u32>(
                    base as *mut u32,
                    reg
                );

                write_volatile::<u32>(
                    (base + 0x10) as *mut u32,
                    value
                );
            }
        } else {
            panic!("IOApic module not initialised");
        }
    }

    pub fn id(&self) -> u32 {
        RegId(self.read(REG_ID)).id()
    }

    pub fn identification(&self) -> u32 {
        RegId(self.read(REG_ARB)).id()
    }

    pub fn max_red_entries(&self) -> u32 {
        RegVer(self.read(REG_VER)).max_red_entry()
    }

    pub fn version(&self) -> u32 {
        RegVer(self.read(REG_VER)).apic_version()
    }

    pub const fn new() -> IOApic {
        IOApic {
            ioapic_base: None
        }
    }
    pub fn init(&mut self, base: MappedAddr) {
        self.ioapic_base = Some(base);

    }
}

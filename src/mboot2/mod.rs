mod tags;

pub use self::tags::*;

#[repr(C)]
pub struct Info {
    pub size:       u32,
    reserved:       u32,
    pub tag:        tags::Tag
}

pub unsafe fn load(addr: u64) -> &'static Info {
    &*(addr as *const Info)
}

impl Info {
    pub fn kernel_start_addr(&self) -> u64 {
        let item = self.elf_tag().unwrap().sections().nth(0).unwrap();

        item.addr
    }

    pub fn kernel_end_addr(&self) -> u64 {
        let item = self.elf_tag().unwrap().sections().last().unwrap();

        item.addr + item.size
    }

    pub fn tags(&self) -> tags::TagIter {
        tags::TagIter {
            current: &self.tag as *const _
        }
    }

    pub fn memory_map_tag(&self) -> Option<&'static tags::memory::Memory> {
        self.tags().find(
            |t| t.typ == 6
        ).map(
            |t| unsafe {
                &*(t as *const tags::Tag as *const tags::memory::Memory)
            }
        )
    }

    pub fn address_tag(&self) -> Option<&'static tags::address::Address> {
        self.tags().find(
            |t| t.typ == 2
        ).map(
            |t| unsafe {
                &*(t as *const tags::Tag as *const tags::address::Address)
            }
        )
    }

    pub fn elf_tag(&self) -> Option<&'static tags::elf::Elf> {
        self.tags().find(
            |t| t.typ == 9
        ).map(
            |t| unsafe {
                &*(t as *const tags::Tag as *const tags::elf::Elf)
            }
        )
    }
}

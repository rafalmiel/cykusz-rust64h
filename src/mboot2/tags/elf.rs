use mboot2::tags::Tag;

#[repr(packed)]
pub struct Elf {
    pub tag:                    Tag,
    pub num:                    u32,
    pub entsize:                u32,
    pub shndx:                  u32,
    pub first_entry:            ElfSection
}

#[repr(C)]
pub struct ElfSection {
    name:                       u32,
    pub typ:                    u32,
    pub flags:                  u64,
    pub addr:                   u64,
    offset:                     u64,
    pub size:                   u64,
    link:                       u32,
    info:                       u32,
    addr_align:                 u64,
    entsize:                    u64
}

pub struct ElfSectionIter {
    current:                    &'static ElfSection,
    remaining:                  u32,
    entry_size:                 u32
}

#[repr(u32)]
pub enum ElfSectionType {
    Unused =                    0,
    ProgramSection =            1,
    LinkerSymbolTable =         2,
    RelaRelocation =            4,
    SymbolHashTable =           5,
    DynamicLinkingTable =       6,
    Note =                      7,
    Uninitialized =             8,
    RelRelocation =             9,
    Reserved =                  10,
    DynamicLoaderSymbolTable =  11,
}

#[repr(u32)]
pub enum ElfSectionFlags {
    Writable =                  0x1,
    Allocated =                 0x2,
    Executable =                0x4,
}

impl Elf {
    pub fn sections(&'static self) -> ElfSectionIter {
        ElfSectionIter {
            current:    &self.first_entry,
            remaining:  self.num - 1,
            entry_size: self.entsize,
        }
    }
}

impl ElfSection {
    pub fn address(&self) -> u64 {
        self.addr
    }

    pub fn end_address(&self) -> u64 {
        self.addr + self.size
    }
}

impl Iterator for ElfSectionIter {
    type Item = &'static ElfSection;

    fn next(&mut self) -> Option<&'static ElfSection> {
        if self.remaining == 0 {
            None
        } else {
            let section = self.current;

            self.current = unsafe {
                &*(
                    (self.current as *const _ as u64 + self.entry_size as u64)
                        as *const ElfSection
                )
            };

            self.remaining -= 1;

            if (section.typ == ElfSectionType::Unused as u32) ||
               (section.flags & ElfSectionFlags::Allocated as u64 == 0) {
                self.next()
            } else {
                Some(section)
            }
        }
    }
}

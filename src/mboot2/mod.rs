use util;

#[repr(C)]
pub struct Info {
    pub size:       u32,
    reserved:       u32,
    pub tag:        Tag
}

#[repr(C)]
pub struct Tag {
    pub typ:        u32,
    pub size:       u32
}

#[repr(C)]
pub struct MemoryMapTag {
    pub typ:            u32,
    pub size:           u32,
    pub entry_size:     u32,
    pub entry_ver:      u32,
    pub first_entry:    MemoryMapEntry
}

#[repr(C)]
pub struct MemoryMapEntry {
    pub base_addr:      u64,
    pub length:         u64,
    pub typ:            u32,
    pub reserved:       u32
}

pub struct TagIter {
    current: *const Tag
}

pub struct MemoryMapIter {
    current:    *const MemoryMapEntry,
    last:       *const MemoryMapEntry,
    entry_size: u32
}

pub unsafe fn load(addr: u64) -> &'static Info {
    &*(addr as *const Info)
}

impl Info {
    pub fn tags(&self) -> TagIter {
        TagIter {
            current: &self.tag as *const _
        }
    }

    pub fn memory_map_tag(&self) -> Option<&'static MemoryMapTag> {
        self.tags().find(
            |t| t.typ == 6
        ).map(
            |t| unsafe {
                &*(t as *const Tag as *const MemoryMapTag)
            }
        )
    }
}

impl MemoryMapTag {
    pub fn entries(&self) -> MemoryMapIter {
        MemoryMapIter {
            current: (&self.first_entry) as *const _,
            last: ((self as *const _) as u64 + self.size as u64 - self.entry_size as u64) as *const _,
            entry_size: self.entry_size
        }
    }
}

impl Iterator for TagIter {
    type Item = &'static Tag;

    fn next(&mut self) -> Option<&'static Tag> {
        match unsafe{&*self.current} {
            &Tag {typ: 0, size: 8} => None,
            tag => {
                self.current = util::align(self.current as u64 + tag.size as u64, 8) as *const _;

                Some(tag)
            }
        }
    }
}

impl Iterator for MemoryMapIter {
    type Item = &'static MemoryMapEntry;

    fn next(&mut self) -> Option<&'static MemoryMapEntry> {
        if self.current as u64 > self.last as u64 {
            None
        } else {
            let entry = unsafe {
                &*self.current
            };

            self.current = (self.current as u64 + self.entry_size as u64) as *const MemoryMapEntry;

            if entry.typ == 1 {
                Some(entry)
            } else {
                Some(entry)
            }
        }
    }
}

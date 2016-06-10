mod tags;

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
}

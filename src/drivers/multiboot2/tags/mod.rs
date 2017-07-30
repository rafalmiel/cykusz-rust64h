pub mod memory;
pub mod address;
pub mod elf;
pub mod modules;

use util;

#[repr(C)]
pub struct Tag {
    pub typ:        u32,
    pub size:       u32
}

pub struct TagIter {
    pub current: *const Tag,
}

impl Iterator for TagIter {
    type Item = &'static Tag;

    fn next(&mut self) -> Option<&'static Tag> {
        match unsafe{&*self.current} {
            &Tag {typ: 0, size: 8} => None,
            tag => {
                self.current =
                    util::align(
                        self.current as usize + tag.size as usize, 8
                    ) as *const _;

                Some(tag)
            }
        }
    }
}

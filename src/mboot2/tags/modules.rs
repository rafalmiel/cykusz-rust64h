use mboot2::tags::Tag;

#[repr(packed)]
pub struct Modules {
    pub tag:                    Tag,
    pub mod_start:              u32,
    pub mod_end:                u32,
}
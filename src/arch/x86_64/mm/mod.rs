pub mod phys;
pub mod virt;
mod types;

pub use self::types::*;

use drivers::multiboot2;

pub const PAGE_SIZE: usize = 0x1000;

pub fn init(mboot_info: &multiboot2::Info) {
    phys::init(mboot_info);

    println!("[ OK ] Initialised physical memory");

    virt::init(&mboot_info);

    println!("[ OK ] Initialised virtual memory");
}
pub use arch::mm::*;

use arch;

pub mod heap;
pub mod virt;
mod frame;

pub use self::frame::Frame;

pub fn allocate() -> Option<Frame> {
    arch::mm::phys::allocate()
}

pub fn deallocate(frame: &Frame) {
    arch::mm::phys::deallocate(frame);
}

pub fn map_flags(virt: VirtAddr, flags: virt::PageFlags) {
    arch::mm::virt::map_flags(virt, flags);
}

pub fn map(virt: VirtAddr) {
    arch::mm::virt::map(virt);
}

pub fn unmap(virt: VirtAddr) {
    arch::mm::virt::unmap(virt);
}

#[allow(unused)]
pub fn map_to(virt: VirtAddr, phys: PhysAddr) {
    arch::mm::virt::map_to(virt, phys);
}

pub trait PhysAddrConv {
    fn to_mapped(&self) -> MappedAddr;
    fn to_virt(&self) -> VirtAddr;
}

pub trait VirtAddrConv {
    fn to_phys(&self) -> PhysAddr;
}

pub trait MappedAddrConv {
    fn to_phys(&self) -> PhysAddr;
}

pub fn init() {
    heap::init();

    println!("[ OK ] Initialised heap");
}


use core::fmt;
use core::ops::*;

pub use kernel::mm::{PhysAddrConv, MappedAddrConv, VirtAddrConv};

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Debug, Default)]
pub struct PhysAddr(pub usize);

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Debug, Default)]
pub struct MappedAddr(pub usize);

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Debug, Default)]
pub struct VirtAddr(pub usize);

enable_unsigned_ops!(PhysAddr);
enable_unsigned_ops!(MappedAddr);
enable_unsigned_ops!(VirtAddr);

const VIRT : VirtAddr = VirtAddr(0xFFFFFF0000000000);
const PHYSMAP : MappedAddr = MappedAddr(0xFFFF800000000000);

impl PhysAddrConv for PhysAddr {
    fn to_mapped(&self) -> MappedAddr {
        if self.0 < PHYSMAP.0 {
            return MappedAddr(self.0 + PHYSMAP.0);
        } else {
            return MappedAddr(self.0);
        }
    }

    fn to_virt(&self) -> VirtAddr {
        if self.0 < PHYSMAP.0 {
            return VirtAddr(self.0 + VIRT.0);
        } else {
            return VirtAddr(self.0);
        }
    }
}

impl VirtAddrConv for VirtAddr {
    fn to_phys(&self) -> PhysAddr {
        if self >= &VIRT {
            PhysAddr(self.0 - VIRT.0)
        } else {
            PhysAddr(self.0)
        }
    }
}

impl MappedAddrConv for MappedAddr {
    fn to_phys(&self) -> PhysAddr {
        if self >= &PHYSMAP {
            PhysAddr(self.0 - PHYSMAP.0)
        } else {
            PhysAddr(self.0)
        }
    }
}
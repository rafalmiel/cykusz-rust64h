use arch::mm::virt::entry::Entry;
use arch::mm::virt::entry;
use arch::mm::phys::Frame;

use arch::mm::PhysAddr;

const ENTRIES_COUNT: usize = 4096;

pub struct Table {
    frame: Frame,
}

impl Table {
    pub fn new_from_frame(frame: Frame) -> Table {
        Table {
            frame: frame,
        }
    }

    fn entry_at(&self, idx: usize) -> Entry {
        unsafe {
            Entry::from_addr(self.frame.address_mapped() + idx * 8)
        }
    }

    fn entry_write(&self, idx: usize, entry: &Entry) {
        assert!(idx < ENTRIES_COUNT, "Entry index out of bound");

        unsafe {
            *((self.frame.address_mapped() + idx * 8) as *mut PhysAddr) = entry.raw();
        }
    }

    pub fn next_level(&self, idx: usize) -> Table {
        let mut entry = self.entry_at(idx);

        if entry.is_unused() {
            let frame = ::arch::mm::phys::allocate().expect("Out of memory!");

            entry.set_frame(&frame);
        }

        entry.set_flags(entry::PRESENT | entry::WRITABLE);

        self.entry_write(idx, &entry);

        println!("Writing entry at idx {} -> 0x{:x}", idx, entry.raw());

        Table::new_from_frame(Frame::new(entry.address()))
    }
}

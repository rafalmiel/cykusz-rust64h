use arch::mm::virt::entry::Entry;
use arch::mm::virt::entry;
use arch::mm::phys::Frame;

const ENTRIES_COUNT: usize = 512;

pub struct Table {
    entries: [Entry; ENTRIES_COUNT]
}

impl Table {
    pub fn new_at_frame_mut<'a>(frame: &Frame) -> &'a mut Table {
        unsafe {
            &mut *(frame.address_mapped() as *mut Table)
        }
    }

    pub fn new_at_frame<'a>(frame: &Frame) -> &'a Table {
        unsafe {
            &*(frame.address_mapped() as *const Table)
        }
    }

    pub fn clear(&mut self) {
        for i in 0..ENTRIES_COUNT {
            self.entries[i].clear();
        }
    }

    pub fn next_level_mut(&mut self, idx: usize) -> Option<&mut Table> {
        let entry = &self.entries[idx];

        if !entry.contains(entry::PRESENT) {
            return None
        }

        Some(Table::new_at_frame_mut(&Frame::new(entry.address())))
    }

    pub fn alloc(&mut self, idx: usize) {
        let entry = &mut self.entries[idx];

        if !entry.contains(entry::PRESENT) {
            let frame = ::arch::mm::phys::allocate().expect("Out of memory!");

            Table::new_at_frame_mut(&frame).clear();

            entry.set(&frame, entry::PRESENT | entry::WRITABLE);
        }
    }

    pub fn unmap(&mut self, idx: usize) {
        let entry = &mut self.entries[idx];

        if entry.contains(entry::PRESENT) {
            let frame = ::arch::mm::phys::Frame::new(entry.address());

            ::arch::mm::phys::deallocate(&frame);

            entry.clear();

            //println!("Unmapped index {}", idx);
        }
    }

    pub fn alloc_next_level(&mut self, idx: usize) -> &mut Table {
        let entry = &mut self.entries[idx];

        if !entry.contains(entry::PRESENT) {
            let frame = ::arch::mm::phys::allocate().expect("Out of memory!");

            Table::new_at_frame_mut(&frame).clear();

            entry.set_frame(&frame);
        }

        entry.set_flags(entry::PRESENT | entry::WRITABLE);

        //println!("Writing entry at idx {} -> 0x{:x}", idx, entry.raw());

        Table::new_at_frame_mut(&Frame::new(entry.address()))
    }
}

use arch::mm::virt::entry::Entry;
use arch::mm::virt::entry;
use arch::mm::phys::Frame;

const ENTRIES_COUNT: usize = 512;

pub struct Table {
    entries: [Entry; ENTRIES_COUNT]
}

impl Table {
    pub fn new_at_frame<'a>(frame: &Frame) -> &'a mut Table {
        unsafe {
            &mut *(frame.address_mapped() as *mut Table)
        }
    }

    pub fn clear(&mut self) {
        for i in 0..ENTRIES_COUNT {
            self.entries[i].clear();
        }
    }

    pub fn next_level(&mut self, idx: usize) -> &mut Table {
        let entry = &mut self.entries[idx];

        if entry.is_unused() {
            let frame = ::arch::mm::phys::allocate().expect("Out of memory!");

            Table::new_at_frame(&frame).clear();

            entry.set_frame(&frame);
        }

        entry.set_flags(entry::PRESENT | entry::WRITABLE);

        println!("Writing entry at idx {} -> 0x{:x}", idx, entry.raw());

        Table::new_at_frame(&Frame::new(entry.address()))
    }
}

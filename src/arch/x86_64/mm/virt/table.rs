use core::marker::PhantomData;

use arch::mm::virt::entry::Entry;
use arch::mm::virt::entry;
use arch::mm::phys::Frame;

const ENTRIES_COUNT: usize = 512;

pub enum Level4 {}
#[allow(dead_code)]
pub enum Level3 {}
#[allow(dead_code)]
pub enum Level2 {}
pub enum Level1 {}

pub trait HierarchicalLevel: TableLevel {
    type NextLevel: TableLevel;
}
pub trait TableLevel {}
pub trait LastLevel : TableLevel {}
pub trait HugePageLevel : TableLevel {}
pub trait TopLevel : TableLevel {}

impl TableLevel for Level4 {}
impl TableLevel for Level3 {}
impl TableLevel for Level2 {}
impl TableLevel for Level1 {}

impl LastLevel for Level1 {}
impl TopLevel for Level4 {}
impl HugePageLevel for Level2 {}
impl NotLastLevel for Level4 {
    type NextLevel = Level3;
}
impl NotLastLevel for Level3 {
    type NextLevel = Level2;
}
impl NotLastLevel for Level2 {
    type NextLevel = Level1;
}

pub struct Table<L: TableLevel> {
    entries: [Entry; ENTRIES_COUNT],
    level: PhantomData<L>,
}

pub type P4Table = Table<Level4>;

impl<L> Table<L>
where
    L :TopLevel
{
    pub fn new_mut<'a>(frame: &Frame) -> &'a mut Table<L> {
        Table::<L>::new_at_frame_mut(frame)
    }

    pub fn new<'a>(frame: &Frame) -> &'a Table<L> {
        Table::<L>::new_at_frame(frame)
    }
}

impl<L> Table<L>
where
    L: TableLevel
{
    fn new_at_frame_mut<'a>(frame: &Frame) -> &'a mut Table<L> {
        unsafe {
            &mut *(frame.address_mapped() as *mut Table<L>)
        }
    }

    fn new_at_frame<'a>(frame: &Frame) -> &'a Table<L> {
        unsafe {
            &*(frame.address_mapped() as *const Table<L>)
        }
    }

    pub fn clear(&mut self) {
        for i in 0..ENTRIES_COUNT {
            self.entries[i].clear();
        }
    }
}

impl<L> Table<L>
where
    L: NotLastLevel
{

    pub fn next_level_mut(&mut self, idx: usize) -> Option<&mut Table<L::NextLevel>> {
        let entry = &self.entries[idx];

        if !entry.contains(entry::PRESENT) {
            return None
        }

        Some(Table::<L::NextLevel>::new_at_frame_mut(&Frame::new(entry.address())))
    }

    pub fn alloc_next_level(&mut self, idx: usize) -> &mut Table<L::NextLevel> {
        let entry = &mut self.entries[idx];

        if !entry.contains(entry::PRESENT) {
            let frame = ::arch::mm::phys::allocate().expect("Out of memory!");

            Table::<L::NextLevel>::new_at_frame_mut(&frame).clear();

            entry.set_frame(&frame);
        }

        entry.set_flags(entry::PRESENT | entry::WRITABLE | entry::USER);

        Table::<L::NextLevel>::new_at_frame_mut(&Frame::new(entry.address()))
    }
}

impl<L> Table<L>
where
    L: HugePageLevel
{
    pub fn set_hugepage(&mut self, idx: usize, frame: &Frame) {
        let entry = &mut self.entries[idx];

        if !entry.contains(entry::PRESENT) {
            entry.set_frame_flags(&frame, entry::PRESENT | entry::WRITABLE | entry::HUGE_PAGE);
        }
    }
}

impl<L> Table<L>
where
    L: LastLevel
{
    pub fn alloc(&mut self, idx: usize) {
        let entry = &mut self.entries[idx];

        if !entry.contains(entry::PRESENT) {
            let frame = ::arch::mm::phys::allocate().expect("Out of memory!");

            Table::<L>::new_at_frame_mut(&frame).clear();

            entry.set_frame_flags(&frame, entry::PRESENT | entry::WRITABLE);
        }
    }

    pub fn set(&mut self, idx: usize, frame: &Frame) {
        let entry = &mut self.entries[idx];

        if !entry.contains(entry::PRESENT) {
            entry.set_frame_flags(&frame, entry::PRESENT | entry::WRITABLE);
        }
    }

    pub fn alloc_set_flags(&mut self, idx: usize, flags: entry::Entry) {
        let entry = &mut self.entries[idx];

        if !entry.contains(entry::PRESENT) {
            let frame = ::arch::mm::phys::allocate().expect("Out of memory!");

            Table::<L>::new_at_frame_mut(&frame).clear();

            entry.set_frame_flags(&frame, flags | entry::PRESENT);
        } else {
            let frame = Frame::new(entry.address());
            entry.set_frame_flags(&frame, flags | entry::PRESENT);
        }
    }

    pub fn set_flags(&mut self, idx: usize, frame: &Frame, flags: entry::Entry) {
        let entry = &mut self.entries[idx];

        if !entry.contains(entry::PRESENT) {
            entry.set_frame_flags(&frame, flags);
        }
    }

    pub fn unmap(&mut self, idx: usize) {
        let entry = &mut self.entries[idx];

        if entry.contains(entry::PRESENT) {
            let frame = ::arch::mm::phys::Frame::new(entry.address());

            ::arch::mm::phys::deallocate(&frame);

            entry.clear();
        }
    }
}

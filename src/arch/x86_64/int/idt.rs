use core::ptr;
use x86;
use x86::shared::dtables::*;
use x86::current::irq::IdtEntry;
use x86::shared::PrivilegeLevel;
use x86::shared::paging::VAddr;

extern "C" {
    static interrupt_handlers: [*const u8; 256];
}

pub struct Idt {
    table: [IdtEntry; 256],
}

impl Idt {
    pub const fn new() -> Idt {
        Idt {
            table: [x86::current::irq::IdtEntry::MISSING; 256],
        }
    }

    pub fn init(&mut self) {
        self.setup_gates();

        unsafe {
            x86::shared::dtables::lidt(
                &DescriptorTablePointer::new_idtp(&self.table)
            );
        }
    }

    fn setup_gates(&mut self) {
        unsafe {
            for (index, &handler) in interrupt_handlers.iter().enumerate() {
                if handler != ptr::null() {
                    self.set_gate(index, handler);
                }
            }
        }
    }

    fn set_gate(&mut self, num: usize, handler: *const u8) {
        use x86::shared::segmentation::cs;
        if num != 80 {
            self.table[num] =
                IdtEntry::new(
                    VAddr::from_usize(handler as usize),
                    cs().bits(),
                    PrivilegeLevel::Ring0,
                    false
                );
        } else {
            self.table[num] =
                IdtEntry::new(
                    VAddr::from_usize(handler as usize),
                    cs().bits(),
                    PrivilegeLevel::Ring3,
                    false
                );
        }

    }
}

pub unsafe fn test() {
    int!(81);
}

pub unsafe fn enable() {
    x86::shared::irq::enable();
}

pub unsafe fn disable() {
    x86::shared::irq::disable();
}

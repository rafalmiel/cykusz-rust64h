use core::ptr;
use core::mem::size_of;
use x86;

extern "C" {
    static interrupt_handlers: [*const u8; 256];
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
struct IdtDescriptor {
    offset_low: u16,
    selector: u16,
    zero: u8,
    type_and_attr: u8,
    offset_high: u64,
    zero2: u16,
}

trait TablePointer {
    fn setup_table(&mut self, table: &[IdtDescriptor; 256]);
}

impl TablePointer for x86::dtables::DescriptorTablePointer {
    fn setup_table(&mut self, table: &[IdtDescriptor; 256]) {
        self.limit = (size_of::<IdtDescriptor>() * 256) as u16;
        self.base = table as *const _ as u64;
    }
}

pub struct Idt {
    table: [IdtDescriptor; 256],
    pointer: x86::dtables::DescriptorTablePointer,
}

impl Idt {
    pub const fn new() -> Idt {
        Idt {
            table: [IdtDescriptor {
                offset_low: 0,
                selector: 0,
                zero: 0,
                type_and_attr: 0,
                offset_high: 0,
                zero2: 0,
            }; 256],
            pointer: x86::dtables::DescriptorTablePointer {
                limit: 0,
                base: 0,
            },
        }
    }

    pub fn init(&mut self) {
        self.pointer.setup_table(&self.table);
        self.setup_gates();

        println!("pointer limit: {}", self.pointer.limit);
        println!("pointer base: 0x{:x}", self.pointer.base);
        println!("pointer itself 0x{:x}", (&self.pointer as *const _ as u64));


        unsafe {
            x86::dtables::lidt(&self.pointer);
        }
    }

    fn setup_gates(&mut self) {
        unsafe {
            for (index, &handler) in interrupt_handlers.iter().enumerate() {
                if handler != ptr::null() {
                    self.set_gate(x86::segmentation::cs(), 0b1000_1110, index, handler);
                }
            }
        }
    }

    fn set_gate(&mut self, gdt_code_selector: x86::segmentation::SegmentSelector, flags: u8, num: usize, handler: *const u8) {
        let e: &mut IdtDescriptor = &mut self.table[num];

        e.offset_low = ((handler as u64) & 0xFFFF) as u16;
        e.offset_high = (handler as u64) >> 16;

        e.selector = gdt_code_selector.bits();
        e.type_and_attr = flags;
    }
}

pub unsafe fn test() {
    int!(80);
}

pub unsafe fn enable() {
    x86::irq::disable();
}

pub unsafe fn disable() {
    x86::irq::enable();
}

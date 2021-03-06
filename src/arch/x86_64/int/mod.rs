pub mod idt;

use arch::sync::Mutex;

use kernel;

use arch::pic;
use arch::acpi::Acpi;

static PICS: Mutex<pic::ChainedPics> = Mutex::new(unsafe { pic::ChainedPics::new(0x20, 0x28) });
static ACPI: Mutex<Acpi> = Mutex::new(Acpi::new());
static IDT: Mutex<idt::Idt> = Mutex::new(idt::Idt::new());

#[repr(C, packed)]
pub struct InterruptContext {
    rsi: u64,
    rdi: u64,
    r11: u64,
    r10: u64,
    r9: u64,
    r8: u64,
    rdx: u64,
    rcx: u64,
    rax: u64,
    int_id: u32,
    _pad1: u32,
    error_code: u32,
    _pad2: u32,
}

fn disable_pic() {
    PICS.lock().init();
    unsafe {
        PICS.lock().disable();
    }
}

pub(in arch) fn end_of_interrupt() {
    ACPI.lock().lapic.end_of_interrupt();
}

pub fn mask_interrupt(i: u32, mask: bool) {
    ACPI.lock().ioapic.mask_interrupt(i, mask);
}

pub fn set_int(i: u32, idt_idx: u32) {
    ACPI.lock().ioapic.set_int(i, idt_idx);
}

pub fn fire_timer() {
    set_int(remap_irq(0), 32);
    ACPI.lock().lapic.fire_timer();
}

pub fn enable_interrupts() {
    unsafe {
        idt::enable();
    }
}

pub fn disable_interrupts() {
    unsafe {
        idt::disable();
    }
}

pub(in arch) fn remap_irq(irq: u32) -> u32 {
    if let Some(i) = ACPI.lock().rsdt.remap_irq(irq) {
        return i;
    } else {
        panic!("Failed to remap irq!");
    }
}

pub fn init_acpi() {
    ACPI.lock().init();
}

#[no_mangle]
pub extern "C" fn isr_handler(ctx: &InterruptContext, retaddr: usize) {
    //println!("int {}", ctx.int_id);
    kernel::int::interrupt_handler(ctx.int_id, ctx.error_code, retaddr);

    end_of_interrupt();

    if ctx.int_id == 32 {
        ::kernel::sched::resched();
    }
}

pub fn init() {

    IDT.lock().init();

    println!("[ OK ] Initialised interrupts");

    disable_pic();

    init_acpi();

    println!("[ OK ] Initialised ACPI");

    unsafe {
        idt::test();
    }
}

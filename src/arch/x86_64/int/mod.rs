pub mod idt;

use arch::sync::Mutex;
use arch::pic;
use arch::apic::Acpi;
use arch::task::resched;

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

pub fn disable_pic() {
    unsafe {
        PICS.lock().disable();
    }
}

pub fn end_of_interrupt() {
    disable_interrupts();
    ACPI.lock().lapic.end_of_interrupt();
    enable_interrupts();
}

pub fn mask_interrupt(i: u32, mask: bool) {
    ACPI.lock().ioapic.mask_interrupt(i, mask);
}

pub fn set_int(i: u32, idt_idx: u32) {
    ACPI.lock().ioapic.set_int(i, idt_idx);
}

pub fn fire_timer() {
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

pub fn remap_irq(irq: u32) -> u32 {
    if let Some(i) = ACPI.lock().rsdt.remap_irq(irq) {
        return i;
    } else {
        panic!("Failed to remap irq!");
    }
}

pub fn init_acpi() {
    println!("Initializing acpi");
    ACPI.lock().init();
}

#[no_mangle]
pub extern "C" fn isr_handler(ctx: &InterruptContext) {
    //println!("int {}", ctx.int_id);
    match ctx.int_id {
        80 => {
            unsafe {
                asm!("xchg %bx, %bx");
            }
            println!("SYSCALL FROM USERSPACE");

        }
        33 => println!("Keyboard interrupt detected"),
        13 => {
            println!("GPF");
            disable_interrupts();
            loop{}
        }
        14 => {
            println!("PAGE FAULT 0x{:x}", ctx.error_code);
            unsafe {
                asm!("xchg %bx, %bx");
            }
            loop{};
        },
        _ => {
            //println!("OTHER INTERRUPT {}", ctx.int_id);
            //loop{};
        }
    }

    end_of_interrupt();

    if ctx.int_id == 32 {
        unsafe {
            //println!("INT TIMER!");
            asm!("xchg %bx, %bx");
        }
        //return;
        resched();
    }
}

pub fn init() {

    IDT.lock().init();

    PICS.lock().init();

    disable_pic();

    init_acpi();
}

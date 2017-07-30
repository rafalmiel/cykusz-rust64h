use arch::int;

pub fn mask_interrupt(i: u32, mask: bool) {
    int::mask_interrupt(i, mask);
}

pub fn set_int(i: u32, idt_idx: u32) {
    int::set_int(i, idt_idx);
}

pub fn fire_timer() {
    int::fire_timer();
}

pub fn enable_interrupts() {
    int::enable_interrupts();
}

pub fn disable_interrupts() {
    int::disable_interrupts();
}

pub fn interrupt_handler(int_id: u32, error_code: u32, retaddr: usize) {
    match int_id {
        80 => {
            println!("[ TASK ] System call from user space");

        }
        81 => {
            println!("[ OK ] Interrupt test passes");

        },
        33 => println!("Keyboard interrupt detected"),
        13 => {
            println!("GPF 0x{:x} err: 0x{:x}", retaddr, error_code);
            disable_interrupts();
            loop{}
        }
        14 => {
            println!("PAGE FAULT 0x{:x}, addr: 0x{:x}", error_code, retaddr);
            loop{};
        },
        _ => {
            //println!("OTHER INTERRUPT {}", ctx.int_id);
            //loop{};
        }
    }
}

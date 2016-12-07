mod rsdp;
mod rsdt;
mod util;

use spin::Mutex;

use arch::acpi::rsdp::Rsdp;
use arch::acpi::rsdt::Rsdt;
use arch::mm::PhysAddr;

pub struct Acpi {
    rsdp: Option<&'static Rsdp>,
    rsdt: Rsdt
}

impl Acpi {
    pub const fn new() -> Acpi {
        Acpi { rsdp: None, rsdt: Rsdt::new() }
    }

    pub fn init(&mut self) {
        unsafe {
            self.rsdp = Rsdp::find();

            if let Some(r) = self.rsdp {
                println!("Found RSDT address! 0x{:x}", r.rsdt_address);

                self.rsdt.init(r.rsdt_address as PhysAddr);
            }
        }
    }
}

static ACPI: Mutex<Acpi> = Mutex::new(Acpi::new());

pub fn init() {
    println!("Initializing acpi");
    ACPI.lock().init();
}

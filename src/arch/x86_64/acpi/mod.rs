mod rsdp;
mod util;

use spin::Mutex;

use arch::acpi::rsdp::Rsdp;

pub struct Acpi {
    rsdp: Option<&'static Rsdp>,
}

impl Acpi {
    pub const fn new() -> Acpi {
        Acpi { rsdp: None }
    }

    pub fn init(&mut self) {
        unsafe {
            self.rsdp = Rsdp::find();

            if let Some(r) = self.rsdp {
                println!("Found RSDT address! 0x{:x}", r.rsdt_address);
            }
        }
    }
}

static ACPI: Mutex<Acpi> = Mutex::new(Acpi::new());

pub fn init() {
    println!("Initializing acpi");
    ACPI.lock().init();
}

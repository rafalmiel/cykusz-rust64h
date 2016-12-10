mod rsdp;
mod rsdt;
mod util;
mod lapic;

use spin::Mutex;

use arch::apic::rsdp::Rsdp;
use arch::apic::rsdt::Rsdt;
use arch::apic::lapic::LApic;
use arch::mm::PhysAddr;

use arch::mm::phys_to_physmap;

pub struct Acpi {
    rsdp: Option<&'static Rsdp>,
    rsdt: Rsdt,
    lapic: LApic,
}

impl Acpi {
    pub const fn new() -> Acpi {
        Acpi { rsdp: None, rsdt: Rsdt::new(), lapic: LApic::new() }
    }

    pub fn init(&mut self) {
        unsafe {
            self.rsdp = Rsdp::find();

            if let Some(r) = self.rsdp {
                println!("Found RSDT address! 0x{:x}", r.rsdt_address);

                self.rsdt.init(phys_to_physmap(r.rsdt_address as PhysAddr));

                if let Some(lapic_base) = self.rsdt.local_controller_address() {
                    self.lapic.init(lapic_base);
                    println!("LApic initialised!");
                }
            }
        }
    }
}

static ACPI: Mutex<Acpi> = Mutex::new(Acpi::new());

pub fn init() {
    println!("Initializing acpi");
    ACPI.lock().init();
}

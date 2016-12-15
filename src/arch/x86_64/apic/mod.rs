mod rsdp;
mod rsdt;
mod util;
mod lapic;
mod ioapic;

use spin::Mutex;

use arch::apic::rsdp::Rsdp;
use arch::apic::rsdt::Rsdt;
use arch::apic::lapic::LApic;
use arch::apic::ioapic::IOApic;
use arch::mm::PhysAddr;

use arch::mm::phys_to_physmap;

pub struct Acpi {
    rsdp: Option<&'static Rsdp>,
    rsdt: Rsdt,
    lapic: LApic,
    ioapic: IOApic
}

impl Acpi {
    pub const fn new() -> Acpi {
        Acpi { rsdp: None, rsdt: Rsdt::new(), lapic: LApic::new(), ioapic: IOApic::new() }
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

                    if let Some(ioapic_base) = self.rsdt.ioapic_address() {
                        self.ioapic.init(ioapic_base);
                        println!("IOApic initialised!");

                        println!("IOAPIC ID: {} IDENT: {}", self.ioapic.id(), self.ioapic.identification());
                        println!("IOAPIC ENTRIES: {} VERSION: {}", self.ioapic.max_red_entries(), self.ioapic.version());
                    }
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

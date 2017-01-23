mod rsdp;
mod rsdt;
mod util;
mod lapic;
mod ioapic;

use arch::apic::rsdp::Rsdp;
use arch::apic::rsdt::Rsdt;
use arch::apic::lapic::LApic;
use arch::apic::ioapic::IOApic;
use arch::mm::PhysAddr;

use arch::mm::phys_to_physmap;

pub struct Acpi {
    rsdp: Option<&'static Rsdp>,
    pub rsdt: Rsdt,
    pub lapic: LApic,
    pub ioapic: IOApic
}

impl Acpi {
    pub const fn new() -> Acpi {
        Acpi { rsdp: None, rsdt: Rsdt::new(), lapic: LApic::new(), ioapic: IOApic::new() }
    }

    pub fn init(&mut self) {
        unsafe {
            self.rsdp = Rsdp::find();

            if let Some(r) = self.rsdp {
                self.rsdt.init(phys_to_physmap(r.rsdt_address as PhysAddr));

                if let Some(lapic_base) = self.rsdt.local_controller_address() {
                    self.lapic.init(lapic_base);
                    println!("LApic initialised!");

                    if let Some(ioapic_base) = self.rsdt.ioapic_address() {
                        self.ioapic.init(ioapic_base);
                        println!("IOApic initialised! id: {}, ident: {}, entries: {}, version: {}",
                            self.ioapic.id(), self.ioapic.identification(),
                            self.ioapic.max_red_entry() + 1, self.ioapic.version());
                    }
                }
            }
        }
    }
}

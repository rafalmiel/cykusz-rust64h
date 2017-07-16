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
    pub rsdt: Rsdt,
    pub lapic: LApic,
    pub ioapic: IOApic
}

impl Acpi {
    pub const fn new() -> Acpi {
        Acpi { rsdt: Rsdt::new(), lapic: LApic::new(), ioapic: IOApic::new() }
    }

    pub fn init(&mut self) {
        unsafe {
            let rsdp = Rsdp::find().expect("RSDP Not found!");

            println!("Found Rsdp!");

            self.rsdt.init(phys_to_physmap(rsdp.rsdt_address as PhysAddr));

            let lapic_base = self.rsdt.local_controller_address().expect("LAPIC address not found!");

            println!("Initialised lapic_base");

            self.lapic.init(lapic_base);

            let ioapic_base = self.rsdt.ioapic_address().expect("IOApic address not found!");

            println!("Initialised ioapic_base!");

            self.ioapic.init(ioapic_base);

            println!("IOApic initialised! id: {}, ident: {}, entries: {}, version: {}",
                            self.ioapic.id(), self.ioapic.identification(),
                            self.ioapic.max_red_entry() + 1, self.ioapic.version());
        }
    }
}

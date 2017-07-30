mod rsdp;
mod rsdt;
mod util;
mod lapic;
mod ioapic;

use arch::acpi::rsdp::Rsdp;
use arch::acpi::rsdt::Rsdt;
use arch::acpi::lapic::LApic;
use arch::acpi::ioapic::IOApic;
use kernel::mm::*;

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

            self.rsdt.init(PhysAddr(rsdp.rsdt_address as usize).to_mapped());

            let lapic_base = self.rsdt.local_controller_address().expect("LAPIC address not found!");

            // println!("Initialised lapic_base");

            self.lapic.init(lapic_base);

            let ioapic_base = self.rsdt.ioapic_address().expect("IOApic address not found!");

            // println!("Initialised ioapic_base!");

            self.ioapic.init(ioapic_base);

            // println!("IOApic initialised! id: {}, ident: {}, entries: {}, version: {}",
            //                 self.ioapic.id(), self.ioapic.identification(),
            //                 self.ioapic.max_red_entry() + 1, self.ioapic.version());
        }
    }
}

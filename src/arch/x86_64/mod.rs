pub mod cpuio;
pub mod mm;

use vga;
use mboot2;

use self::mm::phys_to_virt;

#[no_mangle]
pub extern "C" fn x86_64_rust_main(multiboot_addr: u64) {
    vga::clear_screen();

    let mboot_info = unsafe { mboot2::load(phys_to_virt(multiboot_addr)) };

    mm::init(& mboot_info);

    ::rust_main();
}

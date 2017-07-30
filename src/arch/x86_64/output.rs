pub fn clear() {
    ::drivers::video::vga::clear_screen();
}

pub fn write_fmt(args: ::core::fmt::Arguments) -> ::core::fmt::Result {
    ::core::fmt::write(&mut *::drivers::video::vga::WRITER.lock(), args)
}
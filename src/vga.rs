use core::ptr::Unique;
use spin::Mutex;

use arch::cpuio::Port;

macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

macro_rules! print {
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        $crate::vga::WRITER.lock().write_fmt(format_args!($($arg)*)).unwrap();
    });
}

#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Clone, Copy)]
struct ColorCode(u8);

impl ColorCode {
    const fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
struct ScreenChar {
    char: u8,
    color: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

static CURSOR_INDEX: Mutex<Port<u8>> = Mutex::new(unsafe { Port::new(0x3D4) });

static CURSOR_DATA: Mutex<Port<u8>> = Mutex::new(unsafe { Port::new(0x3D5) });

pub static WRITER: Mutex<Writer> = Mutex::new(Writer {
    column: 0,
    row: 0,
    color: ColorCode::new(Color::LightGreen, Color::Black),
    buffer: unsafe { Unique::new(0xb8000 as *mut _) },
});

struct Buffer {
    chars: [ScreenChar; BUFFER_WIDTH * BUFFER_HEIGHT],
}

pub struct Writer {
    column: usize,
    row: usize,
    color: ColorCode,
    buffer: Unique<Buffer>,
}

fn mk_scr_char(c: u8, clr: ColorCode) -> ScreenChar {
    ScreenChar {
        char: c,
        color: clr,
    }
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                let row = self.row;
                let col = self.column;

                self.buffer().chars[row * BUFFER_WIDTH + col] = mk_scr_char(byte, self.color);
                self.column += 1;
            }
        }

        self.scroll();
    }

    fn update_cursor(&mut self) {
        let position: u16 = (BUFFER_WIDTH * self.row + self.column) as u16;

        CURSOR_INDEX.lock().write(0x0F);
        CURSOR_DATA.lock().write((position & 0xFF) as u8);

        CURSOR_INDEX.lock().write(0x0E);
        CURSOR_DATA.lock().write((position >> 8) as u8);
    }

    fn buffer(&mut self) -> &mut Buffer {
        unsafe { self.buffer.get_mut() }
    }

    fn scroll(&mut self) {
        if self.row > BUFFER_HEIGHT - 1 {
            let blank = mk_scr_char(b' ', self.color);

            {
                let buffer = self.buffer();
                for i in 0..((BUFFER_HEIGHT - 1) * (BUFFER_WIDTH)) {
                    buffer.chars[i] = buffer.chars[i + BUFFER_WIDTH];
                }

                for i in ((BUFFER_HEIGHT - 1) * (BUFFER_WIDTH))..(BUFFER_HEIGHT * BUFFER_WIDTH) {

                    buffer.chars[i] = blank;
                }
            }

            self.row = BUFFER_HEIGHT - 1;
        }
    }

    fn new_line(&mut self) {
        self.column = 0;
        self.row += 1;
    }

    fn clear(&mut self) {
        let blank = mk_scr_char(b' ', self.color);

        for i in 0..(BUFFER_HEIGHT * BUFFER_WIDTH) {
            self.buffer().chars[i] = blank;
        }

        self.update_cursor();
    }

    fn clear_row(&mut self) {
        let blank = mk_scr_char(b' ', self.color);
        let row = self.row;

        for i in (row * BUFFER_WIDTH)..(row * BUFFER_WIDTH + BUFFER_WIDTH) {
            self.buffer().chars[i] = blank;
        }
    }

    pub fn write_str(&mut self, s: &str) {
        for byte in s.bytes() {
            self.write_byte(byte)
        }
        self.scroll();
        self.update_cursor();
    }
}

impl ::core::fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> ::core::fmt::Result {
        for byte in s.bytes() {
            self.write_byte(byte)
        }

        self.update_cursor();

        Ok(())
    }
}

pub fn clear_screen() {
    WRITER.lock().clear();
}

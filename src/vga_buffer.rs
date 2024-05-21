#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
// can be initialized via
// let blue = Color::Blue;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
// newtype tuple struct that wraps around unsigned 8bit int
struct ColorCode(u8);

impl ColorCode {
    // custom constructor fn
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

// repr(C) because rust doesn't have a default field ordering 
// default structs
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

use volatile::Volatile;

// transparent to make each entry in the array 
// the same structure as ScreenChar
#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

use core::fmt;

impl Writer {
    // always write to the LAST line and shift up when full
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code,
                });
                self.column_position += 1;
            }
        }
    }

    // shifts all the lines up by 1 (discards row 0) and empties 
    // bottom line (to clear it!)
    fn new_line(&mut self) {
        // go through all rows 
        for row in 1..BUFFER_HEIGHT {
            // go through all chars in that row
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                //shift up
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT -1);
        self.column_position = 0;
    }

    // self explanatory name
    fn clear_row(&mut self, row:usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }

    // write a string using iterating and the write_byte method
    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // printable ASCII byte or newline
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // not part of printable ASCII range
                // will just print a blank square
                _ => self.write_byte(0xfe),
            }
        }
    }
}

// to gain access to rust fomatting macros, so we can print other types
// like ints or floats, and instead of impl a fn for each type
impl fmt::Write for Writer {
    // this name cannot be changed because we are impl for core
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

// // no longer relevant with the impl of the public static writer
// pub fn print_something() {
//     use core::fmt::Write;
//     let mut writer = Writer {
//         column_position: 0,
//         color_code: ColorCode::new(Color::Yellow, Color::Black),
//         buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
//     };
// 
//     writer.write_byte(b'H');
//     writer.write_string("ello! ");
//     write!(writer, "The numbers are {} and {}", 42, 1.0/3.0).unwrap();
// }

use lazy_static::lazy_static;
use spin::Mutex;

// is initialized when accessed for the first time (at runtime) 
// instead of at compile time... blacc magicc??
lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

// However this will mean that the println! macro will default to printing 
// to vga buffer (which might not be desirable later?)


// $crate:: will allow for the print! macro to use vga_buffer::_print
// even without importing vga_buffer::_print directly
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

// same for reasoning for $crate::print! 
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

// will appear hidden from the generated documentation
#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}

#![allow(dead_code)]
use core::fmt::{Arguments, Write};
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::White, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
        default_color: ColorCode::new(Color::White, Color::Black),
    });
}

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> Self {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }

    fn with_foreground(&self, foreground: Color) -> Self {
        let bg = self.0 & 0xf0;
        let val = bg | (foreground as u8);
        ColorCode(val)
    }

    fn with_background(&self, background: Color) -> Self {
        let fg = self.0 & 0x0f;
        let val = fg | ((background as u8) << 4);
        ColorCode(val)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_char: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    default_color: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
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
                    ascii_char: byte,
                    color_code,
                });
                self.column_position += 1;
            }
        }
    }

    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let ch = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(ch);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_char: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }

    fn set_color(&mut self, color_code: ColorCode) {
        self.color_code = color_code;
    }

    fn reset_color(&mut self) {
        self.color_code = self.default_color;
    }

    fn process_ansi(&mut self, bytes: &mut core::str::Bytes) {
        if let Some(next) = bytes.next() {
            if next != b'[' {
                self.write_byte(next);
                return;
            }
        } else {
            return;
        }
        const BUF_SIZE: usize = 3;
        let mut buf: [u8; BUF_SIZE] = [0; BUF_SIZE];
        let mut index = 0;
        let mut valid = false;
        for next in bytes.by_ref() {
            if (b'0'..=b'9').contains(&next) && index < buf.len() {
                buf[index] = next - b'0';
            } else if next == b'm' {
                // next is not numeric || index == buf.len()
                valid = true;
                break;
            } else {
                // next is not numeric || index == buf.len() && next != ';'
                self.write_byte(b'[');
                for i in buf.iter().take(index) {
                    self.write_byte(i + b'0');
                }
                self.write_byte(next);
                return;
            }
            index += 1;
        }

        if !valid {
            self.write_byte(b'[');
            for i in buf.iter().take(index) {
                self.write_byte(i + b'0');
            }
            return;
        }

        let mut num = 0;
        let mut factor = 1;
        for (i, j) in buf.iter().rev().enumerate() {
            if i < BUF_SIZE - index {
                continue;
            }
            num += *j * factor;
            factor = factor.overflowing_mul(10).0;
        }

        match num {
            0 => self.set_color(self.default_color),
            30 => self.set_color(self.color_code.with_foreground(Color::Black)),
            31 => self.set_color(self.color_code.with_foreground(Color::Red)),
            32 => self.set_color(self.color_code.with_foreground(Color::Green)),
            33 => self.set_color(self.color_code.with_foreground(Color::Yellow)),
            34 => self.set_color(self.color_code.with_foreground(Color::Blue)),
            35 => self.set_color(self.color_code.with_foreground(Color::Magenta)),
            36 => self.set_color(self.color_code.with_foreground(Color::Cyan)),
            37 => self.set_color(self.color_code.with_foreground(Color::White)),
            90 => self.set_color(self.color_code.with_foreground(Color::LightGray)),
            91 => self.set_color(self.color_code.with_foreground(Color::LightRed)),
            92 => self.set_color(self.color_code.with_foreground(Color::LightGreen)),
            93 => self.set_color(self.color_code.with_foreground(Color::Yellow)),
            94 => self.set_color(self.color_code.with_foreground(Color::LightBlue)),
            95 => self.set_color(self.color_code.with_foreground(Color::Magenta)),
            96 => self.set_color(self.color_code.with_foreground(Color::LightCyan)),
            97 => self.set_color(self.color_code.with_foreground(Color::White)),
            40 => self.set_color(self.color_code.with_background(Color::Black)),
            41 => self.set_color(self.color_code.with_background(Color::Red)),
            42 => self.set_color(self.color_code.with_background(Color::Green)),
            43 => self.set_color(self.color_code.with_background(Color::Yellow)),
            44 => self.set_color(self.color_code.with_background(Color::Blue)),
            45 => self.set_color(self.color_code.with_background(Color::Magenta)),
            46 => self.set_color(self.color_code.with_background(Color::Cyan)),
            47 => self.set_color(self.color_code.with_background(Color::White)),
            100 => self.set_color(self.color_code.with_background(Color::LightGray)),
            101 => self.set_color(self.color_code.with_background(Color::LightRed)),
            102 => self.set_color(self.color_code.with_background(Color::LightGreen)),
            103 => self.set_color(self.color_code.with_background(Color::Yellow)),
            104 => self.set_color(self.color_code.with_background(Color::LightBlue)),
            105 => self.set_color(self.color_code.with_background(Color::Magenta)),
            106 => self.set_color(self.color_code.with_background(Color::LightCyan)),
            107 => self.set_color(self.color_code.with_background(Color::White)),
            _ => self.write_byte(b'x'),
        }
    }

    fn itos(&mut self, mut num: u8) {
        let mut buf = [0; 3];
        let mut i = 2;
        while num != 0 {
            buf[i] = num % 10;
            num /= 10;
            i = i.overflowing_sub(1).0;
        }
        for i in buf {
            self.write_byte(i + b'0');
        }
    }
}

impl Write for Writer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let mut bytes = s.bytes();
        while let Some(byte) = bytes.next() {
            match byte {
                0x1b => {
                    self.process_ansi(&mut bytes);
                }
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(b'?'),
            }
        }
        Ok(())
    }
}

// print macros
// stolen bc macros is some mad magic
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: Arguments) {
    x86_64::instructions::interrupts::without_interrupts(|| {
        WRITER.lock().write_fmt(args).unwrap();
    });
}

#[doc(hidden)]
pub fn _eprint(args: Arguments) {
    WRITER
        .lock()
        .set_color(ColorCode::new(Color::Red, Color::Black));
    WRITER.lock().write_fmt(args).unwrap();
    WRITER.lock().reset_color();
}

#[macro_export]
macro_rules! eprint {
    ($($arg:tt)*) => ($crate::vga_buffer::_eprint(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! eprintln {
    () => ($crate::eprint!("\n"));
    ($($arg:tt)*) => ($crate::eprint!("{}\n", format_args!($($arg)*)));
}

#![allow(unused)]

use core::{cmp::min, fmt, ptr::NonNull};

use conquer_once::spin::OnceCell;
use spin::Mutex;
use volatile::{VolatilePtr, VolatileRef};

use crate::serial_println;

/// The location in memory of the VGA buffer.
///
/// The buffer is 2D with 25 rows and 80 columns.
const VGA_BUFFER: *mut u16 = 0xb8000 as *mut u16;

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

/// The default foreground color.
pub const DEFAULT_FG: Color = Color::LightGreen;

/// The default background color.
pub const DEFAULT_BG: Color = Color::Black;

// pub const DEFAULT_COLOR: ColorCode = ColorCode::new(Color::White, Color::Black);

pub static WRITER: OnceCell<Mutex<Writer>> = OnceCell::uninit();

pub fn init_writer() -> Mutex<Writer> {
    Mutex::new(Writer::new())
}

/// A VGA character. Note: use `.into::<u16>()`
/// to write to a VGA buffer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct VgaChar {
    /// An ASCII character.
    ch: u8,

    /// The foreground [Color] of this character.
    fg: Color,
    /// The background [Color] of this character.
    bg: Color,

    /// Whether or not the text should blink.
    blink: bool,
}

impl From<VgaChar> for u16 {
    fn from(ch: VgaChar) -> Self {
        (ch.ch as u16) | ((ch.fg as u16) << 8) | ((ch.bg as u16) << 12) | ((ch.blink as u16) << 15)
    }
}

impl From<char> for VgaChar {
    fn from(ch: char) -> Self {
        Self {
            ch: if ch.is_ascii() { ch as u8 } else { 0x13 },
            fg: DEFAULT_FG,
            bg: DEFAULT_BG,
            blink: false,
        }
    }
}

impl From<u8> for VgaChar {
    fn from(ch: u8) -> Self {
        Self {
            ch,
            fg: DEFAULT_FG,
            bg: DEFAULT_BG,
            blink: false,
        }
    }
}

impl From<u16> for VgaChar {
    fn from(word: u16) -> Self {
        /*
        (ch.ch as u16) |
        ((ch.fg as u16) << 8) |
        ((ch.bg as u16) << 12) |
        ((ch.blink as u16) << 15)
         */
        let ch = (word & 0xff) as u8;
        let fg = ((word >> 8) as u8 & 0xf).into();
        let bg = ((word >> 12) as u8 & 0xf).into();
        let blink = word & (1 << 15) != 0;

        Self { ch, fg, bg, blink }
    }
}

/// A VGA color.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
#[allow(dead_code)]
pub enum Color {
    Black = 0,
    Blue,
    Green,
    Cyan,
    Red,
    Magenta,
    Brown,

    LightGray,
    DarkGray,
    LightBlue,
    LightGreen,
    LightCyan,
    LightRed,
    Pink,
    Yellow,
    White,
}

impl From<u8> for Color {
    fn from(value: u8) -> Self {
        match value & 0b1111 {
            0 => Self::Black,
            1 => Self::Blue,
            2 => Self::Green,
            3 => Self::Cyan,
            4 => Self::Red,
            5 => Self::Magenta,
            6 => Self::Brown,
            7 => Self::LightGray,
            8 => Self::DarkGray,
            9 => Self::LightBlue,
            10 => Self::LightGreen,
            11 => Self::LightCyan,
            12 => Self::LightRed,
            13 => Self::Pink,
            14 => Self::Yellow,
            15 => Self::White,
            _ => unreachable!(),
        }
    }
}

pub trait AsciiWrite {
    /// Write a single ASCII byte.
    fn write_byte(&mut self, byte: u8);

    /// Flush the output.
    ///
    /// This may just mean adding a newline.
    fn flush(&mut self);

    /// Write a single UTF-8 character,
    /// substituting a default for non-ASCII
    /// characters.
    fn write_char(&mut self, ch: char) {
        self.write_byte(ch as u8);
    }

    /// Write an ASCII string. Note: UTF-8
    /// will be interpreted as malformed ASCII.
    fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            self.write_byte(byte);
        }
    }
}

// /// A [`print`]/[`println`] compatible escape sequence.
// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub enum EscapeCode {
//     SetFg(Color),
//     SetBg(Color),
//     SetBlink,
//     UnsetBlink,
//     Reset,
//     Nbsp,
// }

/// A wrapper around the VGA buffer.
pub struct Writer {
    column: usize,
    fg: Color,
    bg: Color,
    blink: bool,
    buffer: &'static mut u16,
}

impl AsciiWrite for Writer {
    fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.flush(),
            b'\t' => {
                if self.column >= BUFFER_WIDTH {
                    self.flush();
                } else {
                    self.column += min(BUFFER_WIDTH - self.column, 4);
                }
            }
            byte => {
                if self.column >= BUFFER_WIDTH {
                    self.flush();
                }

                let row = BUFFER_HEIGHT - 1;
                self.set_cell(
                    self.column,
                    row,
                    VgaChar {
                        ch: byte,
                        fg: self.fg,
                        bg: self.bg,
                        blink: false,
                    },
                );

                self.column += 1;
            }
        }
    }

    /// Write a string to VGA output.
    fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            self.write_byte(byte);
        }
    }

    fn flush(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let ch = self.get_cell(col, row).unwrap();
                self.set_cell(col, row - 1, ch);
            }
        }

        self.clear_row(BUFFER_HEIGHT - 1);
        self.column = 0;
    }
}

impl Writer {
    /// Creates a new Writer over the VGA buffer.
    pub fn new() -> Self {
        let mut me = Self {
            column: 0,
            fg: DEFAULT_FG,
            bg: DEFAULT_BG,
            blink: false,
            buffer: unsafe { &mut *VGA_BUFFER },
        };

        me.clear();

        me
    }

    fn clear_row(&mut self, row: usize) {
        let blank = VgaChar {
            ch: 0,
            fg: self.fg,
            bg: self.bg,
            blink: self.blink,
        }
        .into();

        for col in 0..BUFFER_WIDTH {
            self.set_cell(col, row, blank);
        }
    }

    fn clear(&mut self) {
        for row in 0..BUFFER_HEIGHT {
            self.clear_row(row);
        }

        self.column = 0;
    }

    fn get_cell(&self, x: usize, y: usize) -> Option<VgaChar> {
        if x >= BUFFER_WIDTH || y >= BUFFER_HEIGHT {
            #[cfg(debug_assertions)]
            serial_println!("tried to read outside of vga buffer bounds (from {x}, {y})");

            return None;
        }

        let x = x as isize;
        let y = y as isize;

        let ptr = unsafe {
            (self.buffer as *const u16 as *mut u16).offset(x + (y * BUFFER_WIDTH as isize))
        };
        let volatile = unsafe { VolatilePtr::new(NonNull::new_unchecked(ptr)) };
        Some(volatile.read().into())
    }

    fn set_cell(&mut self, x: usize, y: usize, ch: VgaChar) {
        if x >= BUFFER_WIDTH || y >= BUFFER_HEIGHT {
            #[cfg(debug_assertions)]
            serial_println!("tried to write outside of vga buffer bounds (to {x}, {y})");

            return;
        }

        let x = x as isize;
        let y = y as isize;

        let ptr = unsafe { (self.buffer as *mut u16).offset(x + (y * BUFFER_WIDTH as isize)) };
        let volatile = unsafe { VolatilePtr::new(NonNull::new_unchecked(ptr)) };
        volatile.write(u16::from(ch));
    }
}

impl core::fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

/// Moves the cursor one character the the right,
/// if not already at the end of a line.
pub fn right() {
    let mut writer = WRITER.get_or_init(init_writer).lock();

    if writer.column != BUFFER_WIDTH
        && u16::from(writer.get_cell(writer.column, BUFFER_HEIGHT - 1).unwrap()) & 0xff != 0
    {
        writer.column += 1;
    }
}

/// Moves the cursor one character to the left,
/// if not already at the start of a line.
pub fn left() {
    let mut writer = WRITER.get_or_init(init_writer).lock();

    if writer.column != 0 {
        writer.column -= 1;
    }
}

/// Returns the cursor to the start of the line.
pub fn home() {
    WRITER.get_or_init(init_writer).lock().column = 0;
}

/// Puts the cursor after any printed characters on the line.
pub fn end() {
    let mut writer = WRITER.get_or_init(init_writer).lock();

    for i in 0..BUFFER_WIDTH {
        let ch: u16 = writer.get_cell(i, BUFFER_HEIGHT - 1).unwrap().into();
        if ch & 0xff != 0 {
            if i == BUFFER_WIDTH {
                writer.column = BUFFER_WIDTH;
            } else {
                writer.column = i + 1;
            }

            return;
        }
    }
}

/// Creates a new line.
pub fn enter() {
    WRITER.get_or_init(init_writer).lock().flush();
}

/// Deletes the last character. Doesn't cross lines.
pub fn backspace() {
    let mut writer = WRITER.get_or_init(init_writer).lock();

    if writer.column != 0 {
        let col = writer.column - 1;

        let fg = writer.fg;
        let bg = writer.bg;
        let blink = writer.blink;

        writer.set_cell(
            col,
            BUFFER_HEIGHT - 1,
            VgaChar {
                ch: 0,
                fg,
                bg,
                blink,
            }
            .into(),
        );

        writer.column = col;
    }
}

/// Clears the screen and resets the cursor.
pub fn clear() {
    WRITER.get_or_init(init_writer).lock().clear();
}

/// Sets the VGA blink. Affects future characters.
#[inline]
pub fn set_blink(blink: bool) {
    WRITER.get_or_init(init_writer).lock().blink = blink;
}

/// Sets the foreground color. Affects future characters.
#[inline]
pub fn set_fg(fg: Color) {
    WRITER.get_or_init(init_writer).lock().fg = fg;
}

/// Sets the background color. Affects future characters.
#[inline]
pub fn set_bg(bg: Color) {
    WRITER.get_or_init(init_writer).lock().bg = bg;
}

/// Resets the foreground/background colors and blink.
#[inline]
pub fn reset() {
    let mut writer = WRITER.get_or_init(init_writer).lock();
    writer.blink = false;
    writer.fg = DEFAULT_FG;
    writer.bg = DEFAULT_BG;
}

/// Prints to the VGA buffer.
///
/// Equivalent to the [`println`] macro except
/// that a newline is not printed at the end
/// of the message.
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga::_print(format_args!($($arg)*)));
}

/// Prints to the VGA buffer, with a newline.
///
/// Equivalent to the [`print`] macro except
/// that a newline is printed at the end of
/// the message.
///
/// This macro uses the same syntax as format,
/// but writes to the standard output instead.
/// See [`core::fmt`] for more information.
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

/// Prints to the VGA buffer.
///
/// Please use the [`print`] or [`println`] macros instead.
#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    x86_64::instructions::interrupts::without_interrupts(|| {
        WRITER
            .get_or_init(init_writer)
            .lock()
            .write_fmt(args)
            .unwrap()
    });
}

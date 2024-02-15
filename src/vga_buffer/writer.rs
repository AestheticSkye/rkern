//! System for writing to VGA screen.

use core::fmt;

use lazy_static::lazy_static;
use spin::Mutex;

use super::buffer::{Buffer, ScreenChar, BUFFER_HEIGHT, BUFFER_WIDTH};
use super::color::ColorCode;
use crate::vga_buffer::color::Color;

lazy_static! {
    /// A global `Writer` instance that can be used for printing to the VGA text buffer.
    ///
    /// Used by the `print!` and `println!` macros.
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code:      ColorCode::new(Color::Yellow, Color::Black),
        buffer:          unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

/// A writer type that allows writing ASCII bytes and strings to an underlying `Buffer`.
///
/// Wraps lines at `BUFFER_WIDTH`. Supports newline characters and implements the
/// `core::fmt::Write` trait.
pub struct Writer {
    pub column_position: usize,
    pub color_code:      ColorCode,
    pub buffer:          &'static mut Buffer,
}

impl Writer {
    /// Writes the given ASCII string to the buffer.
    ///
    /// Wraps lines at `BUFFER_WIDTH`. Supports the `\n` newline character. Does **not**
    /// support strings with non-ASCII characters, since they can't be printed in the VGA text
    /// mode.
    pub fn write_string(&mut self, string: &str) {
        string.bytes().for_each(|byte| match byte {
            0x20..=0x7f | b'\n' => self.write_byte(byte),
            _ => self.write_byte(0xfe),
        });
    }

    /// Writes an ASCII byte to the buffer.
    ///
    /// Wraps lines at `BUFFER_WIDTH`. Supports the `\n` newline character
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

    /// Moves the curser back one and removes last character.
    pub fn backspace(&mut self) {
        if self.column_position > 0 {
            self.column_position -= 1;
            self.buffer.chars[BUFFER_HEIGHT - 1][self.column_position].write(ScreenChar {
                ascii_character: b' ',
                color_code:      self.color_code,
            });
        }
    }

    /// Clears the entire screen.
    pub fn clear(&mut self) { (0..BUFFER_HEIGHT).for_each(|row| self.clear_row(row)) }

    /// Shifts all lines one line up and clears the last row.
    fn new_line(&mut self) {
        (1..BUFFER_HEIGHT).for_each(|row| {
            (0..BUFFER_WIDTH).for_each(|col| {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            });
        });
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    /// Clears a row by overwriting it with blank characters.
    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code:      self.color_code,
        };
        (0..BUFFER_WIDTH).for_each(|col| self.buffer.chars[row][col].write(blank));
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

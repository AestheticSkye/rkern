//! Interaction with the VGA display buffer.
//!
//! Todo: Implement a system for userspace programs to
//! be able to draw their own things to screen

mod buffer;
mod color;
pub mod print;
mod writer;

use x86_64::instructions::interrupts;

use self::writer::WRITER;

/// Moves the curser back one and removes last character.
pub fn backspace() { interrupts::without_interrupts(|| WRITER.lock().backspace()); }

/// Clears the entire screen.
pub fn clear() { interrupts::without_interrupts(|| WRITER.lock().clear()); }

#[test_case]
fn test_println_output() {
    use core::fmt::Write;

    use x86_64::instructions::interrupts;

    use crate::vga_buffer::buffer::BUFFER_HEIGHT;
    use crate::vga_buffer::writer::WRITER;

    let s = "Some test string that fits on a single line";
    interrupts::without_interrupts(|| {
        let mut writer = WRITER.lock();
        writeln!(writer, "\n{}", s).expect("writeln failed");
        for (i, c) in s.chars().enumerate() {
            let screen_char = writer.buffer.chars[BUFFER_HEIGHT - 2][i].read();
            assert_eq!(char::from(screen_char.ascii_character), c);
        }
    });
}

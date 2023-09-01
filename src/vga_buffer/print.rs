//! `print` & `println` macros for the VGA buffer.

use x86_64::instructions::interrupts;

use crate::vga_buffer::writer::WRITER;

/// Prints text to the VGA buffer.
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::print::_print(format_args!($($arg)*)));
}

/// Prints text to the VGA buffer, appending new line.
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
	use core::fmt::Write;

	interrupts::without_interrupts(|| {
		WRITER.lock().write_fmt(args).unwrap();
	});
}

//! Interaction with the VGA display buffer.

mod buffer;
mod color;
pub mod print;
mod writer;

#[cfg(test)]
mod test {
	use crate::println;
	use crate::vga_buffer::buffer::BUFFER_HEIGHT;
	use crate::vga_buffer::writer::WRITER;

	#[test_case]
	fn test_println_output() {
		use core::fmt::Write;

		use x86_64::instructions::interrupts;

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
}

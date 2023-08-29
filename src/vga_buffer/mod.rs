/// VGA screen buffer
mod buffer;
/// Color codes for text
mod color;
/// `print` & `println` macros
pub mod print;
/// System for writing to VGA screen
mod writer;

#[cfg(test)]
mod test {
	use crate::println;
	use crate::vga_buffer::buffer::BUFFER_HEIGHT;
	use crate::vga_buffer::writer::WRITER;

	#[test_case]
	fn test_println_output() {
		let s = "Some test string that fits on a single line";
		println!("{}", s);
		for (i, c) in s.chars().enumerate() {
			let screen_char = WRITER.lock().buffer.chars[BUFFER_HEIGHT - 2][i].read();
			assert_eq!(char::from(screen_char.ascii_character), c);
		}
	}
}
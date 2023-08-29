/// VGA screen buffer
mod buffer;
/// Color codes for text
mod color;
/// `print` & `println` macros
pub mod print;
/// System for writing to VGA screen
mod writer;

use lazy_static::lazy_static;
use spin::Mutex;

use self::color::ColorCode;
use crate::vga_buffer::buffer::Buffer;
use crate::vga_buffer::color::Color;
use crate::vga_buffer::writer::Writer;

lazy_static! {
	pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
		column_position: 0,
		color_code:      ColorCode::new(Color::Yellow, Color::Black),
		buffer:          unsafe { &mut *(0xb8000 as *mut Buffer) },
	});
}

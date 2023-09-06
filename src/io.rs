//! Basic stuff for input output, not designed to be a final product.

use alloc::string::{String, ToString};
use alloc::sync::Arc;

use lazy_static::lazy_static;
use spin::Mutex;
use x86_64::instructions::{hlt, interrupts};

lazy_static! {
	static ref STDIN: Arc<Mutex<String>> = Arc::new(Mutex::new(String::new()));
}

/// Push a single character onto STDIN
pub fn stdin_push(char: char) {
	interrupts::without_interrupts(|| {
		let mut stdin = STDIN.lock();

		stdin.push(char);

		drop(stdin);
	});
}

#[must_use]
pub fn stdin_backspace() -> bool {
	let mut buffer_empty = false;
	interrupts::without_interrupts(|| {
		let mut stdin = STDIN.lock();

		buffer_empty = stdin.pop().is_none();
	});
	!buffer_empty
}

/// Read line from STDIN.
///
/// Returns string up to new line, or waits until newline is received.
pub fn read_line(string: &mut String) {
	let mut running = true;
	while running {
		interrupts::without_interrupts(|| {
			let stdin = STDIN.clone();
			let mut stdin = stdin.lock();

			if let Some(newline_index) = stdin.find('\n') {
				// Grab string up to newline
				*string = stdin[..newline_index].to_string();
				// Set stdin to the remaining string.
				*stdin = stdin[(newline_index + 1)..].to_string();
				running = false;
			}
		});

		hlt();
	}
}

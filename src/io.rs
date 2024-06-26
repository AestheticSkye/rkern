//! Basic stuff for input output, not designed to be a final product.

use alloc::string::{String, ToString};
use alloc::sync::Arc;

use spin::{Lazy, Mutex};
use x86_64::instructions::{hlt, interrupts};

static STDIN: Lazy<Arc<Mutex<String>>> = Lazy::new(|| Arc::new(Mutex::new(String::new())));

/// Push a single character onto STDIN
pub fn stdin_push(char: char) {
    interrupts::without_interrupts(|| {
        let mut stdin = STDIN.lock();

        stdin.push(char);

        drop(stdin);
    });
}

/// Remove a character from the buffer and return it.
#[allow(clippy::must_use_candidate)]
pub fn stdin_backspace() -> Option<char> {
    let mut char = None;
    interrupts::without_interrupts(|| {
        let mut stdin = STDIN.lock();

        char = stdin.pop();
    });
    char
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

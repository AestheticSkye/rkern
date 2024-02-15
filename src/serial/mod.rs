//! Serial components.
//! Used for report back to host during tests.

pub mod print;

use spin::{Lazy, Mutex};
use uart_16550::SerialPort;

/// Internal serial component for for `serial_print` & `serial_println`
static SERIAL1: Lazy<Mutex<SerialPort>> = Lazy::new(|| {
    let mut serial_port = unsafe { SerialPort::new(0x3F8) };
    serial_port.init();
    Mutex::new(serial_port)
});

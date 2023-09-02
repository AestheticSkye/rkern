//! Serial components.
//! Used for report back to host during tests.

pub mod print;

use lazy_static::lazy_static;
use spin::Mutex;
use uart_16550::SerialPort;

lazy_static! {
	/// Internal serial component for for `serial_print` & `serial_println`
	static ref SERIAL1: Mutex<SerialPort> = {
		let mut serial_port = unsafe { SerialPort::new(0x3F8) };
		serial_port.init();
		Mutex::new(serial_port)
	};
}

#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test::test_runner)]
#![reexport_test_harness_main = "test_main"]

/// Serial components.
/// Used for report back to host during tests.
pub mod serial;
/// Systems required for building and running tests.
pub mod test;
/// Direct interaction with the VGA display buffer.
pub mod vga_buffer;

pub use test::{test_panic_handler, test_runner};

pub mod prelude {
	pub use crate::{print, println};
}

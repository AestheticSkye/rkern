#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test::test_runner)]
#![reexport_test_harness_main = "test_main"]

pub mod serial;
pub mod test;
pub mod vga_buffer;

pub use test::{test_panic_handler, test_runner};

/// Core internal components of the kernel.
pub mod prelude {
	pub use crate::{print, println};
}

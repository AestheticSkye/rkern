#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![test_runner(crate::test::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(clippy::module_name_repetitions)]

pub mod gdt;
mod interrupts;
pub mod serial;
pub mod test;
pub mod vga_buffer;

pub use test::{test_panic_handler, test_runner};

/// Core internal components of the kernel.
pub mod prelude {
	pub use crate::{init, print, println};
}

/// Initializes crucial kernel systems.
pub fn init() {
	gdt::init();
	interrupts::init_idt();
}

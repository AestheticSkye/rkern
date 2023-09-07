//! Root for core kernel systems.

#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![feature(const_mut_refs)]
#![test_runner(crate::test::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![warn(missing_docs)]
#![allow(clippy::module_name_repetitions)]

pub mod allocator;
mod fs;
pub mod gdt;
mod interrupts;
pub mod io;
pub mod memory;
pub mod serial;
pub mod shell;
pub mod test;
pub mod vga_buffer;

extern crate alloc;

use bootloader::BootInfo;
pub use test::{test_panic_handler, test_runner};

/// Core internal components of the kernel.
pub mod prelude {
	pub use alloc::boxed::Box;
	pub use alloc::string::String;
	pub use alloc::vec;
	pub use alloc::vec::Vec;

	pub use crate::io::read_line;
	pub use crate::{hlt_loop, init, print, println};
}

/// Initializes crucial kernel systems.
pub fn init(boot_info: &'static BootInfo) {
	gdt::init();
	interrupts::init_idt();
	interrupts::init_pic();
	allocator::init_allocator(boot_info);
}

/// Halts the currently running function.
/// Used to set the system into a sleep state after running.
pub fn hlt_loop() -> ! {
	loop {
		x86_64::instructions::hlt();
	}
}

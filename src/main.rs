#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rkern::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

extern crate alloc;

use core::panic::PanicInfo;

use bootloader::{entry_point, BootInfo};
use rkern::prelude::*;
use rkern::shell::run_shell;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
	init(boot_info);

	println!("Hello World!");

	#[cfg(test)]
	test_main();

	run_shell();

	hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	println!("{}", info);
	hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! { rkern::test_panic_handler(info) }

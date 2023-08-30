#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rkern::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use core::panic::PanicInfo;

use rkern::prelude::*;

#[allow(unconditional_panic)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
	init();

	#[cfg(test)]
	test_main();

	println!("Hello World{}", "!");

	#[allow(clippy::empty_loop)]
	loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	println!("{}", info);
	loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! { rkern::test_panic_handler(info) }

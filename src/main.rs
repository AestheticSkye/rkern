#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rkern::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use rkern::prelude::*;

#[no_mangle]
pub extern "C" fn _start() -> ! {
	println!("Hello World{}", "!");

	#[cfg(test)]
	test_main();

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

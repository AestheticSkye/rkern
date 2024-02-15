//! Systems required for building and running tests.

#![allow(missing_docs)]

use core::panic::PanicInfo;

#[cfg(test)]
use bootloader::{entry_point, BootInfo};

use crate::hlt_loop;
use crate::test::prelude::*;

#[cfg(test)]
entry_point!(test_kernel_main);

/// Core components used for testing.
pub mod prelude {
    pub use crate::test::{exit_qemu, QemuExitCode};
    pub use crate::{serial_print, serial_println};
}

/// Adds test behavior to make any function act like a stdlib test.
pub trait Testable {
    fn run(&self);
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);

    hlt_loop();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed  = 0x11,
}

/// Quits QEMU and returns provided exit code.
pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

#[cfg(test)]
fn test_kernel_main(boot_info: &'static BootInfo) -> ! {
    use crate::init;

    init(boot_info);
    crate::test_main();
    hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! { test_panic_handler(info) }

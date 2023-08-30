//! Interrupt handling.

use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

use crate::println;

lazy_static! {
	/// Global IDT.
	static ref IDT: InterruptDescriptorTable = {
		let mut idt = InterruptDescriptorTable::new();
		idt.breakpoint.set_handler_fn(breakpoint_handler);
		idt
	};
}

/// Initializes the Interrupt Descriptor Table.
/// Called in `rkern::init`.
pub fn init_idt() { IDT.load(); }

/// Handles an interrupt when an it occurs.
extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
	println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

/// Tests to see if the system can recover after an interrupt.
#[test_case]
fn test_breakpoint_exception() {
	// Invoke a breakpoint exception.
	x86_64::instructions::interrupts::int3();
}

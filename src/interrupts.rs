//! Interrupt handling.

mod handlers;
mod pic;

use lazy_static::lazy_static;
use x86_64::structures::idt::InterruptDescriptorTable;

use crate::gdt;
#[allow(clippy::wildcard_imports)]
use crate::interrupts::handlers::*;
use crate::interrupts::pic::{InterruptIndex, PICS};

lazy_static! {
	/// The Interrupt Descriptor Table used to store the different
	/// subroutines for specific exceptions.
	static ref IDT: InterruptDescriptorTable = {
		let mut idt = InterruptDescriptorTable::new();
		idt.breakpoint.set_handler_fn(breakpoint_handler);
		unsafe {
			idt.double_fault.set_handler_fn(double_fault_handler)
				.set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
		}
		idt.page_fault.set_handler_fn(page_fault_handler);
		idt[InterruptIndex::Timer.as_usize()]
			.set_handler_fn(timer_interrupt_handler);
		idt[InterruptIndex::Keyboard.as_usize()]
			.set_handler_fn(keyboard_interrupt_handler);
		idt
	};
}

/// Initializes the Interrupt Descriptor Table.
/// Called in `rkern::init`.
pub fn init_idt() { IDT.load(); }

/// Initializes the Programmable Interrupt Controller.
/// Called in `rkern::init`.
pub fn init_pic() {
	unsafe {
		PICS.lock().initialize();
	}
	x86_64::instructions::interrupts::enable();
}

/// Tests to see if the system can recover after an interrupt.
#[test_case]
fn test_breakpoint_exception() {
	// Invoke a breakpoint exception.
	x86_64::instructions::interrupts::int3();
}

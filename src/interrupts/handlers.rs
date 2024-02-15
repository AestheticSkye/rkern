use x86_64::structures::idt::{InterruptStackFrame, PageFaultErrorCode};

use crate::interrupts::pic::{InterruptIndex, PICS};
use crate::io::{stdin_backspace, stdin_push};
use crate::prelude::*;

/// Handles page fault exceptions.
pub extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    use x86_64::registers::control::Cr2;

    println!("EXCEPTION: PAGE FAULT");
    println!("Accessed Address: {:?}", Cr2::read());
    println!("Error Code: {:?}", error_code);
    println!("{:#?}", stack_frame);
    hlt_loop();
}

/// Handles reading keyboard interrupts.
pub extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    use lazy_static::lazy_static;
    use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
    use spin::Mutex;
    use x86_64::instructions::port::Port;

    lazy_static! {
        static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> =
            Mutex::new(Keyboard::new(
                ScancodeSet1::new(),
                layouts::Us104Key,
                HandleControl::Ignore
            ));
    }

    let mut keyboard = KEYBOARD.lock();
    let mut port = Port::new(0x60);

    let scancode: u8 = unsafe { port.read() };

    // Labbeled scope to allow for breaking out of, to allow for interrupt to be ended.
    'a: {
        let Ok(Some(key_event)) = keyboard.add_byte(scancode) else {
            break 'a;
        };

        let Some(key) = keyboard.process_keyevent(key_event) else {
            break 'a;
        };

        match key {
            DecodedKey::RawKey(_) => {}
            DecodedKey::Unicode(_character @ '\x08') => {
                if stdin_backspace().is_some() {
                    crate::vga_buffer::backspace();
                }
            }
            DecodedKey::Unicode(character) => {
                stdin_push(character);
                print!("{}", character);
            }
        }
    }

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }
}

/// Handles automatic timer interrupts.
pub extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    // print!(".");

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}

/// Handles a breakpoint (INT3) when it occurs.
pub extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

/// Handles double faults when when not
/// picked up by another exception handler.
pub extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

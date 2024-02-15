//! Systems relating to hardware interrupts using the pic 8259.

use pic8259::ChainedPics;

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

/// Primary and secondary Programmable Interrupt Controllers.
///
/// Used to notify the CPU & kernel of interrupts
/// coming from separate hardware components.
pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

/// Index within the PIC the
/// certain interrupt is contained.
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard,
}

impl InterruptIndex {
    pub const fn as_u8(self) -> u8 { self as u8 }

    pub fn as_usize(self) -> usize { usize::from(self.as_u8()) }
}

//! GDD and TSS systems for storing memory for double faults.

use core::ptr::addr_of;

use lazy_static::lazy_static;
use once_cell::sync::Lazy;
use x86_64::structures::gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector};
use x86_64::structures::tss::TaskStateSegment;

/// Index of the stack used by Interrupt Stack Table.
///
/// Defaults to the first out of the 5 possible stacks.
pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

/// Initializes the GDT and TSS.
/// Called in `rkern::init`.
pub fn init() {
    use x86_64::instructions::segmentation::{Segment, CS};
    use x86_64::instructions::tables::load_tss;

    GDT.0.load();
    unsafe {
        CS::set_reg(GDT.1.code_selector);
        load_tss(GDT.1.tss_selector);
    }
}

lazy_static! {
    /// The Global Descriptor Table to hold the TSS & kernel code segment selectors.
    static ref GDT: (GlobalDescriptorTable, Selectors) = {
        let mut gdt = GlobalDescriptorTable::new();
        let code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
        let tss_selector = gdt.add_entry(Descriptor::tss_segment(&TSS));
        (
            gdt,
            Selectors {
                code_selector,
                tss_selector,
            },
        )
    };
}

/// The selectors provided by the GDT
///
/// Used to index into specific components.
struct Selectors {
    /// Kernel code segment
    code_selector: SegmentSelector,
    /// Task State Segment
    tss_selector:  SegmentSelector,
}

lazy_static! {
    /// The TSS holds information necessary for hardware task switching.
    ///
    /// Used for hardware exceptions.
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            const STACK_SIZE: usize = 4096 * 5;
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

            let stack_start = x86_64::VirtAddr::from_ptr(unsafe { addr_of!(STACK) });
            stack_start + STACK_SIZE // stack end
        };
        tss
    };
}

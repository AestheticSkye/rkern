//! Direct interaction with the memory map provided by the kernel.

pub mod frame_allocator;

use x86_64::structures::paging::{OffsetPageTable, PageTable};
use x86_64::VirtAddr;

/// Initialize a new `OffsetPageTable`.
///
/// # Safety
/// This function is unsafe because the caller must guarantee that the
/// complete physical memory is mapped to virtual memory at the passed
/// `physical_memory_offset`. Also, this function must be only called once
/// to avoid aliasing `&mut` references (which is undefined behavior).
#[must_use]
pub(super) unsafe fn init(physical_memory_offset: VirtAddr) -> OffsetPageTable<'static> {
    let level_4_table = active_level_4_table(physical_memory_offset);
    OffsetPageTable::new(level_4_table, physical_memory_offset)
}

/// Returns a mutable reference to the active level 4 table.
///
/// # Safety
/// This function is unsafe because the caller must guarantee that the
/// complete physical memory is mapped to virtual memory at the passed
/// `physical_memory_offset`. Also, this function must be only called once
/// to avoid aliasing `&mut` references (which is undefined behavior).
unsafe fn active_level_4_table(physical_memory_offset: VirtAddr) -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;

    let (level_4_table_frame, _) = Cr3::read();

    let physical_addr = level_4_table_frame.start_address();
    let virtual_addr = physical_memory_offset + physical_addr.as_u64();
    let page_table_ptr: *mut PageTable = virtual_addr.as_mut_ptr();

    &mut *page_table_ptr
}

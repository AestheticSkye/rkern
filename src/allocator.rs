//! Heap memory allocation.

pub mod fixed_size_block;

use bootloader::BootInfo;
use x86_64::structures::paging::mapper::MapToError;
use x86_64::structures::paging::{FrameAllocator, Mapper, Page, PageTableFlags, Size4KiB};
use x86_64::VirtAddr;

use self::fixed_size_block::FixedSizeBlockAllocator;
use crate::memory;
use crate::memory::frame_allocator::BootInfoFrameAllocator;

#[global_allocator]
static ALLOCATOR: Locked<FixedSizeBlockAllocator> = Locked::new(FixedSizeBlockAllocator::new());

/// The starting virtual address for the heap.
pub const HEAP_START: usize = 0x_4444_4444_0000;
/// Total size for the heap.
pub const HEAP_SIZE: usize = 100 * 1024; // 100 KiB

/// A wrapper of a mutex that we can implement the `GlobalAlloc` trait to.
pub struct Locked<A>(spin::Mutex<A>);

impl<A> Locked<A> {
    /// Generates new lockable structure with inner data.
    pub const fn new(inner: A) -> Self { Self(spin::Mutex::new(inner)) }

    /// Locks structure and allows mutable access to inner data.
    pub fn lock(&self) -> spin::MutexGuard<A> { self.0.lock() }
}

/// Initializes the systems allocator.
///
/// # Panics
///
/// Panics if:
///
/// A. The frame allocator runs our of frames.
///
/// B. The mapper fails.
pub(super) fn init_allocator(boot_info: &'static BootInfo) {
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");
}

/// Initializes the kernels heap.
///
/// # Errors
///
/// This function will return an error if:
///
/// A. The frame allocator runs our of frames.
///
/// B. The mapper fails.
fn init_heap(
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) -> Result<(), MapToError<Size4KiB>> {
    let page_range = {
        let heap_start = VirtAddr::new(HEAP_START as u64);
        let heap_end = heap_start + HEAP_SIZE - 1u64; // Inclusive range
        let heap_start_page = Page::containing_address(heap_start);
        let heap_end_page = Page::containing_address(heap_end);
        Page::range_inclusive(heap_start_page, heap_end_page)
    };

    for page in page_range {
        let frame = frame_allocator
            .allocate_frame()
            .ok_or(MapToError::FrameAllocationFailed)?;
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
        unsafe { mapper.map_to(page, frame, flags, frame_allocator)?.flush() };
    }

    unsafe {
        ALLOCATOR.lock().init(HEAP_START as *mut u8, HEAP_SIZE);
    }

    Ok(())
}

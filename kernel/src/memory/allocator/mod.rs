//pub mod bump;
//pub mod linked_list;
pub mod fixed_size_block;
use alloc::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;
use x86_64::{
    structures::paging::{
        mapper::MapToError, FrameAllocator, Mapper, OffsetPageTable, Page, PageTableFlags, Size4KiB,
    },
    VirtAddr,
};

use self::fixed_size_block::FixedSizeBlockAllocator;

use super::handler::MemoryHandler;

#[global_allocator]
static ALLOCATOR: Locked<FixedSizeBlockAllocator> = Locked::new(FixedSizeBlockAllocator::new());
pub const HEAP_START: usize = 0x100000;
pub const HEAP_SIZE: usize = 100 * 1024 * 5;

pub struct Dummy;

unsafe impl GlobalAlloc for Dummy {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        log::error!("ALLOCATION");
        null_mut()
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        log::error!("DEALLOCATION");
        panic!("dealloc should be never called")
    }
}

pub fn init_heap(mem_handler: &mut MemoryHandler) -> Result<(), MapToError<Size4KiB>> {
    let page_range = {
        let heap_start = VirtAddr::new(HEAP_START as u64);
        let heap_end = heap_start + HEAP_SIZE - 1u64;
        let heap_start_page = Page::containing_address(heap_start);
        let heap_end_page = Page::containing_address(heap_end);
        Page::range_inclusive(heap_start_page, heap_end_page)
    };
    for page in page_range {
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
        if let Err(e) = unsafe { mem_handler.map(page, flags) } {
            log::info!("map error");
        }
    }
    log::info!("alloc");

    unsafe {
        ALLOCATOR.lock().init(HEAP_START, HEAP_SIZE);
    }

    Ok(())
}

/// A wrapper around spin::Mutex to permit trait implementations.
pub struct Locked<A> {
    inner: spin::Mutex<A>,
}

impl<A> Locked<A> {
    pub const fn new(inner: A) -> Self {
        Locked {
            inner: spin::Mutex::new(inner),
        }
    }

    pub fn lock(&self) -> spin::MutexGuard<A> {
        self.inner.lock()
    }
}

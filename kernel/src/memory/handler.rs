use limine::response::MemoryMapResponse;
use x86_64::{
    structures::paging::{
        mapper::{MapToError, MapperFlush, UnmapError}, FrameAllocator, Mapper, OffsetPageTable, Page, PageTable, PageTableFlags, PhysFrame, Size4KiB
    },
    PhysAddr, VirtAddr,
};

use log::trace;

use crate::boot_info;

use super::frame_allocator::BootInfoFrameAllocator;

/// Returns a mutable reference to the active level 4 table.
///
/// This function is unsafe because the caller must guarantee that the
/// complete physical memory is mapped to virtual memory at the passed
/// `physical_memory_offset`. Also, this function must be only called once
/// to avoid aliasing `&mut` references (which is undefined behavior).
unsafe fn active_level_4_table(physical_memory_offset: VirtAddr) -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;

    let (level_4_table_frame, _) = Cr3::read();

    let phys = level_4_table_frame.start_address();
    let virt = physical_memory_offset + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    unsafe { &mut *page_table_ptr }
}

pub fn init() {
    let mem_handler = MemoryHandler::new(
        crate::boot_info!().phys_offset as u64,
        &boot_info!().memory_map,
    );
    unsafe { crate::boot_info::MEM_HANDLER.replace(mem_handler) };
}

#[derive(Debug)]
pub struct MemoryHandler {
    pub mapper: OffsetPageTable<'static>,
    pub frame_allocator: BootInfoFrameAllocator,
}
impl MemoryHandler {
    /// Inits heap & frame allocator
    pub fn new(physical_memory_offset: u64, memory_map: &'static MemoryMapResponse) -> Self {
        let physical_memory_offset = VirtAddr::new(physical_memory_offset);
        // trace!("Getting active level 4 table");
        let level_4_table = unsafe { active_level_4_table(physical_memory_offset) };
        // for table in level_4_table.iter() {
        //     log::debug!("{:?}", (table.addr(), table.flags()))
        // }
        todo!();
        log::info!("off");
        let mapper = unsafe { OffsetPageTable::new(level_4_table, physical_memory_offset) };
        log::info!("aa");
        let frame_allocator = unsafe { BootInfoFrameAllocator::init() };
        let mut _self = Self {
            mapper,
            frame_allocator,
        };
        log::info!("heap");
        crate::memory::allocator::init_heap(&mut _self).expect("heap initialization failed"); // Initialize the heap allocator
        log::info!("fu");
        _self
    }
    /// # Safety
    /// Mapping can cause all sorts of panics, set OffsetPageTable
    pub unsafe fn map(
        &mut self,
        page: Page<Size4KiB>,
        flags: PageTableFlags,
    ) -> Result<PhysAddr, MapToError<Size4KiB>> {
        let frame = self.frame_allocator.allocate_frame();
        if frame.is_none() {
            return Err(MapToError::FrameAllocationFailed);
        }
        let frame = frame.unwrap();
        unsafe { self.map_frame(page, frame, flags)? }
        Ok(frame.start_address())
    }
    /// # Safety
    /// Mapping can cause all sorts of panics, set OffsetPageTable
    pub unsafe fn unmap(
        &mut self,
        page: Page<Size4KiB>,
    ) -> Result<(PhysFrame, MapperFlush<Size4KiB>), UnmapError> {
        unsafe { self.mapper.unmap(page) }
    }

    /// # Safety
    /// Mapping can cause all sorts of panics, set OffsetPageTable
    pub unsafe fn map_frame(
        &mut self,
        page: Page<Size4KiB>,
        frame: PhysFrame,
        flags: PageTableFlags,
    ) -> Result<(), MapToError<Size4KiB>> {
        unsafe {
            self.mapper
                .map_to(page, frame, flags, &mut self.frame_allocator)?
                .flush()
        }
        Ok(())
    }
    pub fn malloc(&mut self, flags: PageTableFlags) -> Option<VirtAddr> {
        let frame = self.frame_allocator.allocate_frame()?;
        let virt_addr = VirtAddr::new(frame.start_address().as_u64());
        let page = Page::from_start_address(virt_addr).ok()?;
        unsafe { self.map_frame(page, frame, flags) }.ok()?;
        Some(virt_addr)
    }
}
///TODO Is it unsafe ?
#[track_caller]
pub fn map(page: Page<Size4KiB>, flags: PageTableFlags) -> PhysAddr {
    unsafe { crate::mem_handler!().map(page, flags) }.expect("Failed mapping mandatory page")
}
#[track_caller]
pub fn map_frame(page: Page<Size4KiB>, frame: PhysFrame, flags: PageTableFlags) {
    unsafe { crate::mem_handler!().map_frame(page, frame, flags) }
        .expect("Failed mapping mandatory page/frame")
}
#[macro_export]
macro_rules! mem_map {
    (frame_addr=$addr: expr, $($arg: tt)*) => {
        let page = x86_64::structures::paging::Page::containing_address(x86_64::VirtAddr::new($addr));
        let frame = x86_64::structures::paging::PhysFrame::containing_address(x86_64::PhysAddr::new($addr));
        $crate::mem_map!(page, frame=frame, $($arg)*);
    };
    ($page: expr, frame=$frame: expr, WRITABLE) => {
        let flags = x86_64::structures::paging::PageTableFlags::PRESENT | x86_64::structures::paging::PageTableFlags::WRITABLE;
        $crate::mem_map!($page,frame=$frame, flags);
    };
    ($page: expr, frame=$frame: expr, $flags: expr) => {
        if unsafe{$crate::mem_handler!().map_frame($page,$frame,$flags)}.is_err() {
            log::error!("Failed mapping {:?} -> {:?} with flags: {:#b}", $page, $frame, $flags);
        }
    };
    ($page: expr, $flags: expr) => {
        if unsafe{$crate::mem_handler!().map($page,$flags)}.is_err() {
            log::error!("Failed mapping {:?} with flags: {:#b}", $page, $flags);
        }
    };
}

#[macro_export]
macro_rules! malloc {
    ($flags: expr) => {
        $crate::mem_handler!().malloc($flags)
    };
}

#[derive(Debug)]
pub enum MapFrameError {
    CantAllocateFrame,
}

pub mod allocator;

static MEM_BASE: KernelAddressRequest = KernelAddressRequest::new();

/// Initialises Heap & Page mapper
pub fn init() {
    if let Some(mem_base) = MEM_BASE.get_response() {
        let phys_offset = mem_base.physical_base();
        let page_4 = unsafe{active_table(VirtAddr::new(phys_offset))};
        
        log::error!("Page 4");
        let addresses = [
            // the identity-mapped vga buffer page
            0xb8000,
            // some code page
            0x201008,
            // some stack page
            0x0100_0020_1a10,
            // virtual address mapped to physical address 0
            phys_offset,
        ];
        for &address in &addresses {
            let virt = VirtAddr::new(address);
            let phys = unsafe { translate_addr(virt, VirtAddr::new(phys_offset)) };
            // unsafe{crate::framebuffer::WRITER.as_mut().unwrap()}.write_fmt(format_args!("{:?} -> {:?}", virt, phys));
        }
    } else {
        log::error!("Failed getting memory base")
    }
}



use core::fmt::Write;

use limine::request::KernelAddressRequest;
use x86_64::{
    structures::paging::PageTable, PhysAddr, VirtAddr
};

/// Returns a mutable reference to the active level 4 table.
///
/// This function is unsafe because the caller must guarantee that the
/// complete physical memory is mapped to virtual memory at the passed
/// `physical_memory_offset`. Also, this function must be only called once
/// to avoid aliasing `&mut` references (which is undefined behavior).
pub unsafe fn active_table(physical_memory_offset: VirtAddr)
    -> &'static mut PageTable
{
    use x86_64::registers::control::Cr3;

    let (level_4_table_frame, _) = Cr3::read();

    let phys = level_4_table_frame.start_address();
    let virt = physical_memory_offset + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    &mut *page_table_ptr // unsafe
}


unsafe fn translate_addr(addr: VirtAddr, physical_memory_offset: VirtAddr) -> Option<PhysAddr> {
    use x86_64::structures::paging::page_table::FrameError;
    use x86_64::registers::control::Cr3;

    // read the active level 4 frame from the CR3 register
    let (level_4_table_frame, _) = Cr3::read();

    let table_indexes = [
        addr.p4_index(), addr.p3_index(), addr.p2_index(), addr.p1_index()
    ];
    let mut frame = level_4_table_frame;

    // traverse the multi-level page table
    for &index in &table_indexes {
        // convert the frame into a page table reference
        let virt = physical_memory_offset + frame.start_address().as_u64();
        let table_ptr: *const PageTable = virt.as_ptr();
        let table = unsafe {&*table_ptr};

        // read the page table entry and update `frame`
        let entry = &table[index];
        frame = match entry.frame() {
            Ok(frame) => frame,
            Err(FrameError::FrameNotPresent) => return None,
            Err(FrameError::HugeFrame) => panic!("huge pages not supported"),
    };
}

// calculate the physical address by adding the page offset
Some(frame.start_address() + u64::from(addr.page_offset()))
}
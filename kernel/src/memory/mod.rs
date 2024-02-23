use limine::{paging::Mode, request::PagingModeRequest};
use log::debug;

pub mod allocator;
pub mod frame_allocator;
pub mod handler;

static MEM_REQ: limine::request::PagingModeRequest = PagingModeRequest::new();

pub fn init() {
    debug!("a");
    let resp = MEM_REQ.get_response().unwrap();
    if resp.mode() == Mode::FIVE_LEVEL {panic!("Level 5 paging not supported")}
    for (name, flag) in resp.flags().iter_names() {
        debug!("a {} {:b}", name, flag.bits())
    }
    let (level_4_table_frame, _flags) = x86_64::registers::control::Cr3::read();
    let phys = level_4_table_frame.start_address();
    let virt = phys.as_u64();
    let page_table_ptr = virt as *mut x86_64::structures::paging::PageTable;

    let l4 = unsafe { &mut *page_table_ptr };
    for p in l4.iter() {
        debug!("{:?}", p);
    }
}

//TODO Make an allocator error handler
// #[alloc_error_handler]
// pub fn alloc_error(size: usize, align: usize) -> ! {

// }

use limine::{request::{KernelAddressRequest, MemoryMapRequest}, response::MemoryMapResponse};

use crate::memory::handler::MemoryHandler;

pub static mut BOOT_INFO: Option<BootInfo> = None;
pub struct BootInfo {
    pub phys_offset: u64,
    pub memory_map: &'static MemoryMapResponse,
}
#[macro_export]
macro_rules! boot_info {
    () => {
        unsafe {$crate::boot_info::BOOT_INFO.as_mut().unwrap()}
    };
}

pub fn init() {
    unsafe{BOOT_INFO.replace(BootInfo { 
        phys_offset: MEM_OFF_REQUEST.get_response().unwrap().physical_base(),
        memory_map: MEM_MAP_REQUEST.get_response().unwrap(), 
    })};
}

static MEM_OFF_REQUEST: KernelAddressRequest = KernelAddressRequest::new();
static MEM_MAP_REQUEST: MemoryMapRequest = MemoryMapRequest::new();



pub static mut MEM_HANDLER: Option<MemoryHandler> = None;
#[macro_export]
macro_rules! mem_handler {
    () => {
        unsafe{$crate::boot_info::MEM_HANDLER.as_mut().unwrap()}
    };
}
#![no_std]
#![no_main]
#![cfg_attr(debug_assertions, allow(dead_code, unused))]

use core::{arch::asm, fmt::Write};

use crate::framebuffer::{FrameBufferInfo, WRITER};

/// Sets the base revision to 1, this is recommended as this is the latest base revision described
/// by the Limine boot protocol specification. See specification for further info.
static BASE_REVISION: limine::BaseRevision = limine::BaseRevision::new();

mod framebuffer;
mod log;
#[no_mangle]
unsafe extern "C" fn _start() -> ! {
    assert!(BASE_REVISION.is_supported());
    log::init();

    hcf();
}

#[panic_handler]
fn rust_panic(_info: &core::panic::PanicInfo) -> ! {
    hcf();
}

fn hcf() -> ! {
    unsafe {
        asm!("cli");
        loop {
            asm!("hlt");
        }
    }
}
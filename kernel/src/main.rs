#![no_std]
#![no_main]
#![cfg_attr(debug_assertions, allow(dead_code, unused))]

use core::{arch::asm, fmt::Write};

use crate::framebuffer::{FrameBufferInfo, WRITER};

/// Sets the base revision to 1, this is recommended as this is the latest base revision described
/// by the Limine boot protocol specification. See specification for further info.
static BASE_REVISION: limine::BaseRevision = limine::BaseRevision::new();

extern crate alloc;

mod framebuffer;
mod logger;
mod acpi;
mod bit_manipulation;
mod memory;

#[no_mangle]
unsafe extern "C" fn _start() -> ! {
    assert!(BASE_REVISION.is_supported());
    logger::init();
    interrupts::init();
    memory::init();
    acpi::init();
    hcf();
}

#[panic_handler]
// #[track_caller]
fn rust_panic(info: &core::panic::PanicInfo) -> ! {
    unsafe{framebuffer::WRITER.as_mut().unwrap().write_str("PANIC")};
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
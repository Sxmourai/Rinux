#![no_std]
#![no_main]
#![cfg_attr(debug_assertions, allow(dead_code, unused))]

use core::{arch::asm, fmt::Write};

use crate::framebuffer::{FrameBufferInfo, WRITER};

static FRAMEBUFFER_REQUEST: limine::request::FramebufferRequest = limine::request::FramebufferRequest::new();
/// Sets the base revision to 1, this is recommended as this is the latest base revision described
/// by the Limine boot protocol specification. See specification for further info.
static BASE_REVISION: limine::BaseRevision = limine::BaseRevision::new();

mod framebuffer;
#[no_mangle]
unsafe extern "C" fn _start() -> ! {
    assert!(BASE_REVISION.is_supported());

    // Ensure we got a framebuffer.
    if let Some(mut framebuffer_response) = FRAMEBUFFER_REQUEST.get_response().and_then(|req|Some(req.framebuffers())) {
        // Get the first framebuffer's information.
        let framebuffer = &framebuffer_response.next().unwrap();
        let mut buffer = unsafe {core::slice::from_raw_parts_mut(framebuffer.addr(), framebuffer.height() as usize*framebuffer.pitch() as usize) as &'static mut [u8]};
        let info = FrameBufferInfo {
            width: framebuffer.width() as usize,
            height: framebuffer.height() as usize,
            stride: framebuffer.pitch() as usize,
            pixel_format: framebuffer::PixelFormat::Rgb,
            bytes_per_pixel: framebuffer.bpp().div_ceil(8) as usize,
        };
        let mut writer = framebuffer::FrameBufferWriter::new(buffer, info);
        WRITER.replace(writer);
        log::set_logger(WRITER.as_ref().unwrap()).unwrap();
        log::set_max_level(log::LevelFilter::Trace);
        log::info!("Initialised logger !");
    }

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
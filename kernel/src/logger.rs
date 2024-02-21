use crate::framebuffer::{self, FrameBufferInfo, FrameBufferWriter, WRITER};

static FRAMEBUFFER_REQUEST: limine::request::FramebufferRequest = limine::request::FramebufferRequest::new();
pub fn init() {
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
        unsafe{WRITER.replace(writer)};
        log::set_logger(unsafe{WRITER.as_ref().unwrap()}).unwrap();
        log::set_max_level(log::LevelFilter::Trace);
        log::info!("Initialised logger !");
    }
}

impl log::Log for FrameBufferWriter {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        todo!()
    }

    fn log(&self, record: &log::Record) {
        let prefix = match record.level() {
            log::Level::Error => "[ERROR]",
            log::Level::Warn =>  "[Warn]",
            log::Level::Info =>  "[Info]",
            log::Level::Debug => "[Debug]",
            log::Level::Trace => "[Trace]",
        };
        let mut_self = unsafe{very_bad_function(self)};
        for b in record.args().as_str().unwrap().chars() {
            mut_self.write_char(b as char);
        }
        mut_self.newline();
    }

    fn flush(&self) {
        todo!()
    }
}
/// https://stackoverflow.com/questions/54237610/is-there-a-way-to-make-an-immutable-reference-mutable
#[allow(invalid_reference_casting)]
unsafe fn very_bad_function<T>(reference: &T) -> &mut T {
    let const_ptr = reference as *const T;
    let mut_ptr = const_ptr as *mut T;
    &mut *mut_ptr
}
#![no_std]
#![no_main]
#![cfg_attr(debug_assertions, allow(dead_code, unused))]
#![feature(abi_x86_interrupt)] // For interrupts in src/interrupts/irq.rs i.e.
#![feature(const_mut_refs)] // See src/memory/allocated/fixed_size...
#![feature(panic_info_message)] // See down in panic handler

use core::{arch::asm, fmt::Write};

use crate::framebuffer::{FrameBufferInfo, WRITER};

/// Sets the base revision to 1, this is recommended as this is the latest base revision described
/// by the Limine boot protocol specification. See specification for further info.
static BASE_REVISION: limine::BaseRevision = limine::BaseRevision::new();

extern crate alloc;

mod acpi;
mod bit_manipulation;
mod boot_info;
mod framebuffer;
mod gdt;
mod interrupts;
mod logger;
mod memory;
mod ps2;
mod serial;
mod task;

#[no_mangle]
unsafe extern "C" fn _start() -> ! {
    assert!(BASE_REVISION.is_supported());
    crate::logger::init();
    log::info!("Boot info !");
    crate::boot_info::init();
    log::info!("GDT");
    crate::gdt::init();
    log::info!("Interrupts");
    crate::interrupts::init();
    log::info!("Memory");
    crate::memory::handler::init();
    log::info!("ACPI");
    crate::acpi::init();
    hcf();
}

#[panic_handler]
// #[track_caller]
fn rust_panic(panic_info: &core::panic::PanicInfo) -> ! {
    
    if let Some(info) = panic_info.payload().downcast_ref::<&str>() {
        writeln!(
            unsafe { framebuffer::WRITER.as_mut().unwrap() },
            "panic occurred: {:?}",
            (info, panic_info.location(), panic_info.message())
        );
    } else {
        writeln!(
            unsafe { framebuffer::WRITER.as_mut().unwrap() },
            "panic occurred {:?}", (panic_info.location(), panic_info.message())
        );
    }
    // write!(unsafe { framebuffer::WRITER.as_mut().unwrap() }, "PANIC {}", info.);
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

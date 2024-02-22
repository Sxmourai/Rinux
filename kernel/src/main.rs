#![no_std]
#![no_main]
#![cfg_attr(debug_assertions, allow(dead_code, unused))]
#![feature(abi_x86_interrupt)] // For interrupts in src/interrupts/irq.rs i.e.
#![feature(const_mut_refs)] // See src/memory/allocated/fixed_size...

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
mod interrupts;
mod gdt;
mod ps2;
mod task;
mod boot_info;
mod serial;

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
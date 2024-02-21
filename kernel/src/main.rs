#![no_std]
#![no_main]
#![cfg_attr(debug_assertions, allow(unused, dead_code))]

#[export_name = "_start"]
fn main() {
    loop {}
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    loop {}
}
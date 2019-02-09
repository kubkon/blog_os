#![no_std]
#![no_main]

#[macro_use]
mod vga_buffer;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    write!("Welcome to the BlogOS!");

    loop {}
}

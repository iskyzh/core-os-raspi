#![no_std]
#![no_main]
#![feature(asm)]
#![feature(global_asm)]

global_asm!(include_str!("start.S"));

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

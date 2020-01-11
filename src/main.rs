#![no_std]
#![no_main]
#![feature(asm)]
#![feature(format_args_nl)]
#![feature(global_asm)]
#![feature(panic_info_message)]

mod print;
mod console;

global_asm!(include_str!("start.S"));

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    if let Some(message) = _info.message() {
        println!("kernel panic: {}", message);
    } else {
        println!("kernel panic!");
    }
    loop {}
}

use core::ops::Range;

unsafe fn zero_volatile<T>(range: Range<*mut T>)
where
    T: From<u8>,
{
    let mut ptr = range.start;

    while ptr < range.end {
        core::ptr::write_volatile(ptr, T::from(0));
        ptr = ptr.offset(1);
    }
}

unsafe fn bss_range() -> Range<*mut usize> {
    extern "C" {
        static mut __bss_start: usize;
        static mut __bss_end: usize;
    }

    Range {
        start: &mut __bss_start,
        end: &mut __bss_end,
    }
}

#[inline(always)]
unsafe fn zero_bss() {
    zero_volatile(bss_range());
}

#[no_mangle]
unsafe extern "C" fn runtime_init() -> ! {
    zero_bss();

    kernel_init()
}

fn kernel_init() -> ! {
    println!("Hello, World!");
    panic!("Stop.")
}

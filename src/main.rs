#![no_std]
#![no_main]
#![feature(asm)]
#![feature(format_args_nl)]
#![feature(global_asm)]
#![feature(panic_info_message)]

mod console;
mod memory;
mod panic_handler;
mod print;

global_asm!(include_str!("start.S"));

#[no_mangle]
unsafe extern "C" fn runtime_init() -> ! {
    use memory::zero_bss;
    zero_bss();

    kernel_init()
}

fn kernel_init() -> ! {
    println!("Hello, World!");
    panic!("Stop.")
}

#![no_std]
#![no_main]
#![feature(asm)]
#![feature(format_args_nl)]
#![feature(global_asm)]
#![feature(panic_info_message)]

mod arch;
mod bsp;
mod memory;
mod panic_handler;
mod print;

pub unsafe fn runtime_init() -> ! {
    use memory::zero_bss;
    zero_bss();

    kernel_init()
}

fn kernel_init() -> ! {
    println!("Hello, World!");
    panic!("Stop.")
}

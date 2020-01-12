#![no_std]
#![no_main]
#![feature(asm)]
#![feature(format_args_nl)]
#![feature(global_asm)]
#![feature(panic_info_message)]
#![feature(trait_alias)]

#[macro_use]
extern crate lazy_static;

mod arch;
mod bsp;
mod memory;
mod panic_handler;
mod print;
mod interface;

use interface::console::Stat;

pub unsafe fn runtime_init() -> ! {
    use memory::zero_bss;
    zero_bss();

    kernel_init()
}

fn kernel_init() -> ! {
    use interface::console::*;
    for i in bsp::device_drivers().iter_mut() {
        if let Err(()) = i.init() {
            panic!("Error loading driver: {}", i.name())
        }
    }
    bsp::post_driver_init();
    bsp::console().write_char('a');
    kernel_main()
}

fn kernel_main() -> ! {
    use interface::console::*;
    loop {
        if bsp::console().read_char() == '\n' {
            break;
        }
    }
    
    println!("[1] Hello, World!");
    let chars_written = bsp::console().chars_written();
    println!("chars written: {}", chars_written);
    println!("[2] Echo...");
    loop {
        let c = bsp::console().read_char();
        bsp::console().write_char(c);
    }
}

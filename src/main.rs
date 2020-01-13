#![no_std]
#![no_main]
#![feature(asm)]
#![feature(format_args_nl)]
#![feature(global_asm)]
#![feature(panic_info_message)]
#![feature(trait_alias)]

mod arch;
mod bsp;
mod memory;
mod panic_handler;
mod print;
mod interface;
mod relocate;
mod runtime_init;

fn kernel_init() -> ! {
    println!("--- core-os kernel ---");
    println!("[1] loading drivers...");
    for i in bsp::device_drivers().iter_mut() {
        if let Err(()) = i.init() {
            panic!("error loading driver: {}", i.name())
        } else {
            println!("  {}", i.name());
        }
    }
    bsp::post_driver_init();
    
    kernel_main()
}

fn kernel_main() -> ! {
    use interface::console::*;
    println!("[1] Hello, World!");
    let chars_written = bsp::console().chars_written();
    println!("chars written: {}", chars_written);
    println!("[2] Echo...");
    loop {
        let c = bsp::console().read_char();
        bsp::console().write_char(c);
    }
}

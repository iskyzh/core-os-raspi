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
mod runtime_init;

fn kernel_init() -> ! {
    for driver in bsp::device_drivers().iter_mut() {
        if let Err(()) = driver.init() {
            panic!("error loading driver: {}", driver.name())
        }
    }
    bsp::post_driver_init();
    
    kernel_main()
}

fn kernel_main() -> ! {
    use interface::{console::*, time::*};
    use core::time::Duration;
    info!("kernel intialized");
    info!("driver loaded:");
    for (i, driver) in bsp::device_drivers().iter_mut().enumerate() {
        info!("  {} {}", i, driver.name());
    }
    loop {
        info!("spinning for 1secs");
        arch::timer().spin_for(Duration::from_secs(1));
    }
}

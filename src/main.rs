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
    use interface::time::*;
    use core::time::Duration;
    info!("kernel intialized");
    let (_, privilege_level) = arch::state::current_privilege_level();
    info!("current privilege level: {}", privilege_level);

    info!("exception handling state:");
    arch::state::print_exception_state();

    info!(
        "architectural timer resolution: {} ns",
        arch::timer().resolution().as_nanos()
    );
    info!("driver loaded:");
    for (i, driver) in bsp::device_drivers().iter_mut().enumerate() {
        info!("  {} {}", i, driver.name());
    }
    loop {
        info!("spinning for 1secs");
        arch::timer().spin_for(Duration::from_secs(1));
    }
}

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
mod relocate;

fn kernel_init() -> ! {
    for i in bsp::device_drivers().iter_mut() {
        if let Err(()) = i.init() {
            panic!("Error loading driver: {}", i.name())
        }
    }
    bsp::post_driver_init();
    
    kernel_main()
}

fn kernel_main() -> ! {
    use interface::console::*;

    println!("--- core-os bootloader ---");
    println!("waiting for binary...");

    bsp::console().flush();
    bsp::console().clear();

    for _ in 0..3 {
        bsp::console().write_char(3 as char);
    }
    
    // read binary size
    let mut size: u32 = u32::from(bsp::console().read_char() as u8);
    size |= u32::from(bsp::console().read_char() as u8) << 8;
    size |= u32::from(bsp::console().read_char() as u8) << 16;
    size |= u32::from(bsp::console().read_char() as u8) << 24;

    println!("binary size: {}", size);

    let kernel_addr: *mut u8 = bsp::BOARD_DEFAULT_LOAD_ADDRESS as *mut u8;
    unsafe {
        for i in 0..size {
            if i % 1024 == 0 {
                print!(".");
            }
            *kernel_addr.offset(i as isize) = bsp::console().read_char() as u8;
        }
    }
    println!("");
    println!("binary loaded, jumping to kernel...");

    bsp::console().flush();

    let kernel: extern "C" fn() -> ! = unsafe { core::mem::transmute(kernel_addr as *const ()) };
    
    kernel()
}

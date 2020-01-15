// SPDX-License-Identifier: MIT
//
// Copyright (c) 2020 Alex Chi <iskyzh@gmail.com>

// Rust embedded logo for `make doc`.
#![doc(html_logo_url = "https://git.io/JeGIp")]

//! The `kernel`
//!
//! The `kernel` is composed by glueing together code from
//!
//!   - [Hardware-specific Board Support Packages] (`BSPs`).
//!   - [Architecture-specific code].
//!   - HW- and architecture-agnostic `kernel` code.
//!
//! using the [`kernel::interface`] traits.
//!
//! [Hardware-specific Board Support Packages]: bsp/index.html
//! [Architecture-specific code]: arch/index.html
//! [`kernel::interface`]: interface/index.html

#![allow(incomplete_features)]
#![feature(const_generics)]
#![feature(format_args_nl)]
#![feature(global_asm)]
#![feature(panic_info_message)]
#![feature(trait_alias)]
#![no_main]
#![no_std]

// Conditionally includes the selected `architecture` code, which provides the `_start()` function,
// the first function to run.
mod arch;

// `_start()` then calls `runtime_init()`, which on completion, jumps to `kernel_init()`.
mod runtime_init;

// Conditionally includes the selected `BSP` code.
mod bsp;

mod interface;
mod memory;
mod panic_wait;
mod print;
mod test;

// Operating system utilities
mod process;
mod alloc;

// A simple user program
mod user;


/// Early init code.
///
/// Concerned with with initializing `BSP` and `arch` parts.
///
/// # Safety
///
/// - Only a single core must be active and running this function.
/// - The init calls in this function must appear in the correct order:
///     - Virtual memory must be activated before the device drivers.
///       - Without it, any atomic operations, e.g. the yet-to-be-introduced spinlocks in the device
///         drivers (which currently employ NullLocks instead of spinlocks), will fail to work on
///         the RPi SoCs.
unsafe fn kernel_init() -> ! {
    use interface::mm::MMU;

    arch::enable_exception_handling();

    if let Err(string) = arch::mmu().init() {
        panic!("MMU: {}", string);
    }

    for i in bsp::device_drivers().iter() {
        if let Err(()) = i.init() {
            panic!("Error loading driver: {}", i.compatible())
        }
    }
    bsp::post_driver_init();
    // println! is usable from here on.

    // Transition from unsafe to safe.
    kernel_main()
}

/// The main function running after the early init.
fn kernel_main() -> ! {
    use core::time::Duration;
    use interface::{console::All, time::Timer};

    info!("Booting on: {}", bsp::board_name());

    info!("MMU online. Special regions:");
    bsp::virt_mem_layout().print_layout();

    let (_, privilege_level) = arch::state::current_privilege_level();
    info!("Current privilege level: {}", privilege_level);

    info!("Exception handling state:");
    arch::state::print_exception_state();

    info!(
        "Architectural timer resolution: {} ns",
        arch::timer().resolution().as_nanos()
    );

    info!("Drivers loaded:");
    for (i, driver) in bsp::device_drivers().iter().enumerate() {
        info!("      {}. {}", i + 1, driver.compatible());
    }

    info!("Running tests...");
    test::test();
    info!("Running user program");
    unsafe { user::user_init(); }
    info!("Try R/W regions");
    let mut i = 0x00090000;
    loop {
        unsafe { *(i as *mut u64) = 0x0; }
        info!("0x{:X}", i);
        i += 0x10000;
    }
    info!("Echoing input now");
    loop {
        let c = bsp::console().read_char();
        bsp::console().write_char(c);
    }
}

// SPDX-License-Identifier: MIT
//
// Copyright (c) 2020 Alex Chi <iskyzh@gmail.com>

use crate::{
    arch, bsp, print, println, info,
    interface::{console::*, time::*},
};
use core::time::Duration;

fn test_timer() -> Result<(), &'static str> {
    let before = arch::timer().uptime();
    arch::timer().spin_for(Duration::from_secs(1));
    let after = arch::timer().uptime();
    if after.as_millis() - before.as_millis() >= 1 {
        Ok(())
    } else {
        Err("wrong duration")
    }
}

fn test_exception() -> Result<(), &'static str> {
    // Cause an exception by accessing a virtual address for which no translation was set up. This
    // code accesses the address 8 GiB, which is outside the mapped address space.
    //
    // For demo purposes, the exception handler will catch the faulting 8 GiB address and allow
    // execution to continue.
    let big_addr: u64 = 8 * 1024 * 1024 * 1024;
    unsafe { core::ptr::read_volatile(big_addr as *mut u64) };
    Ok(())
}

fn test_kalloc() -> Result<(), &'static str> {
    use crate::alloc::kalloc;
    for _ in 0..100 {
        let region = kalloc();
        for i in 0..4096 {
            unsafe {
                let target = region.offset(i);
                core::ptr::write_volatile(target, 233);
                if core::ptr::read_volatile(target) != 233 {
                    return Err("write failed");
                }
            }
        }
    }
    Ok(())
}

pub fn test() -> Result<(), ()> {
    let tests : [(&str, fn() -> Result<(), &'static str>); 1] = [("Alloc", test_kalloc)] /* [ ("Exception", test_exception), ("Timer", test_timer)] */;

    for (name, func) in tests.iter() {
        print!("Test: {} ...", name);
        match func() {
            Ok(_) => println!(" Ok"),
            Err(info) => println!(" Failed {}", info)
        }
    }
    /*
    // Now use address 9 GiB. The exception handler won't forgive us this time.
    info!("Trying to write to address 9 GiB...");
    big_addr = 9 * 1024 * 1024 * 1024;
    unsafe { core::ptr::read_volatile(big_addr as *mut u64) };

    // Will never reach here in this tutorial.
    */
    Ok(())
}

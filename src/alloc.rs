// SPDX-License-Identifier: MIT
//
// Copyright (c) 2020 Alex Chi <iskyzh@gmail.com>

use crate::bsp::addr_space_size;

static mut page_allocated : usize = addr_space_size();

pub fn kalloc() -> *mut u8 {
    // addr_space_size()
    unsafe { page_allocated -= 4096; }
    let result = unsafe { page_allocated as *mut u8 };
    result
}

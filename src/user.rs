// SPDX-License-Identifier: MIT
//
// Copyright (c) 2020 Alex Chi <iskyzh@gmail.com>

use crate::println;
use crate::arch::state;

pub unsafe fn user_init() {
    let (level, msg) = state::current_privilege_level();

    println!("User program at {}", msg);
}
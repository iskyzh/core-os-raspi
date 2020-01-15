// SPDX-License-Identifier: MIT
//
// Copyright (c) 2020 Alex Chi <iskyzh@gmail.com>

pub struct CPU {
    pub process: &'static Proc
}

#[derive(Copy, Clone)]
pub struct Proc {
    pub pid: i64
}

const NCPU : usize = 1;
const NPROC : usize = 256;

static init_proc: &'static Proc = &process[0];
static cpus: [CPU; NCPU] = [CPU { process: init_proc }; NCPU];
static process: [Proc; NPROC] = [Proc { pid: 0 }; NPROC];

/* struct cpu cpus[NCPU];
struct proc proc[NPROC]; */

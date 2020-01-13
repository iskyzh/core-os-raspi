use cortex_a::{asm, regs::*};
use crate::bsp;

mod sync;
pub use sync::*;
pub use sync::NullLock as Mutex;

mod time;
pub use time::*;

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    const CORE_MASK: u64 = 0x3;
    if bsp::BOOT_CORE_ID == MPIDR_EL1.get() & CORE_MASK {
        SP.set(bsp::BOOT_CORE_STACK_START);
        crate::runtime_init::runtime_init();
    } else {
        wait_forever()
    }
}

#[inline(always)]
pub fn wait_forever() -> ! {
    loop {
        asm::wfe()
    }
}

pub use asm::nop;

pub fn spin_for_cycles(n: usize) {
    for _ in 0..n {
        nop();
    }
}

static TIMER: Timer = Timer {};

pub fn timer() -> &'static impl crate::interface::time::Timer {
    &TIMER
}

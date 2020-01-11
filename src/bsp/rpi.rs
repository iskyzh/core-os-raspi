pub struct QEMUConsole;

pub const BOOT_CORE_ID: u64 = 0;
pub const BOOT_CORE_STACK_START: u64 = 0x80_000;

const QEMU_CONSOLE_ADDR: u64 = 0x3F20_1000;

use core::fmt;

impl fmt::Write for QEMUConsole {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            unsafe {
                core::ptr::write_volatile(QEMU_CONSOLE_ADDR as *mut u8, c as u8);
            }
        }
        Ok(())
    }
}

pub fn console() -> impl fmt::Write {
    QEMUConsole {}
}

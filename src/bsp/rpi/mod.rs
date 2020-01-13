mod memory_map;

use super::driver;
use crate::interface::{console, driver::Driver};

pub const BOOT_CORE_ID: u64 = 0;
pub const BOOT_CORE_STACK_START: u64 = 0x80_000;
// pub const BOARD_DEFAULT_LOAD_ADDRESS: usize = 0x80_000;

static mut GPIO: driver::GPIO = driver::GPIO::new(memory_map::mmio::GPIO_BASE);
static mut PL011_UART: driver::PL011Uart = unsafe { driver::PL011Uart::new(memory_map::mmio::PL011_UART_BASE) };

pub fn console() -> &'static mut impl console::Console {
    unsafe { &mut PL011_UART }
}

pub fn device_drivers() -> [&'static mut dyn Driver; 2] {
    unsafe { [&mut GPIO, &mut PL011_UART] }
}

/// BSP initialization code that runs after driver init.
pub fn post_driver_init() {
    // Configure PL011Uart's output pins.
    unsafe { GPIO.map_pl011_uart(); }
}

use core::panic::PanicInfo;

use crate::println;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    if let Some(message) = _info.message() {
        println!("kernel panic: {}", message);
    } else {
        println!("kernel panic!");
    }
    loop {}
}

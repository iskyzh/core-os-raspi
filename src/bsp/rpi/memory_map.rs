pub mod mmio {
    pub const BASE:            usize =        0x3F00_0000;

    pub const GPIO_BASE:       usize = BASE + 0x0020_0000;
    pub const PL011_UART_BASE: usize = BASE + 0x0020_1000;
}

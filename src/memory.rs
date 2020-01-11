
use core::ops::Range;

unsafe fn zero_volatile<T>(range: Range<*mut T>)
where
    T: From<u8>,
{
    let mut ptr = range.start;

    while ptr < range.end {
        core::ptr::write_volatile(ptr, T::from(0));
        ptr = ptr.offset(1);
    }
}

unsafe fn bss_range() -> Range<*mut usize> {
    extern "C" {
        static mut __bss_start: usize;
        static mut __bss_end: usize;
    }

    Range {
        start: &mut __bss_start,
        end: &mut __bss_end,
    }
}

#[inline(always)]
pub unsafe fn zero_bss() {
    zero_volatile(bss_range());
}

use core::ops::Range;

pub unsafe fn zero_volatile<T>(range: Range<*mut T>)
where
    T: From<u8>,
{
    core::ptr::write_bytes(range.start, 0, range.end as usize - range.start as usize);
}

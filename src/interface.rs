pub mod driver {
    pub type Result = core::result::Result<(), ()>;
    pub trait Driver {
        fn init(&mut self) -> Result;
        fn name(&self) -> &'static str;
    }
}

pub mod console {
    use core::fmt;
    pub trait Write {
        fn write_char(&mut self, c: char);
        fn write_fmt(&mut self, args: core::fmt::Arguments) -> fmt::Result;
        fn flush(&mut self);
    }

    pub trait Read {
        fn read_char(&mut self) -> char {
            ' '
        }
        fn clear(&mut self);
    }

    pub trait Stat {
        fn chars_written(&self) -> usize {
            0
        }
        fn chars_read(&self) -> usize {
            0
        }
    }

    pub trait Console = Write + Read + Stat;
}

use std::fmt;

pub struct Hex<T>(pub T);

impl<T> Hex<T> {
    pub fn new(item: T) -> Self {
        Hex(item)
    }
}

impl<T: fmt::Debug> fmt::Debug for Hex<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_fmt(format_args!("{:02X?}", self.0))
    }
}

use crate::io as std_io;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum Void {}

pub fn unsupported<T>() -> std_io::Result<T> {
    Err(unsupported_err())
}
    
pub fn unsupported_err() -> std_io::Error {
    std_io::Error::new_const(std_io::ErrorKind::Other, &"operation not supported on this platform")
}

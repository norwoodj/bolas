use std::error::Error;
use std::io;

pub(crate) fn bootstrap_to_io_error<E: Into<Box<dyn Error + Send + Sync>>>(err: E) -> io::Error {
    io::Error::new(io::ErrorKind::Other, err)
}

use std::error;
use std::fmt;
use std::io;

/// A list specifying the errors that can be encountered when constructing a frame header from a reader.
#[derive(Debug)]
pub enum Error {
    /// The size was 0
    InvalidSize,
    /// An error occurred whilst reading the bytes.
    Io(io::Error),
    /// An unknown flag was encountered.
    UnknownFlag,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::InvalidSize => write!(f, "Size must be greater than 0"),
            Error::Io(ref err) => write!(f, "IO error: {}", err),
            Error::UnknownFlag => write!(f, "Unknown flag found"),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::InvalidSize => "size must be greater than 0",
            Error::Io(ref err) => err.description(),
            Error::UnknownFlag => "unknown flag found",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::InvalidSize => None,
            Error::Io(ref err) => Some(err),
            Error::UnknownFlag => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

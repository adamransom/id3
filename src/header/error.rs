use std::error;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum Error {
    InvalidSize,
    Io(io::Error),
    NotID3,
    UnknownFlag,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::InvalidSize => write!(f, "Size must be greater than 0"),
            Error::Io(ref err) => write!(f, "IO error: {}", err),
            Error::NotID3 => write!(f, "Not an ID3 header"),
            Error::UnknownFlag => write!(f, "Unknown flag found"),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::InvalidSize => "size must be greater than 0",
            Error::Io(ref err) => err.description(),
            Error::NotID3 => "not an ID3 header",
            Error::UnknownFlag => "unknown flag found",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::InvalidSize => None,
            Error::Io(ref err) => Some(err),
            Error::NotID3 => None,
            Error::UnknownFlag => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

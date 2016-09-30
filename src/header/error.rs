use std::error;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum HeaderError {
    InvalidSize,
    Io(io::Error),
    NotID3,
    UnknownFlag,
}

impl fmt::Display for HeaderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            HeaderError::InvalidSize => write!(f, "Size must be greater than 0"),
            HeaderError::Io(ref err) => write!(f, "IO error: {}", err),
            HeaderError::NotID3 => write!(f, "Not an ID3 header"),
            HeaderError::UnknownFlag => write!(f, "Unknown flag found"),
        }
    }
}

impl error::Error for HeaderError {
    fn description(&self) -> &str {
        match *self {
            HeaderError::InvalidSize => "size must be greater than 0",
            HeaderError::Io(ref err) => err.description(),
            HeaderError::NotID3 => "not an ID3 header",
            HeaderError::UnknownFlag => "unknown flag found",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            HeaderError::InvalidSize => None,
            HeaderError::Io(ref err) => Some(err),
            HeaderError::NotID3 => None,
            HeaderError::UnknownFlag => None,
        }
    }
}

impl From<io::Error> for HeaderError {
    fn from(err: io::Error) -> HeaderError {
        HeaderError::Io(err)
    }
}

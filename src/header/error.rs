use std::error;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum HeaderError {
    Io(io::Error),
    NotID3,
    InvalidSize(u32),
}

impl fmt::Display for HeaderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            HeaderError::Io(ref err) => write!(f, "IO error: {}", err),
            HeaderError::InvalidSize(size) => write!(f, "Invalid size: {}", size),
            HeaderError::NotID3 => write!(f, "Not an ID3 header"),
        }
    }
}

impl error::Error for HeaderError {
    fn description(&self) -> &str {
        match *self {
            HeaderError::Io(ref err) => err.description(),
            HeaderError::InvalidSize(_) => "invalid size",
            HeaderError::NotID3 => "not an ID3 header",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            HeaderError::Io(ref err) => Some(err),
            HeaderError::InvalidSize(_) => None,
            HeaderError::NotID3 => None,
        }
    }
}

impl From<io::Error> for HeaderError {
    fn from(err: io::Error) -> HeaderError {
        HeaderError::Io(err)
    }
}

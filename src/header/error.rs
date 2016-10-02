use std::error;
use std::fmt;
use std::io;

/// A list specifying the errors that can be encountered when constructing a header from an array
/// of bytes.
#[derive(Debug)]
pub enum Error {
    /// The size was either 0 or greater than 268435455.
    InvalidSize,
    /// Either of the versions were 255.
    InvalidVersion,
    /// An error occurred whilst reading the bytes.
    Io(io::Error),
    /// The tag is not an ID3v2 tag.
    NotID3,
    /// An unknown header flag was encountered.
    UnknownFlag,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::InvalidSize => write!(f, "Size must be greater than 0 and less than 268435456"),
            Error::InvalidVersion => write!(f, "Major and revision versions must be less than 255"),
            Error::Io(ref err) => write!(f, "IO error: {}", err),
            Error::NotID3 => write!(f, "Not an ID3 header"),
            Error::UnknownFlag => write!(f, "Unknown flag found"),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::InvalidSize => "size must be greater than 0 and less than 268435456",
            Error::InvalidVersion => "major and revision versions must be less than 255",
            Error::Io(ref err) => err.description(),
            Error::NotID3 => "not an ID3 header",
            Error::UnknownFlag => "unknown flag found",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::InvalidSize => None,
            Error::InvalidVersion => None,
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

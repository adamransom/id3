use std::error;
use std::fmt;

use header;

/// A list specifying the errors that can be encountered when constructing a tag from a reader.
#[derive(Debug)]
pub enum Error {
    /// The tag header is invalid
    InvalidHeader(header::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::InvalidHeader(ref err) => write!(f, "Header error: {}", err),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::InvalidHeader(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::InvalidHeader(ref err) => Some(err),
        }
    }
}

impl From<header::Error> for Error {
    fn from(err: header::Error) -> Error {
        Error::InvalidHeader(err)
    }
}

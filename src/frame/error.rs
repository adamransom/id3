use std::error;
use std::fmt;

use frame::header;

/// A list specifying the errors that can be encountered when constructing a frame from a reader.
#[derive(Debug)]
pub enum Error {
    /// There was an error reading the header
    Header(header::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Header(ref err) => write!(f, "Header error: {}", err),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Header(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::Header(ref err) => Some(err),
        }
    }
}

impl From<header::Error> for Error {
    fn from(err: header::Error) -> Error {
        Error::Header(err)
    }
}

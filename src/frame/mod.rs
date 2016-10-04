//! Types, structs and functions related to reading a frame from an ID3v2 tag.

use std::io::Read;
use std::result;

use self::header::Header;

pub use self::error::Error;

mod error;
pub mod header;

/// A specialised `Result` type for frame reading operations.
pub type Result<T> = result::Result<T, Error>;

/// A type representing a frame in an ID3v2 tag.
///
/// # Reference
///
/// [ID3v2.3 Informal Standard (Section 3.3)](http://id3.org/id3v2.3.0#ID3v2_frame_overview)
#[derive(Debug, Default)]
pub struct Frame {
    header: Header,
}

impl Frame {
    /// Construct a new frame from a reader.
    ///
    /// # Errors
    ///
    /// If there is an error reading the frame header, then this function will return
    /// `Error::Header`.
    pub fn from_reader<R: Read>(reader: &mut R, version: u8) -> Result<Frame> {
        let header = try!(Header::from_reader(&mut reader.take(10), version));

        let frame = Frame { header: header };

        Ok(frame)
    }
}
